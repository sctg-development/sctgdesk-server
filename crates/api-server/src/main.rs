use rocket::{
    config::LogLevel,
    data::{Limits, ToByteUnit},
};
use sctgdesk_api_server::build_rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    //Run the API Server
    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 21114))
        .merge(("log_level", LogLevel::Debug))
        .merge(("secret_key", "wJq+s/xvwZjmMX3ev0p4gQTs9Ej5wt0brsk3ZGhoBTg="))
        // .merge(("tls.certs", "rustdesk.crt"))
        // .merge(("tls.key", "rustdesk.pem"))
        .merge(("limits", Limits::new().limit("json", 2.mebibytes())));
    let _rocket = build_rocket(figment).await.ignite().await?.launch().await?;
    // End of API Server start

    // Other stuff here
    Ok(())
}