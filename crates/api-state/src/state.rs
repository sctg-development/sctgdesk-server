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
use crate::database::DatabaseUserInfo;
use crate::types;
use crate::{
    bearer::AuthenticatedUserInfo, database::Database, password::UserPasswordInfo, SessionId,
    UserId,
};
use std::{
    collections::HashMap,
    default::Default,
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
    time::SystemTime,
};

use oauth2::ProviderConfig;

use tokio::sync::RwLock;
use utils::{
    AbPeer, AbRule, AbTag, AddUserRequest, AddressBook, CpuCount, Group, OidcState, Peer, Platform,
    Token, UpdateUserRequest, UserListResponse,
};

pub struct ApiState {
    last_maintenance_time: AtomicU64,
    access_tokens: RwLock<HashMap<Token, AccessTokenInfo>>,
    sessions: RwLock<SessionsState>,
    users: RwLock<HashMap<UserId, UserInfo>>,
    address_books: RwLock<HashMap<UserId, AddressBookInfo>>,
    oidc_sessions: RwLock<HashMap<String, OidcState>>,
    db: Database,
    oauth2_providers: RwLock<Vec<ProviderConfig>>,
}

#[derive(Debug, Clone)]
pub struct AccessTokenInfo {
    pub session_id: SessionId,
    pub user_id: UserId,
}

#[derive(Debug, Default)]
struct SessionsState {
    counter: SessionId,
    sessions: HashMap<SessionId, SessionInfo>,
}

#[derive(Debug, Default)]
pub struct UserInfo {
    sessions_count: usize,
    pub username: String,
    pub admin: bool,
}

#[derive(Debug, Default)]
struct SessionInfo {
    #[allow(dead_code)]
    user_id: UserId,
}

#[derive(Debug, Clone)]
pub struct AddressBookInfo {
    modified: bool,
    remove_after_flush: bool,
    pub address_book: AddressBook,
}

const MAINTENANCE_INTERVAL_IN_SECS: u64 = 60;

fn secs_from_epoch() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

impl ApiState {
    pub async fn new_with_db<P: AsRef<Path>>(db_filename: P) -> Self {
        let db = Database::open(db_filename).await;
        Self {
            last_maintenance_time: AtomicU64::new(0),
            access_tokens: Default::default(),
            sessions: Default::default(),
            users: Default::default(),
            address_books: Default::default(),
            db,
            oidc_sessions: Default::default(),
            oauth2_providers: Default::default(),
        }
    }

    pub async fn maintenance_flush_address_books(&self) {
        let mut state_address_books = self.address_books.write().await;

        let mut values: Vec<(UserId, AddressBook)> = vec![];

        for (user_id, address_book_info) in state_address_books.iter_mut() {
            if !address_book_info.modified {
                continue;
            }
            let user_id_clone = user_id.clone();
            values.push((user_id_clone, address_book_info.address_book.clone()));
            address_book_info.modified = false;
        }

        if !values.is_empty() {
            log::debug!("Need to update_address_books");
            self.db.update_legacy_address_books(values).await;
        }
    }

    pub async fn maintenance(&self) {
        self.maintenance_flush_address_books().await;
    }

    pub async fn check_maintenance(&self) {
        log::debug!("check_maintenance...");

        let now = secs_from_epoch();
        let last_mt = self.last_maintenance_time.load(Ordering::Relaxed);

        if now >= (last_mt + MAINTENANCE_INTERVAL_IN_SECS) {
            self.last_maintenance_time.store(now, Ordering::Relaxed);
            log::debug!("check_maintenance NOW");
            self.maintenance().await;
        }
    }

    pub async fn user_login<'s>(
        &self,
        username: &String,
        password_info: UserPasswordInfo<'s>,
        admin_only: bool,
    ) -> Option<(utils::UserInfo, Token)> {
        let (conn, user_id, email, db_user_info) =
            match self.db.find_user_by_name(username.as_str()).await {
                (conn, Some((user_id, email, db_user_info))) => {
                    (conn, user_id, email, db_user_info)
                }
                _ => return None,
            };
        if !db_user_info.active {
            return None;
        }

        if admin_only {
            if !db_user_info.admin {
                return None;
            }
        }

        let (conn, db_password_info) = match self
            .db
            .get_user_hashed_password(conn, user_id.clone())
            .await
        {
            (conn, Some(db_password_info)) => (conn, db_password_info),
            _ => return None,
        };

        if !password_info.check(db_password_info) {
            return None;
        }

        drop(conn);

        let access_token = self
            .get_access_token(user_id, username, db_user_info.admin)
            .await;

        Some((
            utils::UserInfo {
                name: username.to_string(),
                email,
                admin: db_user_info.admin,
                ..Default::default()
            },
            access_token,
        ))
    }

    async fn get_access_token(&self, user_id: Vec<u8>, username: &String, is_admin: bool) -> Token {
        let access_token = Token::new_random();

        let mut state_access_tokens = self.access_tokens.write().await;
        let mut state_sessions = self.sessions.write().await;
        let mut state_users = self.users.write().await;

        state_sessions.counter += 1;
        let session_id = state_sessions.counter;

        if let Some(user_info) = state_users.get_mut(&user_id) {
            user_info.sessions_count += 1;
        } else {
            let user_info = UserInfo {
                sessions_count: 1,
                username: username.clone(),
                admin: is_admin,
            };
            state_users.insert(user_id.clone(), user_info);

            let mut state_address_books = self.address_books.write().await;
            if let Some(abi) = state_address_books.get_mut(&user_id) {
                abi.remove_after_flush = false;
            }
        }

        let session_info = SessionInfo {
            user_id: user_id.clone(),
        };

        let access_token_info = AccessTokenInfo {
            session_id,
            user_id,
        };

        let _ = state_sessions.sessions.insert(session_id, session_info);
        let _ = state_access_tokens.insert(access_token.clone(), access_token_info);
        access_token
    }

    pub async fn find_session(&self, access_token: &Token) -> Option<AccessTokenInfo> {
        let state_access_tokens = self.access_tokens.read().await;

        state_access_tokens.get(access_token).map(|t| t.clone())
    }

    pub async fn get_user_address_book(&self, user_id: UserId) -> Option<AddressBook> {
        let state_address_books = self.address_books.read().await;

        let opt_ab = state_address_books
            .get(&user_id)
            .map(|abi| abi.address_book.clone());

        if opt_ab.is_some() {
            return opt_ab;
        }

        drop(state_address_books);

        let ab = self.db.get_legacy_address_book(user_id.clone()).await?;
        let abi = AddressBookInfo {
            modified: false,
            remove_after_flush: false,
            address_book: ab.clone(),
        };

        let mut state_address_books = self.address_books.write().await;
        state_address_books.insert(user_id, abi.clone());

        Some(ab)
    }

    pub async fn set_user_address_book(
        &self,
        user_id: UserId,
        address_book: AddressBook,
    ) -> Option<()> {
        log::debug!("set_user_ab()");
        let mut state_address_books = self.address_books.write().await;

        if let Some(abi) = state_address_books.get_mut(&user_id) {
            if abi.address_book != address_book {
                abi.modified = true;
                abi.address_book = address_book;
            };
        } else {
            let abi = AddressBookInfo {
                modified: false,
                remove_after_flush: false,
                address_book,
            };
            state_address_books.insert(user_id, abi);
        }
        // log::debug!("set_user_ab() 2");

        // let _ = self.db.update_ab( user_id, &abi.ab ).await;

        log::debug!("ab done!");
        Some(())
    }

    /// Log out the given user from the state.
    ///
    /// This function is used to log out a user when the user's session is
    /// invalidated (e.g. when the user changes their password).
    ///
    /// This function removes the user's access token and session from the
    /// state, and decrements the number of sessions for the user. If the number
    /// of sessions for the user reaches 0, the user is removed from the state
    /// entirely.
    ///
    /// # Returns
    ///
    /// This function returns `None` if the user is not present in the state,
    /// or if removing their session and access token from the state fails.
    /// If the function returns `Some(())`, the logout was successful.
    pub async fn user_logout(&self, user: &AuthenticatedUserInfo) -> Option<()> {
        let mut state_access_tokens = self.access_tokens.write().await;
        let mut state_sessions = self.sessions.write().await;
        let mut state_users = self.users.write().await;

        let user_info = state_users.get_mut(&user.user_id)?;
        user_info.sessions_count -= 1;

        if user_info.sessions_count == 0 {
            state_users.remove(&user.user_id);

            let mut state_address_books = self.address_books.write().await;
            if let Some(abi) = state_address_books.get_mut(&user.user_id) {
                abi.remove_after_flush = true;
            }
        }

        state_sessions.sessions.remove(&user.session_id);
        state_access_tokens.remove(&user.access_token);

        Some(())
    }

    pub async fn get_current_user_name(&self, user: &AuthenticatedUserInfo) -> Option<String> {
        let state_users = self.users.read().await;
        state_users.get(&user.user_id).map(|ui| ui.username.clone())
    }

    pub async fn is_current_user_admin(&self, user: &AuthenticatedUserInfo) -> Option<bool> {
        let state_users = self.users.read().await;
        state_users.get(&user.user_id).map(|ui| ui.admin)
    }

    pub async fn with_user_info<R>(
        &self,
        user_id: &UserId,
        mut f: impl FnMut(&UserInfo) -> R,
    ) -> Option<R> {
        let state_users = self.users.read().await;
        if let Some(user_info) = state_users.get(user_id) {
            Some(f(user_info))
        } else {
            None
        }
    }

    pub async fn ui_get_all_users(&self) -> Option<Vec<types::UserInfo>> {
        self.db.ui_get_all_users().await
    }

    pub async fn ui_update_user_password(
        &self,
        username: String,
        old_password: String,
        new_password: String,
    ) -> Option<()> {
        self.db
            .update_user_password(username, old_password, new_password)
            .await
    }

    pub async fn ui_reset_user_password(
        &self,
        username: String,
        new_password: String,
    ) -> Option<()> {
        self.db.reset_user_password(username, new_password).await
    }

    pub async fn ui_create_user(
        &self,
        username: String,
        password: String,
        admin: bool,
    ) -> Option<UserId> {
        self.db.create_user(username, password, admin).await
    }

    pub async fn user_delete(&self, user_id: &str) -> Option<()> {
        self.db.delete_user(user_id).await
    }

    pub async fn ui_get_user_info(&self, username: String) -> Option<types::UserInfo> {
        let res = self.db.ui_get_user_info(username).await;
        res
    }

    pub async fn update_systeminfo(&self, systeminfo: utils::SystemInfo) -> Option<()> {
        // must be written in the database immediately because peer is mainly used by hbbs
        self.db.update_systeminfo(systeminfo).await
    }

    pub async fn update_heartbeat(&self, heartbeat: utils::HeartbeatRequest) -> Option<()> {
        self.db.update_heartbeat(heartbeat).await
    }

    pub async fn get_oauth2_config(&self, config_file: &str) -> Option<Vec<ProviderConfig>> {
        let mut oauth2_providers = self.oauth2_providers.write().await;

        if oauth2_providers.is_empty() {
            log::debug!("get providers from {}", config_file);
            let config = oauth2::get_providers_config_from_file(config_file);
            if config.len() == 0 {
                return None;
            }
            for c in config.clone() {
                oauth2_providers.push(c);
            }
        }
        Some(oauth2_providers.clone())
    }

    pub async fn insert_oidc_session(
        &self,
        uuid_code: String,
        oidc_state: OidcState,
    ) -> Option<OidcState> {
        let mut oidc_sessions = self.oidc_sessions.write().await;
        let old_value = oidc_sessions.insert(uuid_code, oidc_state.clone());
        if old_value.is_none() {
            return Some(oidc_state);
        }
        None
    }

    pub async fn get_oidc_session(&self, uuid_code: String) -> Option<OidcState> {
        let oidc_sessions = self.oidc_sessions.read().await;
        oidc_sessions.get(&uuid_code).map(|s| s.clone())
    }

    /// Exchange code for tokens
    ///
    /// This function exchanges a code obtained from an oauth2 provider
    /// for an access token.
    /// If the oauth2 provider supports it, also an id token is exchanged.
    ///
    /// # Arguments
    ///
    /// * `authorization_code` - The code obtained from the oauth2 provider
    /// * `uuid_code` - The unique code to identify this code exchange request
    ///
    /// # Returns
    ///
    /// If the code exchange was successful, an `Option` containing the access
    /// and refresh tokens is returned, otherwise `None`.
    pub async fn oidc_session_exchange_code(
        &self,
        authorization_code: String,
        uuid_code: String,
    ) -> Option<String> {
        let mut oidc_sessions = self.oidc_sessions.write().await;
        let oidc_session = oidc_sessions.get_mut(&uuid_code);
        if oidc_session.is_none() {
            return None;
        }
        let oidc_session = oidc_session.unwrap();
        oidc_session.code = Some(authorization_code.clone());
        if oidc_session.provider.is_some()
            && oidc_session.code.is_some()
            && oidc_session.callback_url.is_some()
        {
            let provider = oidc_session.clone().provider.unwrap();
            let callback_url = oidc_session.clone().callback_url.unwrap();
            let exchange_result = provider
                .exchange_code(authorization_code.as_str(), callback_url.as_str())
                .await;

            if exchange_result.is_ok() {
                let access_token = exchange_result.unwrap();
                let username = access_token.username.clone();

                oidc_session.auth_token = Some(access_token.access_token.clone());
                oidc_session.name = Some(if username.len() > 0 {
                    username.clone()
                } else {
                    oidc_session.id.clone()
                });
                oidc_session.email = Some(access_token.email.clone());
                log::debug!("oidc_session_exchange_code {:?}", oidc_session.auth_token);
                return Some(access_token.access_token);
            }
        }
        None
    }

    /// Check if the client uuid has completed the authorization flow
    /// If yes drop the session and provide the access token
    /// If not return None
    ///
    /// # Arguments
    ///
    /// * `uuid_code` - The uuid code of the client
    ///
    /// # Returns
    ///
    /// * `Option<(Token,String,DatabaseUserInfo)>`
    /// - The access token
    /// - The username
    /// - The database user info
    pub async fn oidc_check_session(
        &self,
        uuid_code: String,
    ) -> Option<(Token, String, DatabaseUserInfo)> {
        let mut oidc_sessions = self.oidc_sessions.write().await;
        let oidc_session = oidc_sessions.get_mut(&uuid_code);
        if oidc_session.is_none() {
            return None;
        }
        let oidc_session = oidc_session.unwrap();
        let name = if let Some(name) = oidc_session.name.clone() {
            name
        } else {
            oidc_session.id.clone()
        };
        let email = if let Some(email) = oidc_session.email.clone() {
            email
        } else {
            "tobefilled@example;org".to_string()
        };
        if oidc_session.auth_token.is_some() {
            let res = self
                .db
                .get_user_for_oauth2(name, email, oidc_session.uuid.clone())
                .await;
            if res.is_none() {
                log::debug!("oidc_check_session user not found");
                return None;
            }
            let (uuid_vec, username, db_user_info) = res.unwrap();
            if !db_user_info.active {
                log::debug!("oidc_check_session user not active");
                return None;
            }
            let token = self
                .get_access_token(uuid_vec, &username, db_user_info.admin)
                .await;
            // User has completed the authorization flow
            oidc_sessions.remove(&uuid_code);
            return Some((token, username, db_user_info));
        }
        None
    }

    /// Get the users's personal address book guid
    pub async fn get_ab_personal_guid(&self, user_id: UserId) -> Option<String> {
        self.db.get_ab_personal_guid(user_id).await
    }

    /// Add a peer to an address book
    pub async fn add_ab_peer(&self, ab: &str, ab_peer: AbPeer) -> Option<()> {
        self.db.add_peer_to_ab(ab, ab_peer).await
    }

    /// Get all peers from an address book
    pub async fn get_ab_peers(&self, ab: &str) -> Option<Vec<AbPeer>> {
        self.db.get_peers_from_ab(ab).await
    }

    /// Delete a peer in an address book
    pub async fn delete_ab_peer(&self, ab: &str, peers_to_delete: Vec<String>) -> Option<()> {
        for peer in peers_to_delete {
            self.db.delete_peer_from_ab(ab, peer.as_str()).await;
        }
        Some(())
    }

    /// Get a peer from an address book
    pub async fn get_ab_peer(&self, ab: &str, peer: &str) -> Option<AbPeer> {
        self.db.get_ab_peer(ab, peer).await
    }

    /// Add a tag to an address book
    pub async fn add_ab_tag(&self, ab: &str, tag: AbTag) -> Option<()> {
        self.db.add_tag_to_ab(ab, tag).await
    }

    /// Get all tags from an address book
    pub async fn get_ab_tags(&self, ab: &str) -> Option<Vec<AbTag>> {
        self.db.get_ab_tags(ab).await
    }

    /// Get a tag from an address book
    pub async fn get_ab_tag(&self, ab: &str, tag: &str) -> Option<AbTag> {
        let ab_tag = self.db.get_ab_tag(ab, tag).await;
        if ab_tag.is_none() {
            return None;
        }
        let ab_tag = ab_tag.unwrap();
        Some(ab_tag)
    }

    /// Rename a tag in an address book
    pub async fn rename_ab_tag(&self, ab: &str, old_name: &str, tag: AbTag) -> Option<()> {
        self.db.rename_ab_tag(ab, old_name, tag).await
    }

    /// Delete some tags from an address book
    pub async fn delete_ab_tags(&self, ab: &str, tags_to_delete: Vec<String>) -> Option<()> {
        for tag in tags_to_delete {
            self.db.delete_tag_from_ab(ab, tag.as_str()).await;
        }
        Some(())
    }

    /// Add a user
    /// This function is used to add a user to the database
    pub async fn add_user(&self, user_parameters: AddUserRequest) -> Option<()> {
        self.db
            .add_user(
                user_parameters.name,
                user_parameters.password,
                user_parameters.email,
                user_parameters.is_admin,
                user_parameters.group_name,
            )
            .await
    }

    /// Change user status
    pub async fn user_change_status(&self, user: &str, disable: bool) -> Option<()> {
        self.db.user_change_status(user, disable as u32).await
    }

    /// Get all users
    pub async fn get_all_users(
        &self,
        name: Option<&str>,
        email: Option<&str>,
        current: u32,
        page_size: u32,
    ) -> Option<Vec<UserListResponse>> {
        self.db.get_all_users(name, email, current, page_size).await
    }

    /// Update a user
    pub async fn user_update(
        &self,
        user_id: UserId,
        user_parameters: UpdateUserRequest,
    ) -> Option<()> {
        self.db.user_update(user_id, user_parameters).await
    }

    /// Get all peers
    pub async fn get_all_peers(&self) -> Option<Vec<Peer>> {
        self.db.get_all_peers().await
    }

    /// Get groups
    pub async fn get_groups(&self, offset: u32, page_size: u32) -> Option<Vec<Group>> {
        self.db.get_groups(offset, page_size).await
    }

    /// Get shared address books
    pub async fn get_shared_address_books(&self, user_id: UserId) -> Option<Vec<AddressBook>> {
        self.db.get_shared_address_books(user_id).await
    }

    pub async fn get_ab_rules(&self, offset: u32, page_size: u32, ab: &str) -> Option<Vec<AbRule>> {
        self.db.get_ab_rules(offset, page_size, ab).await
    }

    pub async fn delete_ab_rule(&self, rule: &str) -> Option<()> {
        self.db.delete_ab_rule(rule).await
    }

    pub async fn add_ab_rule(&self, rule: AbRule) -> Option<()> {
        self.db.add_ab_rule(rule).await
    }

    pub async fn get_peers_count(&self, platform: Platform) -> u32 {
        self.db.get_peers_count(platform).await
    }

    pub async fn get_cpus_count(&self) -> Vec<CpuCount> {
        self.db.get_cpus_count().await
    }

    pub async fn create_group(&self, name: &str, team: &str, note: &str) -> Option<()> {
        self.db.create_group(name, team, note).await
    }

    pub async fn update_group(&self, guid: &str, name: &str, team: &str, note: &str) -> Option<()> {
        self.db.update_group(guid, name, team, note).await
    }

    pub async fn get_group(&self, guid: &str) -> Option<Group> {
        self.db.get_group(guid).await
    }

    pub async fn delete_group(&self, guid: &str) -> Option<()> {
        self.db.delete_group(guid).await
    }

    /// Add a shared address book given its name and its owner
    /// It returns the guid of the shared address book
    ///
    /// # Arguments
    ///
    /// - `name` - The name of the shared address book
    ///
    /// - `owner` - The owner of the shared address book
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The guid of the shared address book
    pub async fn add_shared_address_book(&self, name: &str, owner: &str) -> Option<String> {
        self.db.add_shared_address_book(name, owner).await
    }

    pub async fn delete_shared_address_book(&self, guid: &str) -> Option<()> {
        self.db.delete_shared_address_book(guid).await
    }

    pub async fn delete_shared_address_books(&self, shareds:Vec<String>) -> Option<()> {
        for shared in shareds {
            self.db.delete_shared_address_book(shared.as_str()).await;
        }
        Some(())
    }
    pub async fn update_shared_address_book(&self, guid: &str, name: &str) -> Option<()> {
        self.db.update_shared_address_book(guid, name).await
    }
}
