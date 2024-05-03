# sctgdesk-api-server

## Description

This project, sctgdesk-api-server, is a basic implementation of an API server for Rustdesk. Rustdesk is an open-source remote control software. This implementation is written in Rust, utilizing the Rocket framework. The entire REST API is documented using `rocket_okapi`. Upon launching the server, it serves Rapidoc module at `/api/doc`, which allows visualizing and testing the various API routes. The server is configured to listen on port 21114. It is designed to utilize a SQLite3 database compatible with the Rustdesk-server or Rustdesk-server-pro databases.

## Status

The server is currently under development and is not yet ready for production use.

## Licensing

This server is distributed under the terms of the GNU Affero General Public License version 3.0 (AGPL-3.0).

## Authentication

The server includes basic support for authentication with a username and password. Passwords are stored in the database after being hashed with bcrypt. Additionally, similar to Rustdesk-server-pro, it supports authentication with third-party providers compatible with OAuth2. Currently, the username is not retrieved from OAuth2 authentication; the Rustdesk ID is used as the username. This implies that the address book is linked to the Rustdesk ID rather than a username (with OAuth2). All testing has been performed with Dex.

## Configuration

The server requires an `oauth2.toml` configuration file to function. By default, it is expected at `./oauth2.toml`, although this location can be modified using the `OAUTH2_CONFIG_FILE` environment variable. Setting the `OAUTH2_CREATE_USER` variable to `1` enables the automatic creation of a user upon the first OAuth2 login. The user is created with the Rustdesk ID and a random password, which is displayed in the server logs.

## OpenAPI

The server is designed to be fully documented using OpenAPI. The documentation is generated using `rocket_okapi`. The server serves the Rapidoc module at `/api/doc`, which allows visualizing and testing the various API routes.  
Obviously without any test possible a Rapidoc server is deployed at [https://sctg-development.github.io/sctgdesk-api-server/](https://sctg-development.github.io/sctgdesk-api-server/)