use clap::{command, Arg, Command};
use repositories::store::{AppStore, Store};

mod repositories;

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("patch")
                .about("Patches the specified version with the provided information")
                .arg(
                    Arg::new("version-name")
                        .required(true)
                        .short('v')
                        .long("version-name")
                        .help("Version name to be patched"),
                )
                .arg(
                    Arg::new("notes")
                        .short('n')
                        .long("notes")
                        .help("Release notes"),
                )
                .arg(
                    Arg::new("platform")
                        .short('p')
                        .long("platform")
                        .value_parser(["android", "ios", "all"])
                        .default_value("all")
                        .help("Platform to be patched"),
                )
                .arg(
                    Arg::new("key-path")
                        .short('k')
                        .long("key-path")
                        .value_name("file.p8")
                        .required_if_eq_any([("platform", "ios"), ("platform", "all")])
                        .help(
                            "The path to the service account key file. Must be named AuthKey_{KEY_ID}.p8",
                        ),
                )
                .arg(Arg::new("ios-app-id").short('a').long("ios-app-id").required_if_eq("platform", "ios").help("The App Store Connect App ID"))
                .arg(
                    Arg::new("issuer-id")
                        .short('i')
                        .long("issuer-id")
                        .required_if_eq_any([("platform", "ios"), ("platform", "all")])
                        .help("The issuer ID, UUID of the App Store Connect API key"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("patch", matches)) => {
            let mut store = AppStore::new(
                matches.get_one::<String>("key-path").unwrap().to_string(),
                matches.get_one::<String>("issuer-id").unwrap().to_string(),
                matches.get_one::<String>("ios-app-id").unwrap().to_string(),
            );

            if let Some(notes) = matches.get_one::<String>("notes") {
                let version_name = matches.get_one::<String>("version-name").unwrap();

                let result = store.set_changelog("", version_name, notes.as_str());

                match result {
                    Ok(_) => {
                        println!("Successfully patched the version");
                    }

                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }

        _ => {
            panic!("This should not happen!")
        }
    }
}
