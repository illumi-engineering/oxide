use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use structopt::StructOpt;
use oxide_pkg::daemon::config::OxideDaemonConfig;
use oxide_pkg::ipc::protocol::Protocol;
use oxide_pkg::ipc::request::LocalRequest;
use oxide_pkg::ipc::response::LocalResponse;
use oxide_pkg::ipc::OXIDE_LOCAL_COMMUNICATION_ADDRESS;

#[derive(StructOpt)]
#[structopt(name = "oxided")]
struct Args {
    #[structopt(short = "c", long = "config")]
    config_file: PathBuf,
}

fn main() {
    let args = Args::from_args_safe().expect("[oxided] err: no config file specified");
    let config = OxideDaemonConfig::load(args.config_file);

    let listener = TcpListener::bind(OXIDE_LOCAL_COMMUNICATION_ADDRESS).unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            std::thread::spawn(move || {
                handle_connection(stream).map_err(|e| eprintln!("[oxided] err: {}", e))
            });
        }
    }
}

fn handle_connection(stream: TcpStream) -> std::io::Result<()> {
    let mut protocol = Protocol::with_stream(stream)?;

    let request = protocol.read_message::<LocalRequest>()?;

    let resp = match request {
        LocalRequest::SyncProject { root_dir } => {
            LocalResponse::SyncProject { ok: true, message: String::new() }
        },
    };

    protocol.send_message(&resp)
}
