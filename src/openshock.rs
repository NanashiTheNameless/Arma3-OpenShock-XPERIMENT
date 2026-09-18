use reqwest::{
    blocking::{Client, Response},
    header::{HeaderValue, ACCEPT, CONTENT_TYPE},
    redirect::Policy,
};
use serde::Serialize;
use std::{io::Read, time::Duration};
use uuid::Uuid;

pub const CONTROL_URL: &str = "https://api.openshock.app/2/shockers/control";
// https://wiki.openshock.org/dev#authentication
const TOKEN_HEADER: &str = "Open-Shock-Token";
const USER_AGENT: &str = concat!(
    "Arma3-OpenShock/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/NanashiTheNameless/Arma3-OpenShock-XPERIMENT)"
);

#[derive(Clone, Copy, Serialize)]
pub enum Operation {
    Shock,
    Vibrate,
    Sound,
}

impl Operation {
    pub fn callback_name(self) -> &'static str {
        match self {
            Self::Shock => "Shock",
            Self::Vibrate => "Vibrate",
            Self::Sound => "Beep",
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControlRequest {
    shocks: Vec<Control>,
    custom_name: &'static str,
}

#[derive(Serialize)]
struct Control {
    id: String,
    #[serde(rename = "type")]
    operation: Operation,
    intensity: u32,
    duration: u32,
    exclusive: bool,
}

impl ControlRequest {
    pub fn new(
        shocker_id: String,
        operation: Operation,
        intensity: u32,
        duration_seconds: u32,
    ) -> Result<Self, String> {
        let id = Uuid::parse_str(shocker_id.trim())
            .ok()
            .filter(|id| !id.is_nil())
            .ok_or("OpenShock Shocker ID must be a non-empty UUID")?;
        if !matches!(operation, Operation::Sound) && !(1..=100).contains(&intensity) {
            return Err("Intensity must be between 1 and 100".into());
        }
        // Preserve the mod's 1–15 second range. Convert exactly once at the API boundary.
        if !(1..=15).contains(&duration_seconds) {
            return Err("Duration must be between 1 and 15 seconds".into());
        }
        Ok(Self {
            shocks: vec![Control {
                id: id.to_string(),
                operation,
                intensity: if matches!(operation, Operation::Sound) {
                    0
                } else {
                    intensity
                },
                duration: duration_seconds * 1000,
                exclusive: true,
            }],
            custom_name: "Arma 3 OpenShock",
        })
    }
}

pub fn token_header(token: &str) -> Result<HeaderValue, String> {
    let token = token.trim();
    if token.is_empty() {
        return Err("OpenShock API token is required".into());
    }
    let mut header = HeaderValue::from_str(token)
        .map_err(|_| "OpenShock API token contains invalid header characters".to_string())?;
    header.set_sensitive(true);
    Ok(header)
}

pub fn send(url: &str, token: HeaderValue, request: &ControlRequest) -> Result<String, String> {
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .redirect(Policy::none())
        .user_agent(USER_AGENT)
        .build()
        .map_err(|_| "Could not initialize OpenShock HTTP client".to_string())?;
    // Do not retry: a timeout does not prove that the hardware command was not delivered.
    let response = client
        .post(url)
        .header(TOKEN_HEADER, token)
        .header(ACCEPT, "application/json, application/problem+json")
        .json(request)
        .send()
        .map_err(|error| {
            if error.is_timeout() {
                "OpenShock request timed out; delivery unknown (not retried)".to_string()
            } else {
                "OpenShock network request failed; delivery unknown (not retried)".to_string()
            }
        })?;
    let status = response.status();
    if status.is_success() {
        Ok(format!(
            "OpenShock accepted request (HTTP {})",
            status.as_u16()
        ))
    } else {
        Err(format!(
            "OpenShock rejected request (HTTP {}); {}",
            status.as_u16(),
            rejection_reason(response)
        ))
    }
}

fn rejection_reason(response: Response) -> &'static str {
    let status = response.status().as_u16();
    if response
        .headers()
        .get("cf-mitigated")
        .and_then(|v| v.to_str().ok())
        == Some("challenge")
    {
        return "Cloudflare challenged the request before API authentication; check network access to api.openshock.app";
    }
    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    if content_type == "application/problem+json" || content_type == "application/json" {
        // Only map known problem types to fixed text. Never display arbitrary server
        // text (which could contain credentials), and bound how much we read.
        let mut body = Vec::new();
        if response.take(8193).read_to_end(&mut body).is_ok() && body.len() <= 8192 {
            if let Ok(problem) = serde_json::from_slice::<serde_json::Value>(&body) {
                match problem.get("type").and_then(|v| v.as_str()) {
                    Some("Authorization.Token.PermissionMissing") => return "API token lacks the required Shockers_Use permission; check its permissions in OpenShock",
                    Some("ApiToken.Paused") => return "API token is paused; check its control settings in OpenShock",
                    Some("Shocker.Control.NoPermission") => return "control denied for this shocker; check token and share permissions for the requested operation",
                    Some("Shocker.Control.Paused") => return "shocker or share is paused; check its settings in OpenShock",
                    Some("Authentication.TokenInvalid") => return "API token is invalid or expired; update the token in Addon Options",
                    Some("Authentication.HeaderMissingOrInvalid") => return "API token header was missing or invalid; check the token and any proxy",
                    Some("Authentication.AccountDeactivated") => return "OpenShock account is deactivated",
                    _ => {}
                }
            }
        }
    } else if status == 403 && content_type == "text/html" {
        return "received an HTML denial page; a proxy or firewall may have blocked the request before API authentication";
    }
    match status {
        401 => "authentication failed; check that the raw API token is valid and unexpired",
        403 => "access forbidden; check Shockers_Use permission, token pause state and shocker/share permissions; a proxy or firewall may also deny access",
        404 => "shocker or endpoint not found; check the Shocker ID and account access",
        412 => "control precondition failed; check whether the shocker or share is paused",
        429 => "rate limited; wait and increase the addon cooldown",
        300..=399 => "unexpected redirect was not followed; check the API endpoint",
        _ => "check token, shocker permissions and connection",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io::{Read, Write};
    use std::net::TcpListener;

    const ID: &str = "12345678-1234-1234-1234-123456789abc";

    #[test]
    fn operations_and_seconds_are_serialized_for_openshock() {
        for (operation, expected_type, intensity) in [
            (Operation::Shock, "Shock", 42),
            (Operation::Vibrate, "Vibrate", 42),
            (Operation::Sound, "Sound", 0),
        ] {
            for seconds in [1, 5, 15] {
                let request =
                    ControlRequest::new(ID.into(), operation, intensity, seconds).unwrap();
                assert_eq!(
                    serde_json::to_value(request).unwrap(),
                    json!({
                        "shocks": [{"id": ID, "type": expected_type, "intensity": intensity,
                            "duration": seconds * 1000, "exclusive": true}],
                        "customName": "Arma 3 OpenShock"
                    })
                );
            }
        }
    }

    #[test]
    fn invalid_controls_and_credentials_are_rejected() {
        for operation in [Operation::Shock, Operation::Vibrate, Operation::Sound] {
            for duration in [0, 16, u32::MAX] {
                assert!(ControlRequest::new(ID.into(), operation, 1, duration).is_err());
            }
        }
        for operation in [Operation::Shock, Operation::Vibrate] {
            for intensity in [0, 101, u32::MAX] {
                assert!(ControlRequest::new(ID.into(), operation, intensity, 1).is_err());
            }
        }
        for id in ["", "share-code", "00000000-0000-0000-0000-000000000000"] {
            assert!(ControlRequest::new(id.into(), Operation::Shock, 1, 1).is_err());
        }
        for token in ["", "  ", "token\r\nInjected: value"] {
            assert!(token_header(token).is_err());
        }
        assert!(token_header("test-token").unwrap().is_sensitive());
        assert_eq!(token_header(" \r\ntest-token\t ").unwrap(), "test-token");
    }

    #[test]
    fn http_contract_and_error_statuses_use_a_local_mock_only() {
        for (status, content_type, body, expected, extra_headers) in [
            (200, "application/json", "", "accepted", ""),
            (204, "application/json", "", "accepted", ""),
            (
                302,
                "text/html",
                "",
                "redirect was not followed",
                "Location: https://api.openshock.app/\r\n",
            ),
            (400, "application/json", "", "rejected", ""),
            (401, "application/json", "", "authentication failed", ""),
            (403, "application/json", "", "Shockers_Use", ""),
            (
                403,
                "application/problem+json; charset=utf-8",
                r#"{"type":"Authorization.Token.PermissionMissing","detail":"test-token"}"#,
                "lacks the required Shockers_Use",
                "",
            ),
            (
                403,
                "application/problem+json",
                r#"{"type":"ApiToken.Paused"}"#,
                "API token is paused",
                "",
            ),
            (
                403,
                "application/problem+json",
                r#"{"type":"Shocker.Control.NoPermission"}"#,
                "control denied for this shocker",
                "",
            ),
            (
                401,
                "application/problem+json",
                r#"{"type":"Authentication.TokenInvalid"}"#,
                "invalid or expired",
                "",
            ),
            (
                412,
                "application/problem+json",
                r#"{"type":"Shocker.Control.Paused"}"#,
                "shocker or share is paused",
                "",
            ),
            (
                403,
                "text/html",
                "<html>test-token</html>",
                "HTML denial page",
                "",
            ),
            (
                403,
                "text/html",
                "<html>test-token</html>",
                "Cloudflare challenged",
                "cf-mitigated: challenge\r\n",
            ),
            (
                403,
                "application/json",
                r#"{"type":"test-token","title":"test-token"}"#,
                "access forbidden",
                "",
            ),
            (
                403,
                "application/problem+json",
                "invalid JSON test-token",
                "access forbidden",
                "",
            ),
            (429, "application/json", "", "rate limited", ""),
            (500, "application/json", "", "rejected", ""),
        ] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let url = format!(
                "http://{}/2/shockers/control",
                listener.local_addr().unwrap()
            );
            let server = std::thread::spawn(move || {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                let mut raw = Vec::new();
                let mut buffer = [0; 4096];
                loop {
                    let read = stream.read(&mut buffer).unwrap();
                    assert!(read > 0);
                    raw.extend_from_slice(&buffer[..read]);
                    if let Some(end) = raw.windows(4).position(|w| w == b"\r\n\r\n") {
                        let headers = String::from_utf8_lossy(&raw[..end]).to_lowercase();
                        let length: usize = headers
                            .lines()
                            .find_map(|line| line.strip_prefix("content-length: "))
                            .unwrap()
                            .parse()
                            .unwrap();
                        if raw.len() >= end + 4 + length {
                            assert!(headers.starts_with("post /2/shockers/control http/1.1"));
                            assert!(headers
                                .lines()
                                .any(|line| line == "open-shock-token: test-token"));
                            assert!(!headers.contains("authorization:"));
                            assert!(headers
                                .contains(&format!("user-agent: {}", USER_AGENT.to_lowercase())));
                            assert!(headers.contains("content-type: application/json"));
                            assert!(headers
                                .contains("accept: application/json, application/problem+json"));
                            let body: serde_json::Value =
                                serde_json::from_slice(&raw[end + 4..]).unwrap();
                            assert_eq!(body["shocks"][0]["type"], "Vibrate");
                            assert_eq!(body["shocks"][0]["duration"], 1000);
                            break;
                        }
                    }
                }
                write!(
                    stream,
                    "HTTP/1.1 {status} Mock\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n{extra_headers}Connection: close\r\n\r\n{body}", body.len()
                )
                .unwrap();
            });
            let request = ControlRequest::new(ID.into(), Operation::Vibrate, 1, 1).unwrap();
            let result = send(&url, token_header("  test-token\r\n").unwrap(), &request);
            assert_eq!(result.is_ok(), (200..300).contains(&status));
            let message = result.unwrap_or_else(|error| error);
            assert!(message.contains(&status.to_string()));
            assert!(message.contains(expected), "{message}");
            assert!(!message.contains("test-token"));
            server.join().unwrap();
        }
    }
}
