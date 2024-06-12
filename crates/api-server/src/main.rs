// Copyright (c) 2024 Ronan LE MEILLAT for SCTG Development
//
// This file is part of the SCTGDesk project.
//
// SCTGDesk is free software: you can redistribute it and/or modify
// it under the terms of the Affero General Public License version 3 as
// published by the Free Software Foundation.
//
// SCTGDesk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// Affero General Public License for more details.
//
// You should have received a copy of the Affero General Public License
// along with SCTGDesk. If not, see <https://www.gnu.org/licenses/agpl-3.0.html>.
use rocket::{
    config::LogLevel,
    data::{Limits, ToByteUnit},
};
use sctgdesk_api_server::build_rocket;
use clap::{Arg, Command};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Command line argument parsing
    let matches = Command::new("SCTGDeskApiServer")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Runs the SCTGDesk API Server")
        .arg(Arg::new("address")
            .long("address")
            .value_name("ADDRESS")
            .help("Sets the address for the server")
            .to_owned()
            .default_value("127.0.0.1"))
        .arg(Arg::new("port")
            .long("port")
            .value_name("PORT")
            .help("Sets the port for the server")
            .to_owned()
            .default_value("21114"))
        .arg(Arg::new("log_level")
            .long("log_level")
            .value_name("LOG_LEVEL")
            .help("Sets the log level for the server")
            .to_owned()
            .default_value("debug"))
        .arg(Arg::new("secret_key")
            .long("secret_key")
            .value_name("SECRET_KEY")
            .help("Sets the secret key for the server")
            .to_owned()
            .default_value("wJq+s/xvwZjmMX3ev0p4gQTs9Ej5wt0brsk3ZGhoBTg="))
        .get_matches();

    // Get values from command line arguments
    let address = matches.get_one::<String>("address").unwrap();
    let port = (matches.get_one::<String>("port").unwrap()).parse::<u16>().unwrap();
    let log_level = match (matches.get_one::<String>("log_level").unwrap() as &str).to_lowercase().as_str() {
        "off" => LogLevel::Off,
        "critical" => LogLevel::Critical,
        "normal" => LogLevel::Normal,
        "debug" => LogLevel::Debug,
        _ => LogLevel::Debug,
    };
    let secret_key = matches.get_one::<String>("secret_key").unwrap();

    // Configure Rocket
    let figment = rocket::Config::figment()
        .merge(("address", address))
        .merge(("port", port))
        .merge(("log_level", log_level))
        .merge(("secret_key", secret_key))
        .merge(("ident", format!("SCTGDeskApiServer/{}", env!("CARGO_PKG_VERSION"))))
        .merge(("limits", Limits::new().limit("json", 2.mebibytes())));

    // Launch Rocket
    let _rocket = build_rocket(figment).await.ignite().await?.launch().await?;
    
    // End of API Server start
    // Other stuff here
    Ok(())
}
