* Integrated server: [![Full server](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml/badge.svg)](https://github.com/sctg-development/sctgdesk-server/actions/workflows/multiarch-docker-hub.yml)  
* Standalone nightly build: [![Build and Deploy](https://github.com/sctg-development/sctgdesk-api-server/actions/workflows/build.yaml/badge.svg)](https://github.com/sctg-development/sctgdesk-api-server/actions/workflows/build.yaml)

[**Download full server**](https://github.com/sctg-development/sctgdesk-server/releases)  
[**API Documentation**](https://sctg-development.github.io/sctgdesk-api-server/)  


# sctgdesk-api-server

## Disclaimer

First of all this project was not developped to be a replacement of Rustdesk-server-pro. It was developped for our requirements and to be used with Rustdesk.  
So there is no warranty, no support, no guarantee, no liability.  
The target use it to be included as a crate in rustdesk-server.  
As the rustdesk-server was published under AGPL-3.0, this project is also published under AGPL-3.0. That's why you see it !  
Feel free to use it, modify it, but don't forget to publish your modifications.  

## Description

This project, sctgdesk-api-server, is a basic implementation of an API server for Rustdesk. Rustdesk is an open-source remote control software. This implementation is written in Rust, utilizing the Rocket framework. The entire REST API is documented using `rocket_okapi`. Upon launching the server, it serves Rapidoc module at `/api/doc`, which allows visualizing and testing the various API routes. The server is configured to listen on port 21114. It is designed to utilize a SQLite3 database compatible with the Rustdesk-server or Rustdesk-server-pro databases.

## Status

The server is currently under development and is not yet ready for production use.

## Licensing

This server is distributed under the terms of the GNU Affero General Public License version 3.0 (AGPL-3.0).

## Features

* API server for Rustdesk
* New address book API
  * Support for personal address book
  * Support for shared address book at group level
    * read-only, read-write, admin (currently rules need to be set manually in the database)
  * Support for shared address book at user level
    * read-only, read-write, admin (currently rules need to be set manually in the database)
* OpenAPI documentation
* Web console (work in progress)

## Architecture

The server use Rocket as the web framework. The server is designed to be modular and extensible. The server is divided into three main parts:

* The `api` module contains the API routes and the API logic. It is 100% Rust code with Rocket framework.
* The `webconsole` module contains the web console. It is a single page Vue.js application written in Typescript. The API is automatically generated from the OpenAPI with Swagger codegen for Axios Typescript. You can access the web console at `/ui`. You'll find the code in the `webconsole` directory.
* The `openapi` module contains the OpenAPI documentation. It is generated with `rocket_okapi`. You can access the Rapidoc module at `/api/doc`.

## Authentication

The server includes basic support for authentication with a username and password. Passwords are stored in the database after being hashed with bcrypt. Additionally, similar to Rustdesk-server-pro, it supports authentication with third-party providers compatible with OAuth2. Currently, only Github and Dex (as a custom provider) are available. For adding a new provider you must implement the `OAuthProvider` and `OAuthProviderFactory` traits. You can look at the [github_provider.rs](https://github.com/sctg-development/sctgdesk-api-server/blob/main/libs/oauth2/src/github_provider.rs) and [dex_provider.rs](https://github.com/sctg-development/sctgdesk-api-server/blob/main/libs/oauth2/src/dex_provider.rs) files for examples.  
The first time you launch the server it will create a default user with the username `admin` and the password `Hello,world!`. You can change the password after the first login on the webconsole.

## S3 url generation

Our custom clients are stored in a S3 bucket. The S3 configuration is stored in the `s3config.toml` file. The server generates a signed URL for the client download. The URL is valid for 5 minutes. The server generates download links at:

* `/api/software/client-download-link/<key>` for the client download
  * key can be one of osx w64 or ios

## Configuration

The server requires an `oauth2.toml` configuration file to function. By default, it is expected at `./oauth2.toml`, although this location can be modified using the `OAUTH2_CONFIG_FILE` environment variable. Setting the `OAUTH2_CREATE_USER` variable to `1` enables the automatic creation of a user upon the first OAuth2 login. The user is created with the Rustdesk ID and a random password, which is displayed in the server logs.  
The server also requires a `s3config.toml` configuration file to function. By default, it is expected at `./s3config.toml`, although this location can be modified using the `S3_CONFIG_FILE` environment variable. The S3 configuration file is used to configure the S3 storage for the server.  
If you don't provide this two files, the server will create them for you in the working directory.

## OpenAPI

The server is designed to be fully documented using OpenAPI. The documentation is generated using `rocket_okapi`. The server serves the Rapidoc module at `/api/doc`, which allows visualizing and testing the various API routes.  
Obviously without any test possible a Rapidoc server is deployed at [https://sctg-development.github.io/sctgdesk-api-server/](https://sctg-development.github.io/sctgdesk-api-server/)  
The typescript client api is autogenerated with `swagger-codegen.sh`

## Web console

A web console is available at `/ui` it is a work in progress and is not yet ready for production use.  
It is a stub for the future sctgdesk-api-server web console.  
The choosen framework is Vue.js. The API is automatically generated from the OpenAPI with Swagger codegen for Axios Typescript. Note the codegen is not yet ready for production use and a few modifications are needed.  
For regenerating the api code, run the following command **after** the server is running (it needs docker to be running):

```bash
./swagger-codegen.sh
```

### Development

To start the ui development server, run the following commands:

```bash
cd webconsole && npm i && npm run devserver &
VITE_DEVELOPMENT="http://localhost:5173" sctgdesk-api-server
```

It will start a nodejs ui development server on port 5173. Sctgdesk-api-server will proxy the requests to ui development server rather than serving embedded static files. Access the development ui at `http://localhost:21114/ui` .

Each time you modify the code, the server will automatically rebuild and reload the ui development server.

## Standalone API server

The server can be run as a standalone server. To run the server, execute the following command:

In  development mode:
```bash
DATABASE_URL=sqlite://$(pwd)/db_v2.sqlite3 cargo run --release
```

In production mode:
```bash
sctgdesk-api-server --help
Runs the SCTGDesk API Server

Usage: sctgdesk-api-server [OPTIONS]

Options:
      --address <ADDRESS>        Sets the address for the server [default: 127.0.0.1]
      --port <PORT>              Sets the port for the server [default: 21114]
      --log_level <LOG_LEVEL>    Sets the log level for the server [default: debug]
      --secret_key <SECRET_KEY>  Sets the secret key for the server [default: wJq+s/xvwZjmMX3ev0p4gQTs9Ej5wt0brsk3ZGhoBTg=]
  -h, --help                     Print help
  -V, --version                  Print version
```
## Screenshots

### Webconsole

<img width="1085" alt="login" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/fe72a374-8a98-4606-8632-3d919f9317c9">

<img width="1285" alt="dashboard" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/0bb148d6-8723-491f-88c5-b98331d64f61">

<img width="1085" alt="devices" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/6ae55861-f65c-4950-a068-f22eef3ad81a">

<img width="1084" alt="users" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/8d225841-43f5-44f4-8d41-5b6ca3324096">

<img width="1087" alt="groups" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/d84ce3d3-1d19-4765-883f-001f313a4a1e">

<img width="1089" alt="address books" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/db13010b-077a-4e14-943b-9d8de3266f82">

<img width="730" alt="rues" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/3a990deb-d8bb-4725-a47d-435ec3667fee">

<img width="621" alt="add rules" src="https://github.com/sctg-development/sctgdesk-api-server/assets/165936401/355f3903-2b54-4b08-abd0-e33c84a260ed">



### Api documentation

<img width="1502" alt="apidoc" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/88fe7910-fe62-43e5-a16c-70dc1201e040">

### Use in Rustdesk client

<img width="913" alt="Capture d’écran 2024-05-24 à 12 14 34" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/1b253577-dce2-4163-9a49-ba4b3da37812">

<img width="923" alt="Capture d’écran 2024-05-24 à 12 07 21" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/c49b3aba-b13f-4b15-a69c-d492a90e774a">

<img width="927" alt="Capture d’écran 2024-05-24 à 12 07 32" src="https://github.com/sctg-development/sctgdesk-server/assets/165936401/f447f5fa-bc77-4bc6-858a-c6cadf9b7f6c">

## Integration with Rustdesk-Server

The server can be integrated with the Rustdesk-server you can easily integrate it by modifying the [main.rs](https://github.com/sctg-development/sctgdesk-server/blob/tcpserver-master-build/src/main.rs) file of the Rustdesk-server. :

```rust
use sctgdesk_api_server::build_rocket;

#[rocket::main]
async fn start_rocket() -> ResultType<()> {
    let port = get_arg_or("port", RENDEZVOUS_PORT.to_string()).parse::<i32>()?;
    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", port-2))
        .merge(("log_level", LogLevel::Debug))
        .merge(("secret_key", "wJq+s/xvwZjmMX3ev0p4gQTs9Ej5wt0brsk3ZGhoBTg="))
        .merge(("limits", Limits::new().limit("json", 2.mebibytes())));
    let _rocket = build_rocket(figment).await.ignite().await?.launch().await?;
    Ok(())
}
```

and in the `main` function:

```rust
    let rocket_thread = thread::spawn(|| {
        let _ = start_rocket();
    });

    RendezvousServer::start(port, serial, &get_arg("key"), rmem)?;
    let _ = rocket_thread.join();
    Ok(())
```

You can look at the [sctgdesk-server main.rs](https://github.com/sctg-development/sctgdesk-server/blob/tcpserver-master-build/src/main.rs) for a working itegration.

### Integrated API server with Rustdesk-Server

if you want a ready to use integrated server with Rustdesk-Server, you can use my own server [sctgdesk-server](https://github.com/sctg-development/sctgdesk-server). Binaries are available at [release page](https://github.com/sctg-development/sctgdesk-server/releases). Binaries are available for Linux Ubuntu 22.04LTS amd64 and arm64.

## Limitations

* The server is not yet ready for production use. Buy a [Rustdesk-server-pro](https://rustdesk.com/pricing.html) license to get a production-ready server.  
* The Bearen tokens are stored in memory without persistence. It means that each time the server is restarted, all the tokens are lost. You will need to re-authenticate with the server.  

## CLI Usage

* User login:  
  
    ```bash
    curl -X POST "http://127.0.0.1:21114/api/login" \
                    -H "accept: application/json"\
                    -H "content-type: application/json" \
                    -d '{"username":"admin","password":"Hello,world!","id":"string","uuid":"string"}' 
        # Note the Bearen token in the response
    ```

* Create user:
  
    ```bash
    curl -X POST "http://127.0.0.1:21114/api/user" \
                    -H "accept: application/json"\
                    -H "authorization: Bearer viZ2ArJutFtKsg0DDC1TiV-87uSRQqGBZXAoCeHrFHc"\
                    -H "content-type: application/json" \
                    -d '{"name":"testuser","password":"test","confirm-password":"test","email":"string","is_admin":false,"group_name":"Default"}' 
    ```

* Use Rapidoc to test the API at http://127.0.0.1:21114/api/doc
