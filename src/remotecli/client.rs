pub mod remotecli_proto {
    tonic::include_proto!("remotecli");
}

// Proto generated client
use remotecli_proto::remote_cli_client::RemoteCliClient;

// Proto message structs
use remotecli_proto::CommandInput;

use crate::remotecli::auth::get_token;
use crate::RemoteCommandOptions;
use tonic::Request;

pub async fn client_run(
    mut rc_opts: RemoteCommandOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    // getting certificate from disk
    let cert = include_str!("../../certs/client.pem");
    let key = include_str!("../../certs/client.key");
    // creating identify from key and certificate
    let id = tonic::transport::Identity::from_pem(cert.as_bytes(), key.as_bytes());
    // importing our certificate for CA
    let s = include_str!("../../certs/ca.pem");
    // converting it into a certificate
    let ca = tonic::transport::Certificate::from_pem(s.as_bytes());
    // telling our client what is the identity of our server
    let tls = tonic::transport::ClientTlsConfig::new()
        .domain_name("localhost")
        .identity(id)
        .ca_certificate(ca);
    // connecting with tls
    let channel = tonic::transport::Channel::from_static("http://127.0.0.1:50051")
        .tls_config(tls)
        .unwrap()
        .connect()
        .await?;
    let token = get_token(rc_opts.command.pop().unwrap());
    let mut client = RemoteCliClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert(
            "authorization",
            tonic::metadata::MetadataValue::from_str(&token).unwrap(),
        );
        Ok(req)
    });
    let request = tonic::Request::new(CommandInput {
        command: rc_opts.command[0].clone(),
    });
    println!("{:?}", &rc_opts.command);
    match &rc_opts.command.pop().unwrap()[..] {
        "run" => {
            let response = client.shell(request).await?;
            println!("RESPONSE={:?}", response);
        }
        "stop" => {
            let response = client.stop(request).await?;
            println!("RESPONSE={:?}", response);
        }
        "streamer" => {
            let mut response = client.streamer(request).await?.into_inner();
            while let Some(res) = response.message().await? {
                println!("NOTE = {:?}", res);
            }
        }
        "status" => {
            let response = client.status(request).await?;
            println!("RESPONSE={:?}", response);
        }
        "result" => {
            let response = client.result(request).await?;
            println!("RESPONSE={:?}", response);
        }
        _ => {
            println!("Method not found!");
        }
    }
    Ok(())
}
