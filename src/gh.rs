#![allow(dead_code)]

use reqwest::{self};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{
    headers::{self, Headers},
    utils,
    urls
};

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
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub site_admin: bool,
    pub name: String,
    pub company: Option<String>,
    pub blog: String,
    pub location: String,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: String,
    pub twitter_username: Option<String>,
    pub public_repos: u64,
    pub public_gists: u64,
    pub followers: u64,
    pub following: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubCopilotAuth {
    pub annotations_enabled: bool,
    pub chat_enabled: bool,
    pub chat_jetbrains_enabled: bool,
    pub code_quote_enabled: bool,
    pub copilot_ide_agent_chat_gpt4_small_prompt: bool,
    pub copilotignore_enabled: bool,
    pub expires_at: u64,
    pub intellij_editor_fetcher: bool,
    pub prompt_8k: bool,
    pub public_suggestions: String,
    pub refresh_in: u64,
    pub sku: String,
    pub snippy_load_test_enabled: bool,
    pub telemetry: String,
    pub token: String,
    pub tracking_id: String,
    pub vsc_electron_fetcher: bool,
    pub vsc_panel_v2: bool,
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
        let headers = headers::DefaultLoginHeaders().to_headers();

        let req = reqwest::Client::new()
            .post(urls::DEVICE_CODE_LOGIN_URL)
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
        // let headers = DEFAULT_LOGIN_HEADERS.to_headers();
        let headers = headers::DefaultLoginHeaders().to_headers();

        let req = reqwest::Client::new()
            .post(urls::DEVICE_CODE_TOKEN_CHECK_URL)
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
        let headers = headers::DefaultGithubUserHeaders {
            token: &auth.access_token,
            token_type: &auth.token_type,
        }.to_headers();

        let req = reqwest::Client::new()
            .get(urls::GH_AUTH_TOKEN_URL)
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
        let headers = headers::DefaultGithubInternalHeaders {
            token: &auth.access_token,
        }.to_headers();

        let req = reqwest::Client::new()
            .get(urls::GH_COPILOT_INTERNAL_AUTH_URL)
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
