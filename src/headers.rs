use reqwest::header::HeaderMap;

/// `headers` is a macro that allows for easy creation of a `HeaderMap`.
/// It takes in pairs of identifiers and expressions, where each identifier
/// represents a header field name and each expression is the corresponding
/// header value.
///
/// # Examples
///
/// ```
/// let headers = headers! {
///     "Content-Type": "application/json",
///     "Authorization": "Bearer token"
/// };
/// ```
///
/// This will create a `HeaderMap` with "Content-Type" and "Authorization"
/// headers, with the corresponding values "application/json" and "Bearer token".
///
/// # Note
///
/// The header field names are case-insensitive.
macro_rules ! headers {
    ( $($name:expr => $value:expr),* ) => {
        {
            let mut headers = HeaderMap::new();
            $(
                headers.insert($name, reqwest::header::HeaderValue::from_str(&$value).unwrap());
            )*
            headers
        }
    };
}

pub(crate) trait Headers {
    fn to_headers(&self) -> HeaderMap;
}

pub(crate) struct LoginHeaders();

impl Headers for LoginHeaders {
    fn to_headers(&self) -> HeaderMap {
        headers! {
            "Accept" => "application/json",
            "User-Agent" => "GithubCopilot/1.133.0",
            "X-Editor-Version" => "Neovim/0.9.2",
            "X-Editor-Plugin-Version" => "copilot.lua/1.11.4",
            "X-User-Agent-Version" => "GithubCopilot/1.133.0"
        }
    }
}

pub(crate) struct GithubUserHeaders<'a> {
    pub token: &'a String,
    pub token_type: &'a String,
}

impl<'a> Headers for GithubUserHeaders<'a> {
    fn to_headers(&self) -> HeaderMap {
        headers! {
            "Authorization" => format!("{} {}", self.token_type, self.token),
            "User-Agent" => "GithubCopilot/1.133.0",
            "Accept" => "application/json"
        }
    }
}

pub(crate) struct GithubInternalHeaders<'a> {
    pub token: &'a String,
}

impl<'a> Headers for GithubInternalHeaders<'a> {
    fn to_headers(&self) -> HeaderMap {
        headers! {
            "Authorization" => format!("token {}", self.token),
            "user-agent" => "GitHubCopilotChat/0.12.2023120701",
            "editor-version" => "vscode/1.85.1",
            "editor-plugin-version" => "copilot-chat/0.12.2023120701"
        }
    }
}

pub(crate) struct CopilotCompletionHeaders<'a> {
    pub token: &'a String,
    pub vscode_sid: &'a String,
    pub device_id: &'a String,
}

impl<'a> Headers for CopilotCompletionHeaders<'a> {
    fn to_headers(&self) -> HeaderMap {
        headers! {
            "Authorization" => format!("Bearer {}", self.token),
            "vscode-sessionid" => self.vscode_sid,
            "machineid" => self.device_id,
            "editor-version" => "vscode/1.85.1",
            "editor-plugin-version" => "copilot-chat/0.12.2023120701",
            "openai-organization" => "github-copilot",
            "openai-intent" => "conversation-panel",
            "Content-Type" => "application/json",
            "User-Agent" => "GitHubCopilotChat/0.12.2023120701"
        }
    }
}
