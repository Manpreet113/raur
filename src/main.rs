mod args;
mod aur;
mod core;
mod pacman;
mod git_ops;
mod parser;

use anyhow::Result;
use args::{Cli, Commands};
use clap::Parser;
use colored::*;
use std::{ env, path::Path };
use crate::git_ops::clone_repo;
use crate::parser::parse_srcinfo;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Search { query } => {
            let local_pkgs = pacman::search(query)?;

            let aur_pkgs = aur::search(query).await?;

            // TODO: Refactor to alpm

            if !local_pkgs.is_empty() {
                println!("{}", ":: repo / local".blue().bold());
                for pkg in &local_pkgs {
                    let installed = if pkg.installed {
                        " [installed]".cyan()
                    } else {
                        "".clear()
                    };
                    println!(
                        "{}/{} {}{}",
                        pkg.repo.magenta(),
                        pkg.name.bold(),
                        pkg.version.green(),
                        installed
                    );
                    println!("    {}", pkg.description);
                }
            }
            if !aur_pkgs.is_empty() {
                println!("{}", ":: aur / remote".blue().bold());
                for pkg in &aur_pkgs {
                    println!("aur/{} {} ({})",
                             pkg.name.bold(),
                             pkg.version.green(),
                             format!("+{}", pkg.num_votes).yellow()
                    );

                    if let Some(desc) = &pkg.description {
                        println!("    {}", desc);
                    }
                }
            }

            if local_pkgs.is_empty() && aur_pkgs.is_empty() {
                println!("No packages found for '{}'", query);
            }
        }
        Commands::Get { package} => {
            let home = env::var("HOME").expect("Could not find HOME");
            let cache_dir = format!("{}/.cache/raur/{}", home, package);
            let aur_url = format!("https://aur.archlinux.org/{}.git", package);
            let path = Path::new(&cache_dir);

            if path.exists() {
                eprintln!("!! Directory exists. (Pull logic goes here later)");
                return Ok(());
            }

            // CALL THE NEW FUNCTION
            match clone_repo(&aur_url, path) {
                Ok(_) => {
                    println!(":: Package ready in {}", cache_dir);
                    match parse_srcinfo(path){
                        Ok(meta) => {
                            println!(":: Package: {} v{}", meta.pkgbase, meta.version);
                            println!(":: Dependencies: {:?}", meta.depends);
                            println!(":: Build Deps: {:?}", meta.make_depends);
                        }
                        Err(e) => eprintln!("!! Failed to read .SRCINFO: {}", e),
                    }
                },
                Err(e) => eprintln!("!! Failed to clone: {}", e),
            }
        }
    }
    Ok(())
}
