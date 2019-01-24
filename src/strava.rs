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
