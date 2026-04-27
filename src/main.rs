use std::{env, error::Error, process::Command, thread::sleep, time::Duration};

use clap::Parser;
use commands::{Cli, Commands};
use mouse::{MouseButton, MouseDevice};
use profiles::{Config, Mode};

mod commands;
mod mouse;
mod process;
mod profiles;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Cli::parse();

    match arguments.command {
        Commands::Init => Config::init(),

        Commands::Config => {
            let editor = env::var("EDITOR").unwrap_or("nvim".to_string());
            // nvim is superior :3
            let config_path = Config::path()?.join("config.toml");

            Command::new(editor).arg(config_path).spawn()?.wait()?;

            Ok(())
        }

        Commands::List => {
            let config = Config::load()?;

            println!("Profiles list");

            for profile in config.profile.iter() {
                println!();
                println!("{}", profile.name);
                println!(
                    "button: {}",
                    if profile.button == MouseButton::Left {
                        "Left"
                    } else {
                        "Right"
                    }
                );
                match profile.mode {
                    Mode::Click => {
                        println!("mode: Click");
                        println!("interval: {}", profile.interval.unwrap());
                        println!("repeat: {}", profile.repeat.unwrap());
                    }
                    Mode::Hold => println!("mode: Hold"),
                }
            }

            Ok(())
        }

        Commands::Start { name, __run } => {
            let config = Config::load()?;

            if let Some(profile) = config.profile.iter().find(|e| e.name == name).cloned() {
                if __run {
                    println!("Running profile: {}", profile.name);

                    let mut mouse = MouseDevice::new()?;

                    match profile.mode {
                        Mode::Click => loop {
                            for _ in 0..=profile.repeat.unwrap() {
                                let _ = mouse.click(&profile.button);
                            }

                            sleep(Duration::from_millis(profile.interval.unwrap()));
                        },
                        Mode::Hold => {
                            ctrlc::set_handler(move || {
                                match MouseDevice::new() {
                                    Ok(mut mouse) => {
                                        let _ = mouse.release(&profile.button);
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to create MouseDevice: {}", e);
                                    }
                                }
                                std::process::exit(0);
                            })?;

                            let _ = mouse.hold(&profile.button);

                            loop {
                                sleep(Duration::from_secs(1)); // keep process alive
                            }
                        }
                    }
                } else {
                    process::spawn(&profile.name)?;
                }
            } else {
                eprintln!("profile not found: {}", name);
            }

            Ok(())
        }

        Commands::Stop { name } => {
            process::stop(&name)?;
            Ok(())
        }
    }
}
