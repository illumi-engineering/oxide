#![allow(unused)]

use std::fs;
use std::path::PathBuf;
use std::process::exit;
use clap::Parser;
use oxide_common::project::OxideProject;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct BaseCli {
    subcommand: String
}

fn find_root_project_dir(current_dir: PathBuf) -> PathBuf {
    let parent = current_dir.parent();
    return match parent {
        None => {
            current_dir
        },
        Some(it) => {
            let parent_dir = it.to_path_buf();
            if is_project(parent_dir.clone()) {
                find_root_project_dir(parent_dir.clone())
            } else {
                current_dir
            }
        }
    }
}

fn is_project(current_directory: PathBuf) -> bool {
    return current_directory.join("oxide.toml").exists();
}

fn print_project(project: OxideProject, depth: usize) {
    if depth == 0 {
        println!("{} [root]", project.name);
    } else {
        println!("{}{}", "| ".repeat(depth), project.name);
    }

    match project.subprojects {
        None => {} // do nothing
        Some(subprojects) => {
            for subproject in subprojects {
                print_project(subproject, depth + 1);
            }
        }
    }
}

fn main() {
    let args = BaseCli::parse();

    match args.subcommand.as_str() {
        "init" => {

        }
        "show" => {
            let cwd = fs::canonicalize(PathBuf::from(".")).unwrap();

            if !is_project(cwd.clone()) {
                eprintln!("Oxide: {} is not a valid oxide project.", cwd.to_str().unwrap());
                exit(1);
            }

            let project = OxideProject::load(find_root_project_dir(cwd.clone()));
            println!("Project Structure for {}", project.name);
            print_project(project, 0);
        }
        _ => {
            eprintln!("Oxide: unknown command '{}'", args.subcommand)
        }
    }
}
