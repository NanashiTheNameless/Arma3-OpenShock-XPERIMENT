use reqwest::{blocking::Client, header::HeaderValue, redirect::Policy};
use serde::Serialize;
use std::time::Duration;
use uuid::Uuid;

pub const CONTROL_URL: &str = "https://api.openshock.app/2/shockers/control";
const TOKEN_HEADER: &str = "OpenShockToken";
const USER_AGENT: &str = concat!("Arma3-OpenShock/", env!("CARGO_PKG_VERSION"));

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
        // Do not echo response bodies or credentials into the game's chat/logs.
        Err(format!(
            "OpenShock rejected request (HTTP {}); check token, shocker permissions and connection",
            status.as_u16()
        ))
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
    fn request_matches_the_supplied_v2_schema() {
        let schema: serde_json::Value =
            serde_json::from_str(include_str!("../version-2.json")).unwrap();
        assert!(schema["paths"]["/2/shockers/control"]["post"].is_object());
        assert_eq!(
            schema["components"]["securitySchemes"]["ApiToken"]["name"],
            TOKEN_HEADER
        );
        let schemas = &schema["components"]["schemas"];
        for operation in [Operation::Shock, Operation::Vibrate, Operation::Sound] {
            for seconds in [1, 15] {
                let body = serde_json::to_value(
                    ControlRequest::new(ID.into(), operation, 1, seconds).unwrap(),
                )
                .unwrap();
                let control = &body["shocks"][0];
                for (value, definition) in [
                    (&body, &schemas["ControlRequest"]),
                    (control, &schemas["Control"]),
                ] {
                    for key in definition["required"].as_array().unwrap() {
                        assert!(value.get(key.as_str().unwrap()).is_some());
                    }
                    for key in value.as_object().unwrap().keys() {
                        assert!(definition["properties"].get(key).is_some());
                    }
                }
                assert!(schemas["ControlType"]["enum"]
                    .as_array()
                    .unwrap()
                    .contains(&control["type"]));
                for field in ["intensity", "duration"] {
                    let value = control[field].as_u64().unwrap();
                    let bounds = &schemas["Control"]["properties"][field];
                    assert!((bounds["minimum"].as_u64().unwrap()
                        ..=bounds["maximum"].as_u64().unwrap())
                        .contains(&value));
                }
            }
        }
    }

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
    }

    #[test]
    fn http_contract_and_error_statuses_use_a_local_mock_only() {
        for status in [200, 204, 302, 400, 401, 403, 429, 500] {
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
                            assert!(headers.contains("openshocktoken: test-token"));
                            assert!(headers
                                .contains(&format!("user-agent: {}", USER_AGENT.to_lowercase())));
                            assert!(headers.contains("content-type: application/json"));
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
                    "HTTP/1.1 {status} Mock\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                )
                .unwrap();
            });
            let request = ControlRequest::new(ID.into(), Operation::Vibrate, 1, 1).unwrap();
            let result = send(&url, token_header("test-token").unwrap(), &request);
            assert_eq!(result.is_ok(), (200..300).contains(&status));
            let message = result.unwrap_or_else(|error| error);
            assert!(message.contains(&status.to_string()));
            assert!(!message.contains("test-token"));
            server.join().unwrap();
        }
    }
}
