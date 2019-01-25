use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Login {
  pub access_token: String,
  pub refresh_token: String,
}

pub type LoginResult = Result<Login, reqwest::Error>;

pub fn exchange_token(code: &str, id: u32, secret: &str) -> LoginResult {
  let mut body = HashMap::new();
  body.insert("client_id", format!("{}", id));
  body.insert("client_secret", String::from(secret));
  body.insert("code", String::from(code));
  body.insert("grant_type", String::from("authorization_code"));
  let mut res = reqwest::Client::new()
    .post("https://www.strava.com/oauth/token")
    .json(&body)
    .send()?
    .error_for_status()?;
  Ok(res.json()?)
}

pub fn auth_url(client_id: u32) -> String {
  let scopes = [
    // "read", // Shadowed by read_all
    "read_all",
    "profile:read_all",
    "profile:write",
    // "activity:read", // Shadowed by activity:read_all
    "activity:read_all",
    "activity:write",
  ]
  .join(",");

  let params = [
    format!("client_id={}", client_id),
    String::from("redirect_uri=http://localhost:8000"),
    String::from("response_type=code"),
    String::from("approval_prompt=auto"),
    format!("scope={}", scopes),
  ]
  .join("&");
  format!("https://www.strava.com/oauth/authorize?{}", params)
}
