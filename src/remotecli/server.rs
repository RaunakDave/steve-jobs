use crate::remotecli::auth::interceptor;
use lazy_static::lazy_static;
use nanoid::nanoid;
use std::collections::HashMap;
use std::pin::Pin;
use std::process::Stdio;
use std::sync::Mutex;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio_stream::Stream;
use tonic::{transport::Server, Request, Response, Status};
use nix::sched;
use nix::sys::signal::Signal;
use nix::sys::signal::kill;
use nix::unistd;
use std::process::Command as cmd;
use cgroups_rs::*;
use cgroups_rs::devices::*;
use cgroups_rs::cgroup_builder::*;
use std::os::unix::process::CommandExt;
use std::fs;
use rm_rf::ensure_removed;
use rustls::{
    ciphersuite::TLS13_AES_256_GCM_SHA384, AllowAnyAuthenticatedClient, ServerConfig, RootCertStore, internal::pemfile
};

// Import the generated rust code into module
pub mod remotecli_proto {
    tonic::include_proto!("remotecli");
}

// Proto generated server traits
use remotecli_proto::remote_cli_server::{RemoteCli, RemoteCliServer};

// Proto message structs
use remotecli_proto::{CommandInput, CommandOutput};

// For the server listening address
use crate::ServerOptions;


static HOSTNAME: &str = "mrarsenal";

lazy_static! {
    static ref MAP: Mutex<HashMap<String, nix::unistd::Pid>> = Mutex::new(HashMap::new());
}

fn set_hostname(hostname: &str){
	// can also use libc here
	unistd::sethostname(hostname).unwrap()
}

fn create_isolated_namespace(){
	sched::unshare(sched::CloneFlags::CLONE_NEWNS).expect("Failed to unshare");
}

#[allow(unused_must_use)]
fn spawn_child(hostname: &str, job_id: String, command: &str, command_args: &[String]) -> isize {

	create_isolated_namespace();	
	set_hostname(hostname);

    let h = cgroups_rs::hierarchies::auto();
    let cgroup: Cgroup = CgroupBuilder::new("hello")
        .memory()
            .kernel_memory_limit(1024 * 1024)
            .memory_hard_limit(1024 * 1024)
            .done()
        .cpu()
            .shares(100)
            .done()
        .devices()
            .device(1000, 10, DeviceType::Block, true,
                vec![DevicePermissions::Read,
                    DevicePermissions::Write,
                    DevicePermissions::MkNod])
            .device(6, 1, DeviceType::Char, false, vec![])
            .done()
        .network()
            .class_id(1337)
            .priority("eth0".to_string(), 100)
            .priority("wl0".to_string(), 200)
            .done()
        .hugepages()
            .limit("2M".to_string(), 0)
            .limit("4M".to_string(), 4 * 1024 * 1024 * 100)
            .limit("2G".to_string(), 2 * 1024 * 1024 * 1024)
            .done()
        .blkio()
            .weight(123)
            .leaf_weight(99)
            .weight_device(6, 1, Some(100), Some(55))
            .weight_device(6, 1, Some(100), Some(55))
            .throttle_iops()
                .read(6, 1, 10)
                .write(11, 1, 100)
            .throttle_bps()
                .read(6, 1, 10)
                .write(11, 1, 100)
            .done()
            .build(h);
    
    let output: std::process::Output;
    unsafe {
        output = cmd::new(command).args(command_args)
            .pre_exec(move || {
                let pid = std::process::id();
                cgroup.add_task(CgroupPid::from(pid as u64));
            Ok(())
            })
            .output()
            .expect("Failed to execute command");
    };
    fs::write("/tmp/steve-jobs/".to_owned() + &job_id + "/" + &job_id + ".status", output.status.to_string());
    fs::write("/tmp/steve-jobs/".to_owned() + &job_id + "/" + &job_id + ".out", output.stdout);
    fs::write("/tmp/steve-jobs/".to_owned() + &job_id + "/" + &job_id + ".err", output.stderr);
	return 0;
}

#[derive(Default)]
pub struct Cli {}

#[tonic::async_trait]
impl RemoteCli for Cli {
    #[allow(unused_must_use)]
    async fn shell(
        &self,
        request: Request<CommandInput>,
    ) -> Result<Response<CommandOutput>, Status> {
        let req_command = request.into_inner();
        let command = req_command.command;
        let job_id = nanoid!();
        println!("Running command: {:?}", &command);
        
        let iter: Vec<String> = command.split(" ").map(String::from).collect();
        let hostname = HOSTNAME;
	    const STACK_SIZE: usize = 1024 * 1024;
	    let stack: &mut [u8; STACK_SIZE] = &mut [0; STACK_SIZE];
	    let cb = Box::new(|| spawn_child(hostname, job_id.clone(), &iter[0], &iter[1..]));
        let clone_flags = sched::CloneFlags::CLONE_NEWNS | sched::CloneFlags::CLONE_NEWPID | sched::CloneFlags::CLONE_NEWNET;
	    let child_pid = sched::clone(cb, stack, clone_flags, Some(Signal::SIGCHLD as i32)).expect("Failed to create child process");
        let mut hmap = MAP.lock().unwrap();
        hmap.insert(job_id.clone(), child_pid);
        fs::create_dir_all("/tmp/steve-jobs/".to_owned() + &job_id + "/");
        fs::write("/tmp/steve-jobs/".to_owned() + &job_id + "/" + &job_id + ".status", "Running!");
        Ok(Response::new(CommandOutput { output: job_id }))
    }
    #[allow(unused_must_use)]
    async fn stop(
        &self,
        request: Request<CommandInput>,
    ) -> Result<Response<CommandOutput>, Status> {
        let req_command = request.into_inner();
        let command = req_command.command;

        println!("Killing job: {:?}", &command);

        let mut hmap = MAP.lock().unwrap();
        let process = hmap.remove(&command).unwrap();
        kill(process, Signal::SIGKILL);
        fs::write("/tmp/steve-jobs/".to_owned() + &command + "/" + &command + ".status", "Killed!")?;
        Ok(Response::new(CommandOutput {
            output: String::from("Killed!"),
        }))
    }
    type StreamerStream =
        Pin<Box<dyn Stream<Item = Result<CommandOutput, Status>> + Send + Sync + 'static>>;
    async fn streamer(
        &self,
        request: Request<CommandInput>,
    ) -> Result<Response<Self::StreamerStream>, Status> {
        let req_command = request.into_inner();
        let command = req_command.command;
        println!("Streaming command: {:?}", &command);
        let iter: Vec<String> = command.split(" ").map(String::from).collect();
        let mut process = Command::new(&iter[0]);
        for arg in &iter[1..] {
            process.arg(arg);
        }
        let mut cmd = process
            .stdout(Stdio::piped())
            .spawn()
            .expect("Could not spawn process");
        let out = cmd.stdout.take().unwrap();
        let mut stdout = BufReader::new(out).lines();
        let output = async_stream::try_stream! {
            while let Some(line) = stdout.next_line().await.unwrap() {
                yield CommandOutput { output: line };
            }
        };
        Ok(Response::new(Box::pin(output) as Self::StreamerStream))
    }
    async fn status(
        &self,
        request: Request<CommandInput>,
    ) -> Result<Response<CommandOutput>, Status> {
        let req_command = request.into_inner();
        let command = req_command.command;

        println!("Status of job: {:?}", &command);
        let status = fs::read_to_string("/tmp/steve-jobs/".to_owned() + &command + "/" + &command + ".status").unwrap();
        Ok(Response::new(CommandOutput {
            output: status,
        }))
    }
    async fn result(
        &self,
        request: Request<CommandInput>,
    ) -> Result<Response<CommandOutput>, Status> {
        let req_command = request.into_inner();
        let command = req_command.command;

        println!("Result of job: {:?}", &command);

        let stdout = fs::read_to_string("/tmp/steve-jobs/".to_owned() + &command + "/" + &command + ".out").unwrap();
        let stderr = fs::read_to_string("/tmp/steve-jobs/".to_owned() + &command + "/" + &command + ".err").unwrap();
        let logs = String::from("Stdout\n") + &stdout + "\nStderr\n" + &stderr;
        Ok(Response::new(CommandOutput { output: logs }))
    }
}
#[allow(unused_must_use)]
pub async fn start_server(opts: ServerOptions) -> Result<(), Box<dyn std::error::Error>> {
    ensure_removed("/tmp/steve-jobs");
    let addr = opts.server_listen_addr.parse().unwrap();
    let cli_server = Cli::default();
    let server = RemoteCliServer::with_interceptor(cli_server, interceptor);
    let mut cert: &[u8] = include_bytes!("../../certs/server.pem");
    let cert_pem = pemfile::certs(&mut cert);
    let mut key: &[u8] = include_bytes!("../../certs/server.key");
    let private_key = &pemfile::rsa_private_keys(&mut key).unwrap()[0];
    // reading ca root from disk
    let mut s: &[u8] = include_bytes!("../../certs/ca.pem");
    let mut ca_cert = RootCertStore::empty();
    ca_cert.add_pem_file(&mut s);
    let auth = AllowAnyAuthenticatedClient::new(ca_cert);
    let mut config = ServerConfig::with_ciphersuites(auth, &[&TLS13_AES_256_GCM_SHA384]);
    config.set_single_cert(cert_pem.unwrap(), private_key.clone());
    config.set_protocols(&[Vec::from(&b"h2"[..])]);
    
    // creating tls config
    let mut tls = tonic::transport::ServerTlsConfig::new();
    tls.rustls_server_config(config);
    println!("RemoteCliServer listening on {}", addr);

    Server::builder()
        .tls_config(tls)
        .unwrap()
        .add_service(server)
        .serve(addr)
        .await?;

    Ok(())
}
