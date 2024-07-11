use clap::{command, Arg, ArgAction, Command, ValueHint};
use repositories::store::{self, AppStore, Store};

mod data;
mod repositories;

trait PlatformArguments {
    fn add_commands(self) -> Self;
}

impl PlatformArguments for Command {
    fn add_commands(self) -> Self {
        return self.subcommand(
            Command::new("version")
                .subcommand_required(true)
                .subcommand_precedence_over_arg(true)
                .subcommand(
                    Command::new("create")
                        .about("Creates a new version")
                        .subcommand_precedence_over_arg(true)
                        .arg(
                            Arg::new("name")
                                .help("The name of the version to be created")
                                .required(true),
                        ),
                )
                .subcommand(
                    Command::new("notes")
                        .about("Updates the release notes")
                        .subcommand_precedence_over_arg(true)
                        .arg(
                            Arg::new("message")
                                .help("The message")
                                .long("message")
                                .short('m')
                                .required(true),
                        )
                        .arg(
                            Arg::new("language")
                                .help("The language of the release notes")
                                .long("language")
                                .short('l')
                                .required(true),
                        )
                        .arg(
                            Arg::new("name")
                                .help("The version name to suffer the update")
                                .long("name")
                                .short('n')
                                .required(true),
                        ),
                ),
        );
    }
}

fn main() {
    let matches = command!()
        .propagate_version(true)
        .about("Freitool is a tool to help you manage your app releases.")
        .author("Anderson Freitas <freitas@disroot.org>")
        .subcommand_required(true)
        .subcommand_precedence_over_arg(true)
        .subcommand(
            Command::new("android")
                .add_commands()
                .arg(
                    Arg::new("package-name")
                        .help("The package name")
                        .global(true)
                        .long("package-name")
                        .value_name("com.example.app"),
                )
                .arg(
                    Arg::new("key-path")
                        .global(true)
                        .help("The key path, must be a .json file")
                        .value_name("FILE")
                        .long("key-path")
                        .value_hint(ValueHint::FilePath),
                )
                .arg(
                    Arg::new("track")
                        .global(true)
                        .help("The google play track")
                        .value_parser(["internal", "alpha", "beta", "production"])
                        .long("track"),
                ),
        )
        .subcommand(
            Command::new("ios")
                .add_commands()
                .arg(
                    Arg::new("app-id")
                        .help("The App Store Connect app ID")
                        .global(true)
                        .long("app-id"),
                )
                .arg(
                    Arg::new("key-path")
                        .global(true)
                        .help("The key path, must be a .p8 file")
                        .value_name("FILE")
                        .long("key-path")
                        .value_hint(ValueHint::FilePath),
                )
                .arg(
                    Arg::new("issuer-id")
                        .global(true)
                        .help("The issuer id, must be a valid UUID")
                        .long("issuer-id"),
                ),
        )
        .arg(
            Arg::new("machine")
                .global(true)
                .help("Prints the output in a machine-readable format")
                .action(ArgAction::SetTrue)
                .long("machine"),
        );

    let matches = matches.get_matches();
    let (platform, args) = matches.subcommand().unwrap();
    let (command, args) = args.subcommand().unwrap();

    let ios_store: Option<AppStore> = if platform == "ios" {
        Some(AppStore::new(
            args.get_one::<String>("key-path").unwrap().to_string(),
            args.get_one::<String>("issuer-id").unwrap().to_string(),
            args.get_one::<String>("ios-app-id").unwrap().to_string(),
        ))
    } else {
        None
    };

    let android_store: Option<store::GooglePlay> = if platform == "android" {
        Some(store::GooglePlay::new(
            args.get_one::<String>("key-path").unwrap().to_string(),
            args.get_one::<String>("package-name").unwrap().to_string(),
            args.get_one::<String>("track").unwrap().to_string(),
        ))
    } else {
        None
    };

    let mut store: Box<dyn Store> = if platform == "ios" {
        Box::new(ios_store.unwrap())
    } else {
        Box::new(android_store.unwrap())
    };

    match command {
        "version" => {
            let (subcommand, args) = args.subcommand().unwrap();

            match subcommand {
                "create" => {
                    let version = args.get_one::<String>("name").unwrap();
                    let _ = store.create_version(version).unwrap();
                }
                "notes" => {
                    println!("Updating the release notes");
                }
                _ => {
                    unimplemented!("Command not implemented");
                }
            }
        }

        _ => {
            panic!("This should not happen");
        }
    }
}
