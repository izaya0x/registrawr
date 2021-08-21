use clap::{App, Arg, SubCommand};
use registrawr_core::{get_dapp, list_dapps, register_dapp};
use std::{error, path::Path};
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn error::Error>> {
    let rt = Runtime::new()?;
    let matches = App::new("registrawr")
        .version("0.1")
        .author("Izaya0x <izaya0x@protonmail.com>")
        .about("Distributed tool for downloading Dapp frontends")
        .subcommand(SubCommand::with_name("list").about("lists all registered dapp frontends"))
        .subcommand(
            SubCommand::with_name("install")
                .about("install dapp from registry")
                .arg(
                    Arg::with_name("DAPP_NAME")
                        .help("Name of dapp to install")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("publish")
                .about("publish a dapp frontnend")
                .arg(
                    Arg::with_name("dapp_name")
                        .short("n")
                        .long("name")
                        .value_name("DAPP_NAME")
                        .help("Name of dapp to publish")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("FILE_PATH")
                        .help("Location of the source to publish")
                        .index(1)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("list") {
        rt.block_on(async {
            println!("Getting registerd dapps...");

            let dapps = list_dapps().await.unwrap();
            for dapp in dapps {
                println!("{}", dapp);
            }
        });
    }

    if let Some(matches) = matches.subcommand_matches("install") {
        match matches.value_of("DAPP_NAME") {
            Some(dapp_name) => {
                rt.block_on(async {
                    println!("Installing {}", dapp_name);
                    let dapp_data = get_dapp(dapp_name).await.unwrap();
                    println!("{}", dapp_data);
                });
            }
            None => println!("Error: No dapp given to install"),
        }
    }

    if let Some(matches) = matches.subcommand_matches("publish") {
        match matches.value_of("dapp_name") {
            Some(dapp_name) => {
                rt.block_on(async {
                    println!("Publishing {} frontend...", dapp_name);

                    let artifact_file_path = matches.value_of("FILE_PATH").unwrap();
                    register_dapp(dapp_name, &Path::new(artifact_file_path))
                        .await
                        .unwrap();

                    println!("Published!");
                });
            }
            None => println!("Error: No dapp name provided for publishing!"),
        }
    }

    Ok(())
}
