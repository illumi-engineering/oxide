use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use structopt::StructOpt;
use oxide_pkg::daemon::config::OxideDaemonConfig;
use oxide_pkg::daemon::project_manager::ProjectManager;
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

#[derive(Clone)]
struct Ctx {
    config: OxideDaemonConfig,
    project_manager:ProjectManager,
}

fn main() {
    let args = Args::from_args_safe().expect("[oxided] err: no config file specified");
    let listener = TcpListener::bind(OXIDE_LOCAL_COMMUNICATION_ADDRESS).unwrap();
    println!("[oxided] oxide daemon running at {}", OXIDE_LOCAL_COMMUNICATION_ADDRESS);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            std::thread::spawn({let ctx = Ctx {
                config: OxideDaemonConfig::load(args.config_file.clone()),
                project_manager: ProjectManager::new(),
            }; move || {
                handle_connection(stream, ctx)
                    .map_err(|e| eprintln!("[oxided] err: {}", e))
            }});
        }
    }
}

fn handle_connection(stream: TcpStream, mut ctx: Ctx) -> std::io::Result<()> {
    let mut protocol = Protocol::with_stream(stream)?;

    let request = protocol.read_message::<LocalRequest>()?;

    let resp = match request {
        LocalRequest::SyncProject { root_dir } => {
            println!("[oxided] sync requested for {}", root_dir);
            let updated = ctx.project_manager.sync(PathBuf::from(root_dir.clone()));

            if updated {
                println!("[oxided] project sync for {} successfully finished with changes", root_dir.clone())
            } else {
                println!("[oxided] project sync for {} successfully finished with no effective changes", root_dir.clone())
            }

            LocalResponse::SyncProject { ok: true, changed: updated }
        },
    };

    protocol.send_message(&resp)
}
