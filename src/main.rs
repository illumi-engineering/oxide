#![allow(unused)]

use std::fs;
use std::io;
use std::net::{SocketAddr, SocketAddrV4};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use clap::Parser;
use oxide_ipc::{IpcRequest, IpcResponse, OXIDE_IPC_LOCAL_ADDRESS, Protocol};
use structopt::StructOpt;
use twelf::Layer;
use oxide_config::client::ClientConfig;
use oxide_project::config::{ProjectConfig};
use oxide_project::{find_root_project_dir, is_project, OxideProject, print_project};

#[derive(StructOpt)]
#[structopt(name = "oxided")]
struct BaseCli {
    #[structopt(long)]
    daemon_addr: SocketAddrV4,
    cmd: CliCommand,
}

#[derive(StructOpt)]
enum CliCommand {
    Init {
        #[structopt(long)]
        npm: bool,
    },
    Sync,
    Show,
}

fn sync_project(root_dir: PathBuf, daemon_addr: String) {
    let sync = IpcRequest::SyncProject { root_dir: root_dir.as_path().to_str().unwrap().to_string() };
    Protocol::connect(SocketAddr::from(SocketAddrV4::from_str(daemon_addr.as_str()).unwrap()))
        .and_then(|mut client| {
            client.send_message(&sync);
            Ok(client)
        })
        .and_then(|mut client| client.read_message::<IpcResponse>())
        .map(|resp| {
            match resp {
                IpcResponse::SyncProject { ok, changed } => {
                    if ok { println!("[oxide] daemon successfully notified") }
                    else {
                        println!("[oxide] err: there was a problem notifying the daemon")
                    }
                }
                _ => {} // do nothing for other responses
            }
        });
}

fn main() {
    // let app = clap::Command::new("oxided").args(&BaseCli::clap_args());
    let args = BaseCli::from_args();


    match args.cmd {
        CliCommand::Init { npm } => {
            println!("[oxide] initializing project in current directory");
            println!("Enter Project Name: ");
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("[oxide] err: no project name specified");

            let mut config = ProjectConfig::new(name.trim().to_string());

            let cwd = fs::canonicalize(PathBuf::from(".")).unwrap();
            let root_dir = find_root_project_dir(cwd);

            if npm {
                println!("[oxide] enabling npm support")
                // config.npm = Some(ProjectNpmConfig {
                //     package_file: "package.json".parse().unwrap(),
                // })
            }

            let config_path = root_dir.join("oxide.toml");
            println!("[oxide] writing config to {}", config_path.to_str().unwrap());
            config.write_to(config_path).expect("[oxide] err: failed to write config");

            sync_project(root_dir, args.daemon_addr.into());
        }
        CliCommand::Sync => {
            let cwd = fs::canonicalize(PathBuf::from(".")).unwrap();
            let root_dir = find_root_project_dir(cwd);
            sync_project(root_dir, args.daemon_addr.into());
        }
        CliCommand::Show => {
            let cwd = fs::canonicalize(PathBuf::from(".")).unwrap();

            if !is_project(cwd.clone()) {
                eprintln!("[oxide] err: {} is not a valid oxide project.", cwd.to_str().unwrap());
                exit(1);
            }

            let project = OxideProject::load(find_root_project_dir(cwd.clone()));
            println!("[oxide] project structure for {}", project.name);
            print_project(project, 0);
        }
        _ => {
            eprintln!("[oxide] err: unknown command")
        }
    }
}
