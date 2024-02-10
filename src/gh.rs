#![allow(dead_code)]

use lazy_static::lazy_static;
use reqwest::{self, header::HeaderMap};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::utils;

struct DefaultLoginHeaders {
    accept: &'static str,
    user_agent: &'static str,
    editor_version: &'static str,
    editor_plugin_version: &'static str,
    user_agent_version: &'static str,
}

impl DefaultLoginHeaders {
    pub fn to_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            reqwest::header::HeaderValue::from_static(self.accept),
        );
        headers.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_static(self.user_agent),
        );
        headers.insert(
            "X-Editor-Version",
            reqwest::header::HeaderValue::from_static(self.editor_version),
        );
        headers.insert(
            "X-Editor-Plugin-Version",
            reqwest::header::HeaderValue::from_static(self.editor_plugin_version),
        );
        headers.insert(
            "X-User-Agent-Version",
            reqwest::header::HeaderValue::from_static(self.user_agent_version),
        );
        headers
    }
}

struct DefaultGithubUserHeaders {
    authorization: String,
    user_agent: &'static str,
    accept: &'static str,
}

impl DefaultGithubUserHeaders {
    pub fn to_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&self.authorization).unwrap(),
        );
        headers.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_static(self.user_agent),
        );
        headers.insert(
            "Accept",
            reqwest::header::HeaderValue::from_static(self.accept),
        );
        headers
    }
}

fn get_default_user_headers(token_type: &String, token: &String) -> DefaultGithubUserHeaders {
    DefaultGithubUserHeaders {
        authorization: format!("{} {}", token_type, token),
        user_agent: "GithubCopilot/1.133.0",
        accept: "application/json",
    }
}

struct DefaultGithubInternalHeaders {
    authorization: String,
    user_agent: &'static str,
    editor_version: &'static str,
    editor_plugin_version: &'static str,
}

impl DefaultGithubInternalHeaders {
    pub fn to_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&self.authorization).unwrap(),
        );
        headers.insert(
            "user-agent",
            reqwest::header::HeaderValue::from_static(self.user_agent),
        );
        headers.insert(
            "editor-version",
            reqwest::header::HeaderValue::from_static(self.editor_version),
        );
        headers.insert(
            "editor-plugin-version",
            reqwest::header::HeaderValue::from_static(self.editor_plugin_version),
        );
        headers
    }
}

fn get_default_internal_headers(
    _token_type: &String,
    token: &String,
) -> DefaultGithubInternalHeaders {
    DefaultGithubInternalHeaders {
        authorization: format!("token {}", token),
        editor_version: "vscode/1.85.1",
        editor_plugin_version: "copilot-chat/0.12.2023120701",
        user_agent: "GitHubCopilotChat/0.12.2023120701",
    }
}

lazy_static! {
    static ref DEFAULT_LOGIN_HEADERS: DefaultLoginHeaders = DefaultLoginHeaders {
        accept: "application/json",
        user_agent: "GithubCopilot/1.133.0",
        editor_version: "Neovim/0.9.2",
        editor_plugin_version: "copilot.lua/1.11.4",
        user_agent_version: "GithubCopilot/1.133.0",
    };
    static ref DEVICE_CODE_LOGIN_URL: &'static str = "https://github.com/login/device/code";
    static ref DEVICE_CODE_TOKEN_CHECK_URL: &'static str =
        "https://github.com/login/oauth/access_token";
    static ref GH_AUTH_TOKEN_URL: &'static str = "https://api.github.com/user";
    static ref GH_COPILOT_INTERNAL_AUTH_URL: &'static str =
        "https://api.github.com/copilot_internal/v2/token";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubDeviceLoginResponse {
    interval: u64,
    user_code: String,
    expires_in: u64,
    verification_uri: String,
    device_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubDeviceTokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GithubUserData {
    login: String,
    id: u64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_: String,
    site_admin: bool,
    name: String,
    company: Option<String>,
    blog: String,
    location: String,
    email: Option<String>,
    hireable: Option<bool>,
    bio: String,
    twitter_username: Option<String>,
    public_repos: u64,
    public_gists: u64,
    followers: u64,
    following: u64,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubCopilotAuth {
    annotations_enabled: bool,
    chat_enabled: bool,
    chat_jetbrains_enabled: bool,
    code_quote_enabled: bool,
    copilot_ide_agent_chat_gpt4_small_prompt: bool,
    copilotignore_enabled: bool,
    expires_at: u64,
    intellij_editor_fetcher: bool,
    prompt_8k: bool,
    public_suggestions: String,
    refresh_in: u64,
    sku: String,
    snippy_load_test_enabled: bool,
    telemetry: String,
    pub token: String,
    tracking_id: String,
    vsc_electron_fetcher: bool,
    vsc_panel_v2: bool,
}

#[derive(Debug)]
pub struct GithubAuth {
    pub user: GithubUserData,
    pub token: GitHubDeviceTokenResponse,
    pub copilot_auth: GithubCopilotAuth,
}

/// A struct that represents the authentication manager for Github Copilot
pub struct AuthenticationManager {}

impl AuthenticationManager {
    pub fn new() -> Self {
        AuthenticationManager {}
    }

    /// `request_github_auth` is an asynchronous function that requests GitHub authentication.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` which is `Ok` if the authentication request is successful,
    /// containing a `GitHubDeviceLoginResponse`. If the request fails, it returns an `Err` with a description of the error.
    ///
    /// # Example
    ///
    /// ```
    /// match auth_manager.request_github_auth().await {
    ///     Ok(response) => println!("Authentication request successful: {:?}", response),
    ///     Err(e) => println!("Authentication request failed: {}", e),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the authentication request fails.
    pub async fn request_github_auth(&self) -> Result<GitHubDeviceLoginResponse, String> {
        let headers = DEFAULT_LOGIN_HEADERS.to_headers();

        let req = reqwest::Client::new()
            .post(*DEVICE_CODE_LOGIN_URL)
            .json(&serde_json::json!({
                "client_id": "Iv1.b507a08c87ecfe98",
                "scope": "read:user"
            }))
            .headers(headers)
            .send()
            .await
            .unwrap();

        let json = req.json::<GitHubDeviceLoginResponse>().await.unwrap();
        Ok(json)
    }

    /// `check_github_auth` is an asynchronous function that checks the GitHub authentication status.
    ///
    /// # Arguments
    ///
    /// * `device_code` - A reference to a string that holds the device code for GitHub authentication.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` which is `Ok` if the authentication is successful,
    /// containing a `GitHubDeviceTokenResponse`. If the authentication is still pending,
    /// it returns an `Err`.
    ///
    /// # Example
    ///
    /// ```
    /// let device_code = String::from("your_device_code");
    /// match auth_manager.check_github_auth(&device_code).await {
    ///     Ok(response) => println!("Authentication successful: {:?}", response),
    ///     Err(_) => println!("Authentication is still pending"),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the authentication is still pending.
    pub async fn check_github_auth(
        &self,
        device_code: &String,
    ) -> Result<GitHubDeviceTokenResponse, ()> {
        let headers = DEFAULT_LOGIN_HEADERS.to_headers();

        let req = reqwest::Client::new()
            .post(*DEVICE_CODE_TOKEN_CHECK_URL)
            .json(&serde_json::json!({
                "client_id": "Iv1.b507a08c87ecfe98",
                "device_code": device_code,
                "grant_type": "urn:ietf:params:oauth:grant-type:device_code"
            }))
            .headers(headers)
            .send()
            .await
            .unwrap();

        // we have to use text here because there are two possible responses
        let text = req.text().await.unwrap();
        if text.contains("authorization_pending") {
            return Err(());
        }

        let json = serde_json::from_str::<GitHubDeviceTokenResponse>(&text).unwrap();

        Ok(json)
    }

    /// This asynchronous function is responsible for getting the user data from GitHub.
    ///
    /// # Arguments
    ///
    /// * `&self` - A reference to the instance of the struct in which this method is implemented.
    /// * `auth` - A reference to an instance of `GitHubDeviceTokenResponse` which contains the token type and access token.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` type. On successful execution, it returns `Ok(GithubUserData)`,
    /// where `GithubUserData` is the user data retrieved from GitHub. If there is an error during execution,
    /// it returns `Err(String)`, where `String` is the error message.
    ///
    /// # Example
    ///
    /// ```rust
    /// let user_data = gh_get_user(&auth).await;
    /// match user_data {
    ///     Ok(data) => println!("User data: {:?}", data),
    ///     Err(e) => println!("Error occurred: {}", e),
    /// }
    /// ```
    ///
    /// # Remarks
    ///
    /// This function uses the `reqwest` library to send a GET request to the GitHub API.
    /// The headers for the request are set using the `get_default_user_headers` function with the token type and access token from the `auth` argument.
    /// The function then sends the request and awaits the response.
    pub async fn gh_get_user(
        &self,
        auth: &GitHubDeviceTokenResponse,
    ) -> Result<GithubUserData, String> {
        let headers = get_default_user_headers(&auth.token_type, &auth.access_token);
        let headers = headers.to_headers();

        let req = reqwest::Client::new()
            .get(*GH_AUTH_TOKEN_URL)
            .headers(headers)
            .send()
            .await
            .unwrap();

        if req.status().is_success() {
            let json = req.json::<GithubUserData>().await.unwrap();
            Ok(json)
        } else {
            Err("Failed to authenticate with Github".to_string())
        }
    }

    pub async fn gh_copilot_authenticate(
        &self,
        auth: &GitHubDeviceTokenResponse,
    ) -> Result<GithubCopilotAuth, String> {
        let headers = get_default_internal_headers(&auth.token_type, &auth.access_token);
        let headers = headers.to_headers();

        let req = reqwest::Client::new()
            .get(*GH_COPILOT_INTERNAL_AUTH_URL)
            .headers(headers)
            .send()
            .await
            .unwrap();

        if req.status().is_success() {
            let json = req.json::<GithubCopilotAuth>().await.unwrap();
            return Ok(json);
        }

        Err("Failed to authenticate with Github Copilot".to_string())
    }

    /// `auth` is an asynchronous function that handles the entire authentication process with GitHub and GitHub Copilot.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` which is `Ok` if the authentication process is successful,
    /// containing a `GithubAuth` object. If the process fails at any point, it returns an `Err` with a description of the error.
    ///
    /// # Example
    ///
    /// ```
    /// match auth_manager.auth().await {
    ///     Ok(auth) => println!("Authentication successful: {:?}", auth),
    ///     Err(e) => println!("Authentication failed: {}", e),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the authentication request fails,
    /// if the check for GitHub authentication fails,
    /// or if the authentication with GitHub Copilot fails.
    pub async fn auth(&self) -> Result<GithubAuth, String> {
        let response = self.request_github_auth().await?;

        println!(
            "Please visit {} and enter the code {}",
            response.verification_uri, response.user_code
        );

        loop {
            let auth = self.check_github_auth(&response.device_code).await;
            match auth {
                Ok(auth) => {
                    let user = self.gh_get_user(&auth).await.unwrap();
                    let copilot = self.gh_copilot_authenticate(&auth).await.unwrap();
                    return Ok(GithubAuth {
                        user,
                        token: auth,
                        copilot_auth: copilot,
                    });
                }
                Err(_) => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(response.interval)).await;
                }
            }
        }
    }

    /// This asynchronous function is responsible for caching the GitHub authentication.
    ///
    /// # Functionality
    /// It first reads the configuration file to check if the token is already present.
    /// If the token is found, it proceeds to authenticate the user and the copilot with GitHub.
    /// It then returns a `GithubAuth` object which includes the user, the token, and the copilot authentication.
    ///
    /// # Returns
    /// This function returns a `Result` that contains a `GithubAuth` object on success, or a `String` error message on failure.
    ///
    /// # Errors
    /// This function will return an error if the GitHub authentication fails, or if there's an issue with reading the configuration file.
    ///
    /// # Example
    /// ```
    /// let auth_cache = cache_auth().await;
    /// match auth_cache {
    ///     Ok(auth) => println!("Authentication successful!"),
    ///     Err(e) => println!("Error during authentication: {}", e),
    /// }
    /// ```
    pub async fn cache_auth(&self) -> Result<GithubAuth, String> {
        // read the config file, and see if the token is already there
        // if it is, then we just need to do the copilot auth

        let gh_token = utils::read_config_file();

        if gh_token != "" {
            let auth = GitHubDeviceTokenResponse {
                access_token: gh_token,
                token_type: "bearer".to_string(),
                scope: "".to_string(),
            };

            let user = self.gh_get_user(&auth).await.unwrap();
            let copilot = self.gh_copilot_authenticate(&auth).await.unwrap();

            let auth = GithubAuth {
                user,
                token: auth,
                copilot_auth: copilot,
            };

            return Ok(auth);
        }

        let auth = self.auth().await.unwrap();
        utils::write_token_to_config_file(&auth.token.access_token);

        Ok(auth)
    }
}
