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
mod api;
mod extended_json;

use std::collections::HashMap;
use std::env;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;

use api::ActionResponse;
use extended_json::ExtendedJson;
use oauth2::oauth_provider::OAuthProvider;
use oauth2::oauth_provider::OAuthProviderFactory;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::form::validate::Len;
use rocket::http::{ContentType, Header};
use rocket::response::{Redirect, Responder};
use rocket::{async_trait, delete, options, put, routes, uri};
use rocket::{Request, Response};

use s3software::{get_s3_config_file, get_signed_release_url_with_config};
use state::{self};
#[cfg(feature = "ui")]
use ui;
use utils::guid_into_uuid;
use utils::AbProfile;
use utils::AbRule;
use utils::AbRuleAddRequest;
use utils::AbRuleDeleteRequest;
use utils::AbRulesResponse;
use utils::AbSharedAddRequest;
use utils::{
    self, get_host::get_host, AbPeer, AbPeersResponse, AbPersonal, AbSettingsResponse,
    AbSharedProfilesResponse, AbTag, BearerAuthToken, OidcAuthRequest, OidcAuthUrl, OidcResponse,
    OidcState, OidcUser, OidcUserInfo, OidcUserStatus,
};

use base64::prelude::{Engine as _, BASE64_STANDARD};
use rocket::{
    self, figment::Figment, get, post, response::status, serde::json::Json, Build, Rocket, State,
};
use state::{ApiState, UserPasswordInfo};
use utils::{
    include_png_as_base64, unwrap_or_return, uuid_into_guid, AbTagRenameRequest, AddUserRequest,
    AddressBook, EnableUserRequest, Group, GroupsResponse, OidcSettingsResponse, PeersResponse,
    SoftwareResponse, SoftwareVersionResponse, UpdateUserRequest, UserList,
};
use utils::{
    AbGetResponse, AbRequest, AuditRequest, CurrentUserRequest, CurrentUserResponse,
    HeartbeatRequest, LoginReply, LoginRequest, LogoutReply, UserInfo, UsersResponse,
};

type AuthenticatedUser = state::AuthenticatedUser<BearerAuthToken>;
type AuthenticatedAdmin = state::AuthenticatedAdmin<BearerAuthToken>;

use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, settings::UrlObject};
use uuid::Uuid;

use include_dir::{include_dir, Dir};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PUT, DELETE, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

/// # Answers to OPTIONS requests
#[openapi(tag = "Cors")]
#[options("/<_path..>")]
async fn options(_path: PathBuf) -> Result<(), std::io::Error> {
    Ok(())
}

pub async fn build_rocket(figment: Figment) -> Rocket<Build> {
    let state = ApiState::new_with_db("db_v2.sqlite3").await;

    let rocket = rocket::custom(figment)
        .attach(CORS)
        .mount(
            "/",
            openapi_get_routes![
                options,
                login,
                login_options,
                ab_get,
                ab_post,
                ab,
                current_user,
                audit,
                logout,
                heartbeat,
                sysinfo,
                groups,
                group_add,
                users,
                users_client,
                user_add,
                user_enable,
                user_update,
                peers,
                strategies,
                oidc_auth,
                oidc_state,
                oidc_callback,
                oidc_add,
                oidc_get,
                ab_peer_add,
                ab_peer_update,
                ab_peer_delete,
                ab_peers,
                ab_personal,
                ab_tags,
                ab_tag_add,
                ab_tag_update,
                ab_tag_rename,
                ab_tag_delete,
                ab_shared,
                ab_shared_add,
                ab_shared_delete,
                ab_settings,
                ab_rules,
                ab_rule_add,
                ab_rule_delete,
                software,
                software_version,
                webconsole_index,
                webconsole_index_html,
                // webconsole_assets,
            ],
        )
        .mount("/",routes![
            favicon,
            webconsole_vue,
            openapi_snippet
        ])
        .mount(
            "/api/doc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("SCTGDesk API Doc".to_owned()),
                custom_html: Some(include_str!("../rapidoc/index.html").to_owned()),
                slots: SlotsConfig{
                    logo: Some(include_png_as_base64!("../assets/logo.png")),
                    footer: Some(r#"<p slot="footer" style="margin:0; padding:16px 36px; background-color:orangered; color:#fff; text-align:center;"> 
                    © 2024 SCTG. All rights reserved.
                  </p>"#.to_owned()),
                    ..Default::default()
                },
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    allow_spec_file_download: true,
                    show_curl_before_try: true,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .manage(state);

    #[cfg(feature = "ui")]
    {
        rocket = ui::update_rocket(rocket);
    }

    rocket
}

/// # User Login
///
/// This function is an API endpoint that allows a user to log in without oauth.
/// It is tagged with "login" for OpenAPI documentation. <br>
///
/// ## Parameters
/// 
/// - `request`: The request data, which includes the user's username and password.  <br>
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<LoginReply>` object, which includes the user's information and access token.  <br>
/// If the user is not authorized, this function returns a `status::Unauthorized` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the user is not authorized or if the system is in maintenance mode.
#[openapi(tag = "login")]
#[post("/api/login", format = "application/json", data = "<request>")]
async fn login(
    state: &State<ApiState>,
    request: Json<LoginRequest>,
) -> Result<Json<LoginReply>, status::Unauthorized<()>> {
    let status_forbidden = || status::Unauthorized::<()>(());

    let user_password_info = UserPasswordInfo::from_password(request.password.as_str());
    let (user, access_token) = state
        .user_login(&request.username, user_password_info, false)
        .await
        .ok_or_else(status_forbidden)?;

    let reply = LoginReply {
        response_type: "access_token".to_string(),
        user: user,
        access_token,
    };

    log::debug!("login: {:?}", request);

    state.check_maintenance().await;

    Ok(Json(reply))
}

/// # Get the User's Legacy Address Book
///
/// This function is an API endpoint that allows an authenticated user to retrieve their legacy address book. <br>
/// The Legacy Address Book is the address book that was used in the previous version of SCTGDesk. <br>
/// Rustdesk client uses the legacy address book if it cannot find the new one <br>
/// It is tagged with "address book legacy" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - none
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<AbGetResponse>` object, which includes the legacy address book information.  <br>
/// If the user is not authorized, this function returns a `status::Unauthorized` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the user is not authorized.
///
#[openapi(tag = "address book legacy")]
#[get("/api/ab", format = "application/json")]
async fn ab_get(
    state: &State<ApiState>,
    user: AuthenticatedUser,
) -> Result<Json<AbGetResponse>, status::Unauthorized<()>> {
    ab_get_handler(state, user).await
}

/// # Get the User's Address Book
///
/// This function is an API endpoint that allows an authenticated user to retrieve their address book.
/// It is tagged with "address book" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - none
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<AbGetResponse>` object, which includes the address book information.  <br>
/// If the user is not authorized, this function returns a `status::Unauthorized` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the user is not authorized.
///
/// # Example
/// 
/// POST /api/ab/get
#[openapi(tag = "address book")]
#[post("/api/ab/get", format = "application/json")]
async fn ab_post(
    state: &State<ApiState>,
    user: AuthenticatedUser,
) -> Result<Json<AbGetResponse>, status::Unauthorized<()>> {
    ab_get_handler(state, user).await
}

/// Common handler for the user's address book
///
/// # Arguments
///
/// * `state` - The API state
/// * `user` - The authenticated user supplied via a Bearer token
///
/// # Returns
///
/// The user's address book in JSON format
async fn ab_get_handler(
    state: &State<ApiState>,
    user: AuthenticatedUser,
) -> Result<Json<AbGetResponse>, status::Unauthorized<()>> {
    log::debug!("ab get");

    // Get the user's address book from the state
    let abi = state
        .get_user_address_book(user.info.user_id)
        .await
        .unwrap_or_else(|| AddressBook::empty());

    let error = if abi.ab.is_empty() { Some(true) } else { None };
    // Create the reply with the address book and a timestamp
    let reply = AbGetResponse {
        error: error,
        updated_at: if error.is_some() {
            Some("now".to_string())
        } else {
            None
        },
        data: abi.ab,
    };

    // Check if the server is in maintenance mode
    state.check_maintenance().await;

    // Debug log the reply
    log::debug!("ab get reply: {:?}", Json(&reply));

    // Return the reply as JSON
    Ok(Json(reply))
}

/// Set the user's address book
#[openapi(tag = "address book legacy")]
#[post("/api/ab", format = "application/json", data = "<request>")]
async fn ab(
    state: &State<ApiState>,
    user: AuthenticatedUser,
    request: Json<AbRequest>,
) -> Result<(), status::Unauthorized<()>> {
    log::debug!("ab: {:?}", request);

    let ab = request.data.clone();

    log::debug!("new ab: {:?}", &ab);

    let ab = AddressBook { ab, ..Default::default() };

    let _ = unwrap_or_return!(state
        .set_user_address_book(user.info.user_id, ab)
        .await
        .ok_or(Err(status::Unauthorized::<()>(()))));

    state.check_maintenance().await;

    Ok(())
}

/// # Get the Current User
///
/// This function is an API endpoint that allows an authenticated user to retrieve their current user information.
/// It is tagged with "user" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `request`: The request data, which includes the current user information.  <br>
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<CurrentUserResponse>` object, which includes the current user information.  <br>
/// If the user is not authorized, this function returns a `status::Unauthorized` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the user is not authorized.
#[openapi(tag = "user")]
#[post("/api/currentUser", format = "application/json", data = "<request>")]
async fn current_user(
    state: &State<ApiState>,
    user: AuthenticatedUser,
    request: Json<CurrentUserRequest>,
) -> Result<Json<CurrentUserResponse>, status::Unauthorized<()>> {
    log::debug!("current_user authenticated request: {:?}", request);

    let username = unwrap_or_return!(state
        .get_current_user_name(&user.info)
        .await
        .ok_or(Err(status::Unauthorized::<()>(()))));

    let reply = CurrentUserResponse {
        error: false,
        data: UserInfo {
            name: username,
            ..Default::default()
        },
    };

    log::debug!("current_user reply: {:?}", reply);
    Ok(Json(reply))
}

/// Audit
#[openapi(tag = "todo")]
#[post("/api/audit", format = "application/json", data = "<request>")]
async fn audit(state: &State<ApiState>, request: Json<AuditRequest>) {
    log::debug!("audit: {:?}", request);
    state.check_maintenance().await;
}

/// # Log the User Out
///
/// This function is an API endpoint that allows an authenticated user to log out.
/// It is tagged with "login" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `request`: The request data, which includes the current user information.  <br>
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<LogoutReply>` object, which includes a success message.  <br>
/// If the user is not authorized, this function returns a `status::Unauthorized` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the user is not authorized.
///
#[openapi(tag = "login")]
#[post("/api/logout", format = "application/json", data = "<request>")]
async fn logout(
    state: &State<ApiState>,
    user: AuthenticatedUser,
    request: Json<CurrentUserRequest>,
) -> Result<Json<LogoutReply>, status::Unauthorized<()>> {
    log::debug!("logout: {:?}", request);

    let _ = unwrap_or_return!(state
        .user_logout(&user.info)
        .await
        .ok_or(Err(status::Unauthorized::<()>(()))));

    let reply = LogoutReply {
        data: String::new(),
    };

    state.check_maintenance().await;

    Ok(Json(reply))
}

/// # Heartbeat
///
/// This function is an API endpoint that is frequently hit by the client at the /api/heartbeat endpoint.
/// It updates the `last_online` field of the peer.
/// It is tagged with "peer" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `request`: The request data, which includes the heartbeat information.  
///
/// ## Returns
/// 
/// This function always returns a `String` with the message "OK".  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode.
#[openapi(tag = "peer")]
#[post("/api/heartbeat", format = "application/json", data = "<request>")]
async fn heartbeat(state: &State<ApiState>, request: Json<HeartbeatRequest>) -> String {
    log::debug!("heartbeat: {:?}", request);
    let heartbeat = request.0;
    let res = state.update_heartbeat(heartbeat).await;
    log::debug!("res: {:?}", res);
    "OK".to_string()
}

/// # Set the System Info
///
/// This function is an API endpoint that allows a connected client to update its system information.
/// It is tagged with "peer" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `request`: The request data, which includes the system information.  
///
/// ## Returns
/// 
/// If successful, this function returns a `String` with the message "SYSINFO_UPDATED".  <br>
/// If the system info is not found, this function returns a `String` with the message "ID_NOT_FOUND".  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the system info is not found.
///
#[openapi(tag = "peer")]
#[post("/api/sysinfo", format = "application/json", data = "<request>")]
async fn sysinfo(state: &State<ApiState>, request: Json<utils::SystemInfo>) -> String {
    let sysinfo = request.0;
    let res = state.update_systeminfo(sysinfo).await;

    if res.is_none() {
        return "ID_NOT_FOUND".to_string();
    } else {
        return "SYSINFO_UPDATED".to_string();
    }
}

/// # Get the List of Users
///
/// This function is an API endpoint that allows an authenticated admin to retrieve a paginated list of users.
/// It is tagged with "user" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `current`: The current page number.  
/// 
/// - `pageSize`: The number of users per page.  
/// 
/// - `email`: The email to filter the users by.  
/// 
/// - `name`: The name to filter the users by.  
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<UserList>` object, which includes a success message, the total number of users, and the list of users.  <br>
/// If no users are found, this function returns a `status::NotFound` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if no users are found.
///
/// # Example
/// 
/// GET /api/user-list?current=1&pageSize=10&email=test@test.com&name=Test
#[openapi(tag = "user")]
#[get(
    "/api/user-list?<current>&<pageSize>&<email>&<name>",
    format = "application/json"
)]
async fn users(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
    current: u32,
    #[allow(non_snake_case)] pageSize: u32,
    email: Option<&str>,
    name: Option<&str>,
) -> Result<Json<UserList>, status::NotFound<()>> {
    log::debug!("users");
    state.check_maintenance().await;

    let email = if email.is_some() && email.unwrap().is_empty() {
        None
    } else {
        email
    };
    let res = state.get_all_users(name, email, current, pageSize).await;
    if res.is_none() {
        return Err(status::NotFound::<()>(()));
    }
    let response = UserList {
        msg: "success".to_string(),
        total: res.len() as u32,
        data: res.unwrap(),
    };

    Ok(Json(response))
}

/// # Get the List of Groups
///
/// This function is an API endpoint that allows an authenticated admin to retrieve a paginated list of groups.
/// It is tagged with "group" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `current`: The current page number.  
/// 
/// - `pageSize`: The number of groups per page.  
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<GroupsResponse>` object, which includes a success message, the total number of groups, and the list of groups.  <br>
/// If no groups are found, this function returns a `status::NotFound` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if no groups are found.
///
/// # Example
/// 
/// GET /api/groups?current=1&pageSize=10
#[openapi(tag = "group")]
#[get("/api/groups?<current>&<pageSize>", format = "application/json")]
async fn groups(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
    #[allow(unused_variables)] current: u32,
    #[allow(non_snake_case, unused_variables)] pageSize: u32,
) -> Result<Json<GroupsResponse>, status::NotFound<()>> {
    log::debug!("groups");
    state.check_maintenance().await;
    let offset = if current < 1 { 0 } else { current - 1 };
    let page_size = if pageSize < 1 {
        u32::max_value()
    } else {
        pageSize
    };
    let groups = state.get_groups(offset, page_size).await;
    if groups.is_none() {
        return Err(status::NotFound::<()>(()));
    }
    let groups = groups.unwrap();
    let response = GroupsResponse {
        msg: "success".to_string(),
        total: groups.len() as u32,
        data: groups,
    };

    Ok(Json(response))
}

/// # Add a Group (todo)
///
/// This function is an API endpoint that allows an authenticated admin to add a new group.
/// It is tagged with "group" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `request`: The request data, which includes the details of the group to be added.  <br>
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<UsersResponse>` object, which includes a success message, the total number of groups, and the list of groups.  <br>
/// If the admin is not authorized, this function returns a `status::Unauthorized` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the admin is not authorized.
///
/// # Example
/// 
/// POST /api/group
/// {"name":"new group","password":"string","confirm-password":"string","email":"string","is_admin":false,"group_name":"string"}
#[openapi(tag = "group")]
#[post("/api/group", format = "application/json", data = "<_request>")]
async fn group_add(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
    _request: Json<AddUserRequest>,
) -> Result<Json<UsersResponse>, status::Unauthorized<()>> {
    log::debug!("create_group");
    state.check_maintenance().await;

    let response = UsersResponse {
        msg: "success".to_string(),
        total: 1,
        data: "[{}]".to_string(),
    };

    Ok(Json(response))
}

/// # Get Peers
///
/// This function is an API endpoint that retrieves the list of all peers in the network.
/// It is tagged with "peer" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - none
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<PeersResponse>` object, which includes a success message, the total number of peers, and the list of peers.  <br>
/// If no peers are found, this function returns a `status::NotFound` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if no peers are found.
///
/// # Example
/// 
/// GET /api/peers
#[openapi(tag = "peer")]
#[get("/api/peers", format = "application/json")]
async fn peers(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
) -> Result<Json<PeersResponse>, status::NotFound<()>> {
    log::debug!("peers");
    state.check_maintenance().await;
    let peers = state.get_all_peers().await;

    if peers.is_none() {
        return Err(status::NotFound::<()>(()));
    }
    Ok(Json(PeersResponse {
        msg: "success".to_string(),
        total: peers.len() as u32,
        data: peers.unwrap(),
    }))
}

/// # Login Options
///
/// This is called by the client for knowing the Oauth2 provider(s) available
/// You must provide a list of Oauth2 providers in the `oauth2.toml` config file
/// The config file can be overridden by the `OAUTH2_CONFIG_FILE` environment variable
///
/// This function is an API endpoint that is called by the client to get the list of available OAuth2 providers.
/// The list of providers is defined in the `oauth2.toml` config file, which can be overridden by the `OAUTH2_CONFIG_FILE` environment variable.
/// It is tagged with "login" for OpenAPI documentation.
///
/// ## Limitations
/// 
/// It needs to be completed for mapping the username and email from the OAuth2 provider to the SCTGDesk user.
///
/// ## Parameters
/// 
/// - none
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<Vec<String>>` object, which includes the list of available OAuth2 providers.  <br>
/// If the config file is not found or cannot be read, this function returns a `status::Unauthorized` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the config file is not found or cannot be read.
///
/// # Example
/// 
/// GET /api/login-options
#[openapi(tag = "login")]
#[get("/api/login-options", format = "application/json")]
async fn login_options(
    state: &State<ApiState>,
) -> Result<Json<Vec<String>>, status::Unauthorized<()>> {
    let mut providers: Vec<String> = Vec::new();
    let providers_config = state
        .get_oauth2_config(oauth2::get_providers_config_file().as_str())
        .await;
    if providers_config.is_none() {
        return Err(status::Unauthorized::<()>(()));
    }
    for p in providers_config.unwrap() {
        providers.push(p.op_auth_string);
    }
    Ok(Json(providers))
}

/// OIDC Auth request
///
/// This entrypoint is called by the client for getting the authorization url for the Oauth2 provider he chooses
///
/// For testing you can generate a valid uuid field with the following command: `uuidgen | base64`
/// # OIDC Auth Request
///
/// This function is an API endpoint that is called by the client to get the authorization URL for the chosen OAuth2 provider.
/// It is tagged with "login" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `request`: The request data, which includes the chosen OAuth2 provider and a UUID.  <br> For testing you can generate a valid uuid field with the following command: `uuidgen | base64`
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<OidcAuthUrl>` object, which includes the authorization URL and a session code.  <br>
/// If the UUID is invalid or the OAuth2 provider is not found, this function returns an `OidcAuthUrl` object with an empty URL and an error code.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the UUID is invalid or the OAuth2 provider is not found.
///
/// # Example
/// 
/// POST /api/oidc/auth
/// {
///     "op": "github",
///     "uuid": "generated_uuid_base64_encoded"
/// }
#[openapi(tag = "login")]
#[post("/api/oidc/auth", format = "application/json", data = "<request>")]
async fn oidc_auth(
    state: &State<ApiState>,
    request: ExtendedJson<OidcAuthRequest>,
) -> Json<OidcAuthUrl> {
    log::debug!("oidc_auth: {:?}", request);
    let headers = request.headers();
    log::debug!("headers: {:?}", headers);
    let request = request.data;

    let uuid_code = Uuid::new_v4().to_string();
    let uuid_decoded = BASE64_STANDARD.decode(request.uuid.clone());
    if uuid_decoded.is_err() {
        return Json(OidcAuthUrl {
            url: "".to_string(),
            code: "UUID_ERROR".to_string(),
        });
    }
    let uuid_decoded = uuid_decoded.unwrap();
    let uuid_client = String::from_utf8(uuid_decoded).unwrap();
    let callback_url = format!("{}/api/oidc/callback", get_host(headers.clone()));
    let providers_config = state
        .get_oauth2_config(oauth2::get_providers_config_file().as_str())
        .await;
    if providers_config.is_none() {
        return Json(OidcAuthUrl {
            url: "".to_string(),
            code: "".to_string(),
        });
    }
    let providers_config = providers_config.unwrap();
    let provider_config = providers_config
        .iter()
        .find(|config| config.op == request.op);

    if provider_config.is_none() {
        return Json(OidcAuthUrl {
            url: "".to_string(),
            code: "".to_string(),
        });
    }
    let provider_config = provider_config.unwrap();
    let provider_trait_object: Arc<dyn OAuthProvider> = {
        match provider_config.provider {
            oauth2::Provider::Github => Arc::new(oauth2::github_provider::GithubProvider::new()),
            oauth2::Provider::Gitlab => todo!(),
            oauth2::Provider::Google => todo!(),
            oauth2::Provider::Apple => todo!(),
            oauth2::Provider::Okta => todo!(),
            oauth2::Provider::Facebook => todo!(),
            oauth2::Provider::Azure => todo!(),
            oauth2::Provider::Auth0 => todo!(),
            oauth2::Provider::Dex => Arc::new(oauth2::dex_provider::DexProvider::new()),
        }
    };

    let redirect_url =
        provider_trait_object.get_redirect_url(callback_url.as_str(), uuid_code.as_str());
    let _oidc_session = state
        .insert_oidc_session(
            uuid_code.clone(),
            OidcState {
                id: request.id.clone(),
                uuid: uuid_client,
                code: None,
                auth_token: None,
                redirect_url: Some(redirect_url.clone()),
                callback_url: Some(callback_url),
                provider: Some(provider_trait_object),
                name: None,
                email: None,
            },
        )
        .await;
    log::debug!("uuid_code: {:?}", uuid_code);

    Json(OidcAuthUrl {
        url: redirect_url.clone(),
        code: uuid_code,
    })
}

/// # OIDC Auth Callback
///
/// This function is an API endpoint that serves as the OAuth2 callback.
/// It exchanges the authorization code for an access token and stores it in the state.
/// It is tagged with "login" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `code`: The authorization code received from the OIDC provider.  
/// 
/// - `state`: The state parameter received from the OIDC provider. This is the session code.  
///
/// ## Returns
/// 
/// If successful, this function returns "OK".  <br>
/// If the session does not exist or the code exchange fails, this function returns "ERROR".  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the session does not exist or the code exchange fails.
///
/// # Example
/// 
/// GET /api/oidc/callback?code=authorization_code&state=session_code
#[openapi(tag = "login")]
#[get("/api/oidc/callback?<code>&<state>")]
async fn oidc_callback(
    apistate: &State<ApiState>,
    code: &str,
    state: &str,
) -> String {
    let oidc_code = state; // this is the session code
    let oidc_authorization_code = code;
    let updated_oidc_session = apistate
        .oidc_session_exchange_code(oidc_authorization_code.to_string(), oidc_code.to_string())
        .await;
    if updated_oidc_session.is_none() {
        return "ERROR".to_string();
    }
    "OK".to_string()
}

/// # OIDC State
///
/// This function is an API endpoint that checks the state of an OpenID Connect (OIDC) session.
/// It is tagged with "login" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `code`: The authorization code received from the OIDC provider.  
/// 
/// - `id`: The identifier of the OIDC session.  
/// 
/// - `uuid`: The UUID of the OIDC session.  
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<Option<OidcResponse>>` object.  <br>
/// If the session does not exist, this function returns `Json(None)`.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the session does not exist.
///
/// # Example
/// 
/// GET /api/oidc/auth-query?code=authorization_code&id=session_id&uuid=session_uuid
#[openapi(tag = "login")]
#[get("/api/oidc/auth-query?<code>&<id>&<uuid>")]
async fn oidc_state(
    state: &State<ApiState>,
    code: &str,
    id: &str,
    uuid: &str,
) -> Json<Option<OidcResponse>> {
    log::debug!("oidc_state: {:?} {:?} {:?}", code, id, uuid);

    let res = state.oidc_check_session(code.to_string()).await;

    if res.is_none() {
        return Json(None);
    }

    let (token, username, userinfo) = res.unwrap();
    let auth_response = OidcResponse {
        access_token: token.to_base64(),
        type_field: "access_token".to_string(),
        tfa_type: "".to_string(),
        secret: "".to_string(),
        user: OidcUser {
            name: username,
            email: "".to_string(),
            note: "".to_string(),
            status: OidcUserStatus::Normal.into(),
            info: OidcUserInfo {
                email_verification: false,
                email_alarm_notification: false,
                login_device_whitelist: Vec::<String>::new(),
                other: HashMap::<String, String>::new(),
            },
            is_admin: userinfo.admin,
            third_auth_type: "Oauth2".to_string(),
        },
    };

    Json(Some(auth_response))
}

/// # Get Personal Address Book
///
/// This function is an API endpoint that retrieves the personal address book of the authenticated user.
/// It is tagged with "address book" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - none
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<AbPersonal>` object.  <br>
/// If the user is not authorized to access their personal address book, this function returns a `status::Unauthorized` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the user is not authorized to access their personal address book.
///
/// # Example
/// 
/// POST /api/ab/personal
#[openapi(tag = "address book")]
#[post("/api/ab/personal")]
async fn ab_personal(
    state: &State<ApiState>,
    user: AuthenticatedUser,
) -> Result<Json<AbPersonal>, status::Unauthorized<()>> {
    state.check_maintenance().await;
    let guid = state.get_ab_personal_guid(user.info.user_id.clone()).await;
    if guid.is_none() {
        return Err(status::Unauthorized::<()>(()));
    }
    let guid = guid.unwrap();
    log::debug!("user: {:?} ab_personal: {:?}", user.info.user_id, guid);
    let ab_personal = AbPersonal {
        guid: guid,
        error: None,
    };
    Ok(Json(ab_personal))
}

/// # Get the Tags
///
/// This function is an API endpoint that retrieves all tags from an address book.
/// It is tagged with "address book" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `ab`: The identifier of the address book.  
///
/// ## Returns
/// 
/// If successful, this function returns a JSON array of `AbTag` objects.  <br>
/// If the address book does not exist or the user is not authorized to access it, this function returns a `status::NotFound` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the address book does not exist or the user is not authorized to access it.
///
/// # Example
/// 
/// POST /api/ab/tags/018fab24-0ae5-731c-be23-88aa4518ea26
#[openapi(tag = "address book")]
#[post("/api/ab/tags/<ab>")]
async fn ab_tags(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
    ab: &str,
) -> Result<Json<Vec<AbTag>>, status::NotFound<()>> {
    state.check_maintenance().await;
    let ab_tags = state.get_ab_tags(ab).await;
    if ab_tags.is_none() {
        return Err(status::NotFound::<()>(()));
    }
    let ab_tags = ab_tags.unwrap();
    Ok(Json(ab_tags))
}

/// # Add a Tag
///
/// This function is an API endpoint that adds a new tag to an address book.
/// It is tagged with "address book" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `ab`: The identifier of the address book.  
/// 
/// - `request`: A JSON object containing the new tag to be added.  
///
/// ## Returns
/// 
/// If successful, this function returns an `ActionResponse::Empty` object.  <br>
/// If the tag already exists or the user is not authorized to add it, this function returns a `status::Unauthorized` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the tag already exists or the user is not authorized to add it.
///
/// # Example
/// 
/// POST /api/ab/tag/add/018fab24-0ae5-731c-be23-88aa4518ea26
/// Content-Type: application/json
/// 
/// {"name": "tag1", "color": "#FF0000"}
#[openapi(tag = "address book")]
#[post(
    "/api/ab/tag/add/<ab>",
    format = "application/json",
    data = "<request>"
)]
async fn ab_tag_add(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
    ab: &str,
    request: Json<AbTag>,
) -> Result<ActionResponse, status::Unauthorized<()>> {
    state.check_maintenance().await;
    let ab_tag = request.0;
    log::debug!("ab_tag_add: {:?}", ab_tag);
    state.add_ab_tag(ab, ab_tag).await;
    Ok(ActionResponse::Empty)
}

/// # Update a Tag
///
/// This function is an API endpoint that updates a tag in an address book.
/// It is tagged with "address book" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `ab`: The identifier of the address book.  
/// 
/// - `request`: A JSON object containing the updated tag.  
///
/// ## Returns
/// 
/// If successful, this function returns an `ActionResponse::Empty` object.  <br>
/// If the tag does not exist or the user is not authorized to update it, this function returns a `status::Unauthorized` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the tag does not exist or the user is not authorized to update it.
///
/// # Example
/// 
/// PUT /api/ab/tag/update/018fab24-0ae5-731c-be23-88aa4518ea26
/// Content-Type: application/json
/// 
/// {"name": "tag1", "color": "#FF0000"}
#[openapi(tag = "address book")]
#[put(
    "/api/ab/tag/update/<ab>",
    format = "application/json",
    data = "<request>"
)]
async fn ab_tag_update(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
    ab: &str,
    request: Json<AbTag>,
) -> Result<ActionResponse, status::Unauthorized<()>> {
    state.check_maintenance().await;
    let ab_tag = request.0;
    log::debug!("ab_tag_update: {:?}", ab_tag);
    state.add_ab_tag(ab, ab_tag).await;
    Ok(ActionResponse::Empty)
}

/// # Rename a Tag
///
/// This function is an API endpoint that renames a tag in an address book.
/// It is tagged with "address book" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `ab`: The identifier of the address book.
/// 
/// - `request`: A JSON object containing the old and new names of the tag.
///
/// ## Returns
/// 
/// If successful, this function returns an `ActionResponse::Empty` object.  <br>
/// If the tag does not exist or the user is not authorized to access it, this function returns a `status::Unauthorized` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the tag does not exist or the user is not authorized to access it.
///
/// # Example
/// 
/// PUT /api/ab/tag/rename/018fab24-0ae5-731c-be23-88aa4518ea26
/// Content-Type: application/json
/// 
/// {"old": "tag1", "new": "tag2"}
#[openapi(tag = "address book")]
#[put(
    "/api/ab/tag/rename/<ab>",
    format = "application/json",
    data = "<request>"
)]
async fn ab_tag_rename(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
    ab: &str,
    request: Json<AbTagRenameRequest>,
) -> Result<ActionResponse, status::Unauthorized<()>> {
    state.check_maintenance().await;
    let ab_tag_old_name = request.0.old;
    let ab_tag_new_name = request.0.new;

    let ab_tag_old = state.get_ab_tag(ab, ab_tag_old_name.as_str()).await;
    if ab_tag_old.is_none() {
        return Err(status::Unauthorized::<()>(()));
    }
    let mut ab_tag_new = ab_tag_old.unwrap();
    ab_tag_new.name = ab_tag_new_name;
    state
        .rename_ab_tag(ab, ab_tag_old_name.as_str(), ab_tag_new)
        .await;
    Ok(ActionResponse::Empty)
}

/// # Delete a Tag
///
/// This function is an API endpoint that deletes a tag from an address book.
/// It is tagged with "address book" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `ab`: The identifier of the address book.  
/// 
/// - `request`: A JSON object containing an array of tag names to be deleted.  
///
/// ## Returns
/// 
/// If successful, this function returns an `ActionResponse::Empty` object.  <br>
/// If the request is empty or the user is not authorized to access it, this function returns a `status::Unauthorized` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the request is empty or the user is not authorized to access it.
///
/// # Example
/// 
/// DELETE /api/ab/tag/018fab24-0ae5-731c-be23-88aa4518ea26
/// Content-Type: application/json
/// 
/// ["tag1", "tag2"]
#[openapi(tag = "address book")]
#[delete("/api/ab/tag/<ab>", format = "application/json", data = "<request>")]
async fn ab_tag_delete(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
    ab: &str,
    request: Json<Vec<String>>,
) -> Result<ActionResponse, status::Unauthorized<()>> {
    if request.0.is_empty() {
        return Err(status::Unauthorized::<()>(()));
    }
    let tags_to_delete = request.0;
    state.check_maintenance().await;
    state.delete_ab_tags(ab, tags_to_delete).await;
    Ok(ActionResponse::Empty)
}

/// # Get Shared Profiles
///
/// This function is an API endpoint that retrieves the shared profiles from an address book.
/// It is tagged with "address book" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - none
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<AbSharedProfilesResponse>` object containing the shared profiles in the address book.  <br>
/// rule: 1: read, 2: write, 3: full control  <br>
/// If the address book does not exist or the user is not authorized to access it, this function returns a `status::Unauthorized` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the address book does not exist or the user is not authorized to access it.
///
/// # Example
/// 
/// {"data":[{"guid":"018fab24-0ae5-731c-be23-88aa4518ea26","name":"shared profile","owner":"admin","rule":3}],"total":2}
#[openapi(tag = "address book")]
#[post("/api/ab/shared/profiles")]
async fn ab_shared(
    state: &State<ApiState>,
    user: AuthenticatedUser,
) -> Result<Json<AbSharedProfilesResponse>, status::Unauthorized<()>> {
    state.check_maintenance().await;
    let shared_address_books = state.get_shared_address_books(user.info.user_id).await;
    let mut ab_shared_profiles = AbSharedProfilesResponse::default();
    for ab in shared_address_books.expect("shared_address_books is None") {
        let address_book = AbProfile {
            guid: ab.ab,
            name: ab.name.unwrap_or("".to_string()),
            owner: guid_into_uuid(ab.owner.expect("Invalid owner")).expect("Invalid GUID"),
            rule: 3,
            ..Default::default()
        };
        ab_shared_profiles.data.push(address_book);
    }
    ab_shared_profiles.total = ab_shared_profiles.data.len() as u32;
    Ok(Json(ab_shared_profiles))
}

/// # Settings
/// 
/// This function is an API endpoint that retrieves the settings for an address book.<br>
/// TODO: Implement the settings for an address book.
#[openapi(tag = "address book")]
#[post("/api/ab/settings")]
async fn ab_settings(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
) -> Result<Json<AbSettingsResponse>, status::Unauthorized<()>> {
    state.check_maintenance().await;
    let ab_settings = AbSettingsResponse {
        error: None,
        max_peer_one_ab: std::u32::MAX,
    };
    Ok(Json(ab_settings))
}

/// # List peers
///
/// This function is an API endpoint that lists the peers in an address book.
/// It is tagged with "address book" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `current`: The current page number for pagination. This parameter is currently unused.
/// 
/// - `pageSize`: The number of items per page for pagination. This parameter is currently unused.
/// 
/// - `ab`: The identifier of the address book.
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<AbPeersResponse>` object containing the peers in the address book.  <br>
/// If the address book does not exist or the user is not authorized to access it, this function returns a `status::Unauthorized` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the address book does not exist or the user is not authorized to access it.
///
#[openapi(tag = "address book")]
#[post("/api/ab/peers?<current>&<pageSize>&<ab>")]
async fn ab_peers(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
    #[allow(unused_variables)] current: u32,
    #[allow(non_snake_case, unused_variables)] pageSize: u32,
    ab: &str,
) -> Result<Json<AbPeersResponse>, status::Unauthorized<()>> {
    state.check_maintenance().await;
    let ab_peers = state.get_ab_peers(ab).await;
    if ab_peers.is_none() {
        return Err(status::Unauthorized::<()>(()));
    }
    let ab_peers = ab_peers.unwrap();
    let ab_peer_response = AbPeersResponse {
        error: None,
        total: ab_peers.len() as u32,
        data: ab_peers,
    };
    Ok(Json(ab_peer_response))
}

/// # Add peer
/// 
/// This function is an API endpoint that adds a peer to an address book.
/// 
/// ## Parameters
/// 
/// - `ab`: The identifier of the address book.
/// 
/// - `request`: A JSON object containing the new peer information.
/// 
/// ## Returns
/// 
/// If successful, this function returns an `ActionResponse::Empty` object.
#[openapi(tag = "address book")]
#[post(
    "/api/ab/peer/add/<ab>",
    format = "application/json",
    data = "<request>"
)]
async fn ab_peer_add(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
    request: Json<AbPeer>,
    ab: &str,
) -> Result<ActionResponse, status::Unauthorized<()>> {
    let ab_peer = request.0;
    state.check_maintenance().await;
    state.add_ab_peer(ab, ab_peer).await;
    Ok(ActionResponse::Empty)
}

/// # Update peer
/// 
/// This function is an API endpoint that updates a peer in an address book.
/// 
/// ## Parameters
/// 
/// - `ab`: The identifier of the address book.
/// 
/// - `request`: A JSON object containing the updated peer information.
/// 
/// ## Returns
/// 
/// If successful, this function returns an `ActionResponse::Empty` object.
#[openapi(tag = "address book")]
#[put(
    "/api/ab/peer/update/<ab>",
    format = "application/json",
    data = "<request>"
)]
async fn ab_peer_update(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
    request: Json<AbPeer>,
    ab: &str,
) -> Result<ActionResponse, status::Unauthorized<()>> {
    let mut ab_peer = request.0;
    let old_ab_peer = state.get_ab_peer(ab, ab_peer.id.as_str()).await;
    if old_ab_peer.is_none() {
        return Err(status::Unauthorized::<()>(()));
    }
    let old_ab_peer = old_ab_peer.unwrap();
    ab_peer.hash = ab_peer.hash.or(old_ab_peer.hash);
    ab_peer.password = ab_peer.password.or(old_ab_peer.password);
    ab_peer.username = ab_peer.username.or(old_ab_peer.username);
    ab_peer.hostname = ab_peer.hostname.or(old_ab_peer.hostname);
    ab_peer.platform = ab_peer.platform.or(old_ab_peer.platform);
    ab_peer.alias = ab_peer.alias.or(old_ab_peer.alias);
    ab_peer.tags = ab_peer.tags.or(old_ab_peer.tags);
    ab_peer.force_always_relay = ab_peer
        .force_always_relay
        .or(old_ab_peer.force_always_relay);
    ab_peer.rdp_port = ab_peer.rdp_port.or(old_ab_peer.rdp_port);
    ab_peer.rdp_username = ab_peer.rdp_username.or(old_ab_peer.rdp_username);
    ab_peer.login_name = ab_peer.login_name.or(old_ab_peer.login_name);
    ab_peer.same_server = ab_peer.same_server.or(old_ab_peer.same_server);
    state.check_maintenance().await;
    state.add_ab_peer(ab, ab_peer).await;
    Ok(ActionResponse::Empty)
}

/// # Delete peer
/// 
/// This function is an API endpoint that deletes a peer from an address book.
/// 
/// ## Parameters
/// 
/// - `ab`: The identifier of the address book.
/// 
/// - `request`: A JSON object containing an array of peer IDs to be deleted.
/// 
/// ## Returns
/// 
/// If successful, this function returns an `ActionResponse::Empty` object.
#[openapi(tag = "address book")]
#[delete("/api/ab/peer/<ab>", format = "application/json", data = "<request>")]
async fn ab_peer_delete(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
    ab: &str,
    request: Json<Vec<String>>,
) -> Result<ActionResponse, status::Unauthorized<()>> {
    if request.0.is_empty() {
        return Err(status::Unauthorized::<()>(()));
    }
    let peers_to_delete = request.0;
    state.check_maintenance().await;
    state.delete_ab_peer(ab, peers_to_delete).await;
    Ok(ActionResponse::Empty)
}

/// # List strategies
/// 
/// This function is an API endpoint that retrieves the list of all strategies. <br>
/// TODO: This function is currently unused.
/// 
#[openapi(tag = "todo")]
#[get("/api/stategies", format = "application/json")]
async fn strategies(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
) -> Result<Json<UsersResponse>, status::NotFound<()>> {
    log::debug!("peers");
    state.check_maintenance().await;

    let response = UsersResponse {
        msg: "success".to_string(),
        total: 1,
        data: "[{}]".to_string(),
    };

    Ok(Json(response))
}

/// # Add user
/// 
/// This function is an API endpoint that adds a new user.
/// 
/// ## Parameters
/// 
/// - `request`: A JSON object containing the new user information.
/// 
/// ## Returns
/// 
/// If successful, this function returns a `Json<UsersResponse>` object containing the updated user information.
#[openapi(tag = "user")]
#[post("/api/user", format = "application/json", data = "<request>")]
async fn user_add(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
    request: Json<AddUserRequest>,
) -> Result<Json<UsersResponse>, status::Unauthorized<()>> {
    log::debug!("create_user");
    state.check_maintenance().await;

    let user_parameters = request.0;
    if user_parameters.password != user_parameters.confirm_password {
        return Ok(Json(UsersResponse {
            msg: "error: Passwords mismatch".to_string(),
            total: 0,
            data: "[{}]".to_string(),
        }));
    }
    let res = state.add_user(user_parameters).await;
    if res.is_none() {
        return Err(status::Unauthorized::<()>(()));
    }
    let response = UsersResponse {
        msg: "success".to_string(),
        total: 1,
        data: "[{}]".to_string(),
    };

    Ok(Json(response))
}

/// # Enable users
/// 
/// This function is an API endpoint that enables or disables users.
/// 
/// ## Parameters
/// 
/// - `request`: A JSON object containing the list of users to enable or disable.
/// 
/// ## Returns
/// 
/// If successful, this function returns a `Json<UsersResponse>` object containing the updated user information.
#[openapi(tag = "user")]
#[post("/api/enable-users", format = "application/json", data = "<request>")]
async fn user_enable(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
    request: Json<EnableUserRequest>,
) -> Result<Json<UsersResponse>, status::Unauthorized<()>> {
    log::debug!("create_user");
    state.check_maintenance().await;

    let enable_users = request.0;

    let mut count = 0;
    for uuid in enable_users.rows {
        let res = state
            .user_change_status(uuid.as_str(), enable_users.disable)
            .await;
        if res.is_some() {
            count += 1;
        }
    }
    let response = UsersResponse {
        msg: "success".to_string(),
        total: count,
        data: "[{}]".to_string(),
    };

    Ok(Json(response))
}

/// # Update user
/// 
/// This function is an API endpoint that updates a user.<br>
/// Normal user can only update themselves, admin can update any user.<br>
/// 
/// ## Parameters
/// 
/// - `request`: A JSON object containing the updated user information.
/// 
/// ## Returns
/// 
/// If successful, this function returns a `Json<UsersResponse>` object containing the updated user information.
#[openapi(tag = "user")]
#[put("/api/user", format = "application/json", data = "<request>")]
async fn user_update(
    state: &State<ApiState>,
    user: AuthenticatedUser,
    request: Json<UpdateUserRequest>,
) -> Result<Json<UsersResponse>, status::Unauthorized<()>> {
    log::debug!("update_user");
    state.check_maintenance().await;
    let mut guid = uuid_into_guid(request.0.uuid.as_str());
    if guid.is_none() {
        guid = Some(user.info.user_id.clone());
    }

    let guid = guid.unwrap();
    let is_admin = state
        .is_current_user_admin(&user.info)
        .await
        .unwrap_or(false);

    if !is_admin && user.info.user_id != guid {
        return Err(status::Unauthorized::<()>(()));
    }
    let response = UsersResponse {
        msg: "success".to_string(),
        total: 1,
        data: "[{}]".to_string(),
    };
    let user_update = request.0;
    state.user_update(guid, user_update).await;
    Ok(Json(response))
}

/// # Add OIDC Provider
/// 
/// This function is an API endpoint that adds an OIDC provider.
/// 
/// TODO: This function is currently unused.
#[openapi(tag = "todo")]
#[put("/api/oidc/settings", format = "application/json", data = "<_request>")]
async fn oidc_add(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
    _request: Json<EnableUserRequest>,
) -> Result<Json<EnableUserRequest>, status::Unauthorized<()>> {
    log::debug!("Add OIDC Provider");
    state.check_maintenance().await;

    Err(status::Unauthorized::<()>(()))
}

/// # Get OIDC Providers
/// 
/// This function is an API endpoint that retrieves all OIDC providers.
/// 
/// TODO: This function is currently unused.
#[openapi(tag = "todo")]
#[get("/api/oidc/settings", format = "application/json")]
async fn oidc_get(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
) -> Result<Json<OidcSettingsResponse>, status::Unauthorized<()>> {
    log::debug!("create_user");
    state.check_maintenance().await;
    Err(status::Unauthorized::<()>(()))
}

/// # Get Users for client
/// 
/// This function is an API endpoint that retrieves all users.
/// 
/// ## Parameters
/// 
/// - `current`: The current page number for pagination. This parameter is currently unused.
/// 
/// - `pageSize`: The number of items per page for pagination. This parameter is currently unused.
/// 
/// - `accessible`: A boolean value indicating whether the user is accessible. This parameter is currently unused.
/// 
/// - `status`: The status of the user. This parameter is currently unused.
/// 
/// ## Returns
/// 
/// If successful, this function returns a `Json<UserList>` object containing the users.
#[openapi(tag = "user")]
#[get(
    "/api/users?<current>&<pageSize>&<accessible>&<status>",
    format = "application/json"
)]
async fn users_client(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
    current: u32,
    #[allow(non_snake_case, unused_variables)] pageSize: u32,
    #[allow(unused_variables)] accessible: Option<bool>,
    #[allow(unused_variables)] status: Option<u32>,
) -> Result<Json<UserList>, status::NotFound<()>> {
    log::debug!("users");
    state.check_maintenance().await;

    let res = state.get_all_users(None, None, current, pageSize).await;
    if res.is_none() {
        return Err(status::NotFound::<()>(()));
    }
    let response = UserList {
        msg: "success".to_string(),
        total: res.len() as u32,
        data: res.unwrap(),
    };

    Ok(Json(response))
}

/// Get the software download url
///
/// # Arguments
///
/// * `key` - The key to the software download link, it can be `osx`, `w64` or `ios`
///
/// # Usage
///
/// * it needs a valid S3 configuration file defined with the `S3_CONFIG_FILE` environment variable
///
/// <pre>
/// [s3config]<br>
/// Endpoint = "https://compat.objectstorage.eu-london-1.oraclecloud.com"<br>
/// Region = "eu-london-1"<br>
/// AccessKey = "c324ead11faa0d87337c07ddc4a1129fab76188d"<br>
/// SecretKey = "GJurV55f/LD36kjZFpchZMj/uvgTqxHyFkBchUUa8KA="<br>
/// Bucket = "aezoz24elapn"<br>
/// Windows64Key = "master/sctgdesk-releases/sctgdesk-1.2.4-x86_64.exe"<br>
/// Windows32Key = "master/sctgdesk-releases/sctgdesk-1.2.4-i686.exe"<br>
/// OSXKey = "master/sctgdesk-releases/sctgdesk-1.2.4.dmg"<br>
/// OSXArm64Key = "master/sctgdesk-releases/sctgdesk-1.2.4.dmg"<br>
/// IOSKey = "master/sctgdesk-releases/sctgdesk-1.2.4.ipa"<br>
/// </pre>
/// 
#[openapi(tag = "software")]
#[get(
    "/api/software/client-download-link/<key>",
    format = "application/json"
)]
async fn software(key: &str) -> Result<Json<SoftwareResponse>, status::NotFound<()>> {
    log::debug!("software");
    let config = get_s3_config_file()
        .await
        .map_err(|e| status::NotFound(Box::new(e)));

    let config = config.unwrap();
    match key {
        "osx" => {
            let key = config.clone().s3config.osxkey;
            let url = get_signed_release_url_with_config(config, key.as_str())
                .await
                .map_err(|e| status::NotFound(Box::new(e)));
            let url = url.unwrap();
            let response = SoftwareResponse { url };
            Ok(Json(response))
        }
        "w64" => {
            let key = config.clone().s3config.windows64_key;
            let url = get_signed_release_url_with_config(config, key.as_str())
                .await
                .map_err(|e| status::NotFound(Box::new(e)));
            let url = url.unwrap();
            let response = SoftwareResponse { url };
            Ok(Json(response))
        }
        "ios" => {
            let key = config.clone().s3config.ioskey;
            let url = get_signed_release_url_with_config(config, key.as_str())
                .await
                .map_err(|e| status::NotFound(Box::new(e)));
            let url = url.unwrap();
            let response = SoftwareResponse { url };
            Ok(Json(response))
        }
        _ => Err(status::NotFound(())),
    }
}

/// # Retrieve the server version
/// 
/// This function is an API endpoint that retrieves the version of the server.
/// It is tagged with "software" for OpenAPI documentation.
/// 
/// ## Returns
/// 
/// If successful, this function returns a `Json<SoftwareVersionResponse>` object containing the version of the server.
#[openapi(tag = "software")]
#[get("/api/software/version/server", format = "application/json")]
async fn software_version() -> Json<SoftwareVersionResponse> {
    log::debug!("software_version");
    let version = env::var("MAIN_PKG_VERSION").unwrap();
    let response = SoftwareVersionResponse {
        server: Some(version),
        client: None,
    };
    Json(response)
}

/// # List the rules
///
/// This function is an API endpoint that lists the rules attached to a shared address book.
/// It is tagged with "address book" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `current`: The current page number for pagination. This parameter is currently unused.
/// 
/// - `pageSize`: The number of items per page for pagination. This parameter is currently unused.
/// 
/// - `ab`: The identifier of the shared address book.
///
/// ## Returns
/// 
/// If successful, this function returns a `Json<AbRulesResponse>` object containing the rules for the address book.  <br>
/// If the address book does not exist or the user is not authorized to access it, this function returns a `status::Unauthorized` error.  <br>
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode, or if the address book does not exist or the user is not authorized to access it.
///
#[openapi(tag = "address book")]
#[get("/api/ab/rules?<current>&<pageSize>&<ab>", format = "application/json")]
async fn ab_rules(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
    current: u32,
    #[allow(unused_variables)] pageSize: u32,
    ab: &str,
) -> Result<Json<AbRulesResponse>, status::Unauthorized<()>> {
    state.check_maintenance().await;
    let current = if (current < 1) { 0 } else { current - 1};
    let rules = state.get_ab_rules(current,pageSize,ab).await;
    if rules.is_none() {
        return Err(status::Unauthorized::<()>(()));
    }
    let rules = rules.unwrap();
    let response = AbRulesResponse {
        msg: "success".to_string(),
        total: rules.len() as u32,
        data: rules,
    };
    Ok(Json(response))
}

/// # Add a Rule
///
/// This function is an API endpoint that adds a new rule to a shared address book.
/// It is tagged with "address book" for OpenAPI documentation.
///
/// ## Parameters
/// 
/// - `request`: The request containing the details of the rule to be added.
///
/// ## Returns
/// 
/// If successful, this function returns an `ActionResponse::Empty` indicating that the rule was successfully added. <br>
/// If the system is in maintenance mode, this function returns a `status::Unauthorized` error.
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode.
#[openapi(tag = "address book")]
#[post("/api/ab/rule", format = "application/json", data = "<request>")]
async fn ab_rule_add(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
    request: Json<AbRuleAddRequest>,
) -> Result<ActionResponse, status::Unauthorized<()>> {
    state.check_maintenance().await;
    let rule = AbRule {
        guid: request.0.guid,
        user: request.0.user,
        group: request.0.group,
        rule: request.0.rule,
    };
    state.add_ab_rule(rule).await;
    Ok(ActionResponse::Empty)
}

/// # Delete a Rule
///
/// This function is an API endpoint that deletes a rule from a shared address book.
/// It is tagged with "address book" for OpenAPI documentation.
///
/// ## Parameters
///
/// - `request`: The request containing the GUID of the rule to be deleted.
///
/// ## Returns
/// 
/// If successful, this function returns an `ActionResponse::Empty` indicating that the rule was successfully deleted. <br>
/// If the system is in maintenance mode, this function returns a `status::Unauthorized` error.
///
/// ## Errors
/// 
/// This function will return an error if the system is in maintenance mode.
#[openapi(tag = "address book")]
#[delete("/api/ab/rule", format = "application/json", data = "<request>")]
async fn ab_rule_delete(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
    request: Json<AbRuleDeleteRequest>,
) -> Result<ActionResponse, status::Unauthorized<()>> {
    state.check_maintenance().await;
    let rule = request.0.guid;
    state.delete_ab_rule(rule.as_str()).await;
    Ok(ActionResponse::Empty)
}

/// # Add shared profile
/// 
/// TODO: Add shared profile
#[openapi(tag = "address book")]
#[post("/api/ab/shared/add", format = "application/json", data = "<request>")]
async fn ab_shared_add(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
    request: Json<AbSharedAddRequest>,
) -> Result<Json<AbSharedProfilesResponse>, status::Unauthorized<()>> {
    state.check_maintenance().await;
    let ab_shared_profiles = AbSharedProfilesResponse::default();
    Ok(Json(ab_shared_profiles))
}

/// # Delete shared profiles
/// 
/// TODO: Delete shared profiles
#[openapi(tag = "address book")]
#[delete("/api/ab/shared", format = "application/json", data = "<request>")]
async fn ab_shared_delete(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
    request: Json<Vec<String>>,
) -> Result<ActionResponse, status::Unauthorized<()>> {
    state.check_maintenance().await;
    Ok(ActionResponse::Empty)
}

async fn webconsole_index_multi() -> Redirect {
    Redirect::to(uri!("/ui/"))
}

#[openapi(tag = "webconsole")]
#[get("/index.html")]
async fn webconsole_index_html() -> Redirect {
    webconsole_index_multi().await
}

#[openapi(tag = "webconsole")]
#[get("/")]
async fn webconsole_index() -> Redirect {
    webconsole_index_multi().await
}

const STATIC_DIR: Dir = include_dir!("webconsole/dist");
#[derive(Debug)]
struct StaticFileResponse(Vec<u8>, ContentType);

#[async_trait]
impl<'r> Responder<'r, 'r> for StaticFileResponse {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        Response::build()
            .header(self.1)
            .header(Header {
                name: "Cache-Control".into(),
                value: "max-age=604800".into(), // 1 week
            })
            .sized_body(self.0.len(), Cursor::new(self.0))
            .ok()
    }
}

#[get("/js/openapisnippet.min.js")]
async fn openapi_snippet() -> Option<StaticFileResponse> {
    let content = include_str!("../rapidoc/openapisnippet.min.js");
    Some(StaticFileResponse(
        content.as_bytes().to_vec(),
        ContentType::JavaScript,
    ))
}

#[get("/favicon.ico")]
async fn favicon() -> Redirect {
    Redirect::to(uri!("/ui/favicon.ico"))
}

/// Retrieves a static file from the webconsole/dist directory
///
/// # Arguments
///
/// * `path` - the path to the file relative to the webconsole/dist directory
///
/// # Returns
///
/// * `Some(StaticFileResponse)` if the file exists, containing the file data and content type
/// * `None` if the file does not exist
#[get("/ui/<path..>")]
async fn webconsole_vue(path: PathBuf) -> Option<StaticFileResponse> {
    if env::var("VITE_DEVELOPMENT").is_ok() {
        let vite_base = env::var("VITE_DEVELOPMENT").unwrap_or("http://localhost:5173".to_string());
        let url = format!("{}/ui/{}", vite_base, path.to_str().unwrap_or(""));
        let response = reqwest::get(&url).await.unwrap();
        let content_type = response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<ContentType>()
            .unwrap();
        let bytes = response.bytes().await.unwrap();
        let response_content: Vec<u8> = bytes.iter().map(|byte| *byte).collect();
        let content = StaticFileResponse(response_content, content_type);
        return Some(content);
    }

    let path = path.to_str().unwrap_or("");
    let file = STATIC_DIR.get_file(path).map(|file| {
        let content_type = ContentType::from_extension(
            file.path()
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap(),
        )
        .unwrap_or(ContentType::Binary);
        StaticFileResponse(file.contents().to_vec(), content_type)
    });
    if file.is_some() {
        return file;
    } else {
        let file = STATIC_DIR.get_file("index.html").map(|file| {
            let content_type = ContentType::from_extension(
                file.path()
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap(),
            )
            .unwrap_or(ContentType::Binary);
            StaticFileResponse(file.contents().to_vec(), content_type)
        });
        return file;
    }
}
