mod api;
mod extended_json;

use std::collections::HashMap;
use std::env;

use api::ActionResponse;
use extended_json::ExtendedJson;

use rocket::form::validate::Len;
use rocket::{delete, put};

use state::{self};
#[cfg(feature = "ui")]
use ui;
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
    include_png_as_base64, unwrap_or_return, AbTagRenameRequest, AddUserRequest, AddressBook, EnableUserRequest, OidcSettingsResponse, UpdateUserRequest, UserList
};
use utils::{
    AbGetResponse, AbRequest, AuditRequest, CurrentUserRequest, CurrentUserResponse,
    HeartbeatRequest, LoginReply, LoginRequest, LogoutReply, UserInfo, UsersResponse,
};

type AuthenticatedUser = state::AuthenticatedUser<BearerAuthToken>;
type AuthenticatedAdmin = state::AuthenticatedAdmin<BearerAuthToken>;

use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, settings::UrlObject};
use uuid::Uuid;

pub async fn build_rocket(figment: Figment) -> Rocket<Build> {
    let state = ApiState::new_with_db("db_v2.sqlite3").await;

    let rocket = rocket::custom(figment)
        .mount(
            "/",
            openapi_get_routes![
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
                ab_settings
            ],
        )
        .mount(
            "/api/doc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("SCTGDesk API Doc".to_owned()),
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

/// Login
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
        user: UserInfo { name: user },
        access_token,
    };

    log::debug!("login: {:?}", request);

    state.check_maintenance().await;

    Ok(Json(reply))
}

/// Get the user's legacy address book
#[openapi(tag = "address book legacy")]
#[get("/api/ab", format = "application/json")]
async fn ab_get(
    state: &State<ApiState>,
    user: AuthenticatedUser,
) -> Result<Json<AbGetResponse>, status::Unauthorized<()>> {
    ab_get_handler(state, user).await
}

/// Get the user's address book
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

    let ab = AddressBook { ab };

    let _ = unwrap_or_return!(state
        .set_user_address_book(user.info.user_id, ab)
        .await
        .ok_or(Err(status::Unauthorized::<()>(()))));

    state.check_maintenance().await;

    Ok(())
}

/// Get the current user
#[openapi(tag = "User (todo)")]
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
        data: UserInfo { name: username },
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

/// Log the user out
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

/// Heartbeat
///
/// Frequently the client hits the /api/heartbeat endpoint.
/// It updates the last_online field of the peer.
#[openapi(tag = "peer")]
#[post("/api/heartbeat", format = "application/json", data = "<request>")]
async fn heartbeat(state: &State<ApiState>, request: Json<HeartbeatRequest>) -> String {
    log::debug!("heartbeat: {:?}", request);
    let heartbeat = request.0;
    let res = state.update_heartbeat(heartbeat).await;
    log::debug!("res: {:?}", res);
    "OK".to_string()
    //{"error":"License is not set"}
}

/// Set the system info
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

/// Get the list of users
#[openapi(tag = "User (todo)")]
#[get("/api/user-list?<current>&<pageSize>&<email>&<name>", format = "application/json")]
async fn users(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
    current: u32,
    pageSize: u32,
    email: Option<&str>,
    name: Option<&str>,
) -> Result<Json<UserList>, status::NotFound<()>> {
    log::debug!("users");
    state.check_maintenance().await;

    let res = state.get_all_users(name,email,current, pageSize).await;
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

/// Get the list of groups
#[openapi(tag = "todo")]
#[get("/api/groups?<current>&<pageSize>", format = "application/json")]
async fn groups(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
    current: u32,
    pageSize: u32,
) -> Result<Json<UsersResponse>, status::NotFound<()>> {
    log::debug!("users");
    state.check_maintenance().await;

    let response = UsersResponse {
        msg: "success".to_string(),
        total: 1,
        data: "[{}]".to_string(),
    };

    Ok(Json(response))
}

/// Add a group
#[openapi(tag = "todo")]
#[post("/api/group", format = "application/json", data = "<request>")]
async fn group_add(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
    request: Json<AddUserRequest>,
) -> Result<Json<UsersResponse>, status::Unauthorized<()>> {
    log::debug!("create_user");
    state.check_maintenance().await;

    let response = UsersResponse {
        msg: "success".to_string(),
        total: 1,
        data: "[{}]".to_string(),
    };

    Ok(Json(response))
}

/// Get the list of peers
#[openapi(tag = "todo")]
#[get("/api/peers", format = "application/json")]
async fn peers(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
) -> Result<Json<UsersResponse>, status::NotFound<()>> {
    log::debug!("peers");
    state.check_maintenance().await;

    let response = UsersResponse {
        msg: "success".to_string(),
        total: 1,
        data: "[{\"id\":\"test\",\"info\":\"{\\\"username\\\":\\\"\\\",\\\"os\\\":\\\"\\\",\\\"device_name\\\":\\\"\\\"}\",\"user\":\"ff\",\"user_name\":\"Occupancy\",\"node\":\"tt\",\"is_admin\":true}]".to_string(),
    };

    Ok(Json(response))
}

/// Login options
///
/// This is called by the client for knowing the Oauth2 provider(s) available
/// You must provide a list of Oauth2 providers in the `oauth2.toml` config file
/// The config file can be overridden by the `OAUTH2_CONFIG_FILE` environment variable
///
/// # Limitations
///
/// Currently it uses the client id as the user id the limitation is that the client cannot retrieve its address book
/// if the client uses a different client.  
/// For having a `real` user name. We need to add a step after the Oauth2 authorization code is exchanged for an access token.
#[openapi(tag = "login")]
#[get("/api/login-options", format = "application/json")]
async fn login_options(
    state: &State<ApiState>,
) -> Result<Json<Vec<String>>, status::Unauthorized<()>> {
    let mut providers: Vec<String> = Vec::new();
    let providers_config = state
        .get_oauth2_config(oauth2::get_provider_config_file().as_str())
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
    let uuid_client =
        String::from_utf8(BASE64_STANDARD.decode(request.uuid.clone()).unwrap()).unwrap();
    let callback_url = format!("{}/api/oidc/callback", get_host(headers.clone()));
    let providers_config = state
        .get_oauth2_config(oauth2::get_provider_config_file().as_str())
        .await;
    if providers_config.is_none() {
        return Json(OidcAuthUrl {
            url: "".to_string(),
            code: "".to_string(),
        });
    }
    let providers_config = providers_config.unwrap();
    let provider = providers_config
        .iter()
        .find(|config| config.op == request.op);

    if provider.is_none() {
        return Json(OidcAuthUrl {
            url: "".to_string(),
            code: "".to_string(),
        });
    }
    let provider = provider.unwrap();
    let redirect_url =
        oauth2::get_redirect_url(provider, callback_url.as_str(), uuid_code.as_str());
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
                provider_config: Some(provider.clone()),
            },
        )
        .await;
    log::debug!("uuid_code: {:?}", uuid_code);

    Json(OidcAuthUrl {
        url: redirect_url.clone(),
        code: uuid_code,
    })
}

/// OIDC Auth callback
///
/// This entrypoint is the OAuth2 callback.
/// It exchanges the code for an access token and stores it in the state
#[openapi(tag = "login")]
#[get("/api/oidc/callback?<code>&<state>")]
async fn oidc_callback(apistate: &State<ApiState>, code: &str, state: &str) -> String {
    let oidc_code = state; // thius the session code
    let oidc_authorization_code = code;
    let updated_oidc_session = apistate
        .oidc_session_exchange_code(oidc_authorization_code.to_string(), oidc_code.to_string())
        .await;
    if updated_oidc_session.is_none() {
        return "ERROR".to_string();
    }
    "OK".to_string()
}

/// OIDC State request
///
/// This entrypoint is called by the client for getting the status of the OIDC session
/// it returns an empty json object if the session is not found
/// it returns an access token if the session is found
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

/// Address book
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

/// Get the tags
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

/// Add a tag
#[openapi(tag = "address book")]
#[post(
    "/api/ab/tag/add/<ab>",
    format = "application/json",
    data = "<request>"
)]
async fn ab_tag_add(
    state: &State<ApiState>,
    user: AuthenticatedUser,
    ab: &str,
    request: Json<AbTag>,
) -> Result<ActionResponse, status::Unauthorized<()>> {
    state.check_maintenance().await;
    let ab_tag = request.0;
    log::debug!("ab_tag_add: {:?}", ab_tag);
    state.add_ab_tag(ab, ab_tag).await;
    Ok(ActionResponse::Empty)
}

/// Update a tag
#[openapi(tag = "address book")]
#[put(
    "/api/ab/tag/update/<ab>",
    format = "application/json",
    data = "<request>"
)]
async fn ab_tag_update(
    state: &State<ApiState>,
    user: AuthenticatedUser,
    ab: &str,
    request: Json<AbTag>,
) -> Result<ActionResponse, status::Unauthorized<()>> {
    state.check_maintenance().await;
    let ab_tag = request.0;
    log::debug!("ab_tag_update: {:?}", ab_tag);
    state.add_ab_tag(ab, ab_tag).await;
    Ok(ActionResponse::Empty)
}

/// Rename a tag
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

/// Delete a tag
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

/// Shared profile
#[openapi(tag = "address book")]
#[post("/api/ab/shared/profiles")]
async fn ab_shared(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
) -> Result<Json<AbSharedProfilesResponse>, status::Unauthorized<()>> {
    state.check_maintenance().await;
    let ab_shared_profiles = AbSharedProfilesResponse::default();
    Ok(Json(ab_shared_profiles))
}

/// Settings
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

/// List peers
///
#[openapi(tag = "address book")]
#[post("/api/ab/peers?<current>&<pageSize>&<ab>")]
async fn ab_peers(
    state: &State<ApiState>,
    _user: AuthenticatedUser,
    current: i32,
    pageSize: i32,
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

/// Add peer
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

/// Update peer
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

/// Delete peer
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

/// List strategies
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

/// Add user
#[openapi(tag = "User (todo)")]
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

/// Enable users
#[openapi(tag = "User (todo)")]
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
    for user in enable_users.rows {
        let res = state.user_change_status(user.as_str(), enable_users.disable).await;
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

/// Update current user password
#[openapi(tag = "User (todo)")]
#[put("/api/user", format = "application/json", data = "<request>")]
async fn user_update(
    state: &State<ApiState>,
    user: AuthenticatedUser,
    request: Json<UpdateUserRequest>,
) -> Result<Json<UsersResponse>, status::Unauthorized<()>> {
    log::debug!("update_user");
    state.check_maintenance().await;
    let response = UsersResponse {
        msg: "success".to_string(),
        total: 1,
        data: "[{}]".to_string(),
    };
    let user_update = request.0;
    state.user_update(user.info.user_id, user_update).await;
    Ok(Json(response))
}
/// Add OIDC Provider
#[openapi(tag = "todo")]
#[put("/api/oidc/settings", format = "application/json", data = "<request>")]
async fn oidc_add(
    state: &State<ApiState>,
    _user: AuthenticatedAdmin,
    request: Json<EnableUserRequest>,
) -> Result<Json<EnableUserRequest>, status::Unauthorized<()>> {
    log::debug!("Add OIDC Provider");
    state.check_maintenance().await;

    Err(status::Unauthorized::<()>(()))
}

/// Get OIDC Providers
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
