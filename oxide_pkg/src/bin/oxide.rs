#![allow(unused)]

use std::fs;
use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::exit;
use clap::Parser;
use structopt::StructOpt;
use oxide_pkg::ipc::protocol::Protocol;
use oxide_pkg::ipc::request::LocalRequest;
use oxide_pkg::ipc::response::LocalResponse;
use oxide_pkg::ipc::OXIDE_LOCAL_COMMUNICATION_ADDRESS;
use oxide_pkg::project::config::ProjectConfig;
use oxide_pkg::project::{find_root_project, is_project, OxideProject, print_project};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
#[structopt(name = "oxide")]
enum BaseCli {
    Init {
        #[structopt(long)]
        npm: bool,
    },
    Show,
}

fn main() {
    let args = BaseCli::from_args();

    match args {
        BaseCli::Init { npm } => {
            println!("[oxide] initializing project in current directory");
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("[oxide] err: no project name specified");

            let config = ProjectConfig {
                name,
                subprojects: None,
                npm: None,
            };

            let root_dir = fs::canonicalize(PathBuf::from("../../..")).unwrap();

            let config_path = root_dir.join("../../../oxide.toml");
            println!("[oxide] writing config to {}", config_path.to_str().unwrap());
            config.write(config_path).expect("[oxide] err: failed to write config");

            let sync = LocalRequest::SyncProject { root_dir: root_dir.as_path().to_str().unwrap().to_string() };
            Protocol::connect(SocketAddr::from(OXIDE_LOCAL_COMMUNICATION_ADDRESS))
                .and_then(|mut client| {
                    client.send_message(&sync);
                    Ok(client)
                })
                .and_then(|mut client| client.read_message::<LocalResponse>())
                .map(|resp| {
                    match resp {
                        LocalResponse::SyncProject{ ok, message } => {
                            if ok { println!("[oxide] daemon successfully notified.") }
                            else {
                                println!("[oxide] err: there was a problem notifying the daemon, server returned '{}'", message)
                            }
                        }
                        _ => {} // do nothing for other responses
                    }
                });
        }
        BaseCli::Show => {
            let cwd = fs::canonicalize(PathBuf::from("../../..")).unwrap();

            if !is_project(cwd.clone()) {
                eprintln!("[oxide] err: {} is not a valid oxide project.", cwd.to_str().unwrap());
                exit(1);
            }

            let project = OxideProject::load(find_root_project(cwd.clone()));
            println!("[oxide] project structure for {}", project.name);
            print_project(project, 0);
        }
        _ => {
            eprintln!("[oxide] err: unknown command")
        }
    }
}
