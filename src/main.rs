use std::process::exit;

use clap::{command, Arg, Command};
use repositories::store::{self, AppStore, Store};

mod repositories;

fn main() {
    let notes = Command::new("notes")
        .about("Patches the release notes for a certain platform.")
        // Mandatory command arguments
        .arg(
            Arg::new("locale")
                .help("The locale of the release notes")
                .required(true)
                .short('l')
                .long("locale"),
        )
        .arg(
            Arg::new("message")
                .help("The message to be patched")
                .required(true)
                .short('m')
                .long("message"),
        )
        .arg(
            Arg::new("platform")
                .help("The platform to patch the release notes")
                .required(true)
                .short('p')
                .long("platform")
                .value_parser(["android", "ios"]),
        )
        .arg(
            Arg::new("version-name")
                .help("The version name to be patched")
                .required(true)
                .short('n')
                .long("version-name"),
        )
        // Platform specific arguments
        // Android Google Play
        .arg(
            Arg::new("package-name")
                .help("The package name of the app")
                .long("package-name")
                .required_if_eq("platform", "android"),
        )
        .arg(
            Arg::new("google-key-path")
                .help("The path to the service account key file")
                .short('g')
                .long("google-key-path")
                .required_if_eq("platform", "android"),
        )
        // iOS App Store
        .arg(
            Arg::new("ios-app-id")
                .help("The App Store Connect App ID")
                .long("ios-app-id")
                .required_if_eq("platform", "ios"),
        )
        .arg(
            Arg::new("issuer-id")
                .help("The issuer ID, UUID of the App Store Connect API key")
                .long("issuer-id")
                .required_if_eq("platform", "ios"),
        )
        .arg(
            Arg::new("key-path")
                .help("The path to the service account key file. Must be named AuthKey_{KEY_ID}.p8")
                .short('k')
                .long("key-path")
                .required_if_eq("platform", "ios"),
        );

    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(notes)
        .get_matches();

    match matches.subcommand() {
        Some(("notes", matches)) => {
            let platform = matches.get_one::<String>("platform").unwrap().as_str();
            let version_name = matches.get_one::<String>("version-name").unwrap();
            let locale = matches.get_one::<String>("locale").unwrap();
            let notes = matches.get_one::<String>("message").unwrap();

            match platform {
                "ios" => {
                    let mut store = AppStore::new(
                        matches.get_one::<String>("key-path").unwrap().to_string(),
                        matches.get_one::<String>("issuer-id").unwrap().to_string(),
                        matches.get_one::<String>("ios-app-id").unwrap().to_string(),
                    );

                    let result = store.set_changelog(locale, version_name, notes.as_str());

                    match result {
                        Ok(_) => {
                            println!("Done!");
                        }

                        Err(e) => {
                            println!("Error: {}", e);
                            exit(1);
                        }
                    }
                }

                "android" => {
                    let mut store = store::GooglePlay::new(
                        matches
                            .get_one::<String>("google-key-path")
                            .unwrap()
                            .to_string(),
                        matches
                            .get_one::<String>("package-name")
                            .unwrap()
                            .to_string(),
                    );

                    let result = store.set_changelog(locale, version_name, notes);

                    match result {
                        Ok(_) => {
                            println!("Done!");
                        }

                        Err(e) => {
                            println!("Error: {}", e);
                            exit(1);
                        }
                    }
                }

                _ => {
                    panic!("This should not happen!")
                }
            }
        }

        _ => {
            panic!("This should not happen!")
        }
    }
}
