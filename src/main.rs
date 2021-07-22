pub mod remotecli;

use structopt::StructOpt;
use users::{get_current_uid, get_user_by_uid};

// These are the options used by the `server` subcommand
#[derive(Debug, StructOpt)]
pub struct ServerOptions {
    /// The address of the server that will run commands.
    #[structopt(long, default_value = "127.0.0.1:50051")]
    pub server_listen_addr: String,
}

// These are the options used by the `run` subcommand
#[derive(Debug, StructOpt)]
pub struct RemoteCommandOptions {
    /// The full command and arguments for the server to execute
    pub command: Vec<String>,
}

// These are the only valid values for our subcommands
#[derive(Debug, StructOpt)]
pub enum SubCommand {
    /// Start the remote command gRPC server
    #[structopt(name = "server")]
    StartServer(ServerOptions),
    /// Schedule a remote command to the gRPC server
    #[structopt(setting = structopt::clap::AppSettings::TrailingVarArg)]
    Run(RemoteCommandOptions),
    /// Stop a remote command on the gRPC server
    #[structopt(setting = structopt::clap::AppSettings::TrailingVarArg)]
    Stop(RemoteCommandOptions),
    /// Send a remote command to the gRPC server and stream the output
    #[structopt(setting = structopt::clap::AppSettings::TrailingVarArg)]
    Streamer(RemoteCommandOptions),
    /// Send a remote command to the gRPC server and get the status
    #[structopt(setting = structopt::clap::AppSettings::TrailingVarArg)]
    Status(RemoteCommandOptions),
    /// Send a remote command to the gRPC server and get the result
    #[structopt(setting = structopt::clap::AppSettings::TrailingVarArg)]
    Result(RemoteCommandOptions),
}

// This is the main arguments structure that we'll parse from
#[derive(StructOpt, Debug)]
#[structopt(name = "remotecli")]
struct ApplicationArguments {
    #[structopt(flatten)]
    pub subcommand: SubCommand,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ApplicationArguments::from_args();

    match args.subcommand {
        SubCommand::StartServer(opts) => {
            println!("Start the server on: {:?}", opts.server_listen_addr);
            remotecli::server::start_server(opts).await?;
        }
        SubCommand::Run(mut rc_opts) => {
            println!("Run command: '{:?}'", rc_opts.command);
            rc_opts.command.push(String::from("run"));
            let user = get_user_by_uid(get_current_uid()).unwrap();
            rc_opts
                .command
                .push(String::from(user.name().to_str().unwrap()));
            remotecli::client::client_run(rc_opts).await?;
        }
        SubCommand::Stop(mut rc_opts) => {
            println!("Stop command: '{:?}'", rc_opts.command);
            rc_opts.command.push(String::from("stop"));
            let user = get_user_by_uid(get_current_uid()).unwrap();
            rc_opts
                .command
                .push(String::from(user.name().to_str().unwrap()));
            remotecli::client::client_run(rc_opts).await?;
        }
        SubCommand::Streamer(mut rc_opts) => {
            println!("Stream command: '{:?}'", rc_opts.command);
            rc_opts.command.push(String::from("streamer"));
            let user = get_user_by_uid(get_current_uid()).unwrap();
            rc_opts
                .command
                .push(String::from(user.name().to_str().unwrap()));
            remotecli::client::client_run(rc_opts).await?;
        }
        SubCommand::Status(mut rc_opts) => {
            println!("Status command: '{:?}'", rc_opts.command);
            rc_opts.command.push(String::from("status"));
            let user = get_user_by_uid(get_current_uid()).unwrap();
            rc_opts
                .command
                .push(String::from(user.name().to_str().unwrap()));
            remotecli::client::client_run(rc_opts).await?;
        }
        SubCommand::Result(mut rc_opts) => {
            println!("Output command: '{:?}'", rc_opts.command);
            rc_opts.command.push(String::from("result"));
            let user = get_user_by_uid(get_current_uid()).unwrap();
            rc_opts
                .command
                .push(String::from(user.name().to_str().unwrap()));
            remotecli::client::client_run(rc_opts).await?;
        }
    }

    Ok(())
}
