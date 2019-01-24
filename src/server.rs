use rocket::config::{Config, Environment, LoggingLevel};
use rocket::http::RawStr;

#[derive(Debug)]
pub struct AuthInfo {
  pub code: String,
  pub scopes: Vec<String>,
}

impl AuthInfo {
  pub fn new(code: &RawStr, scopes: &RawStr) -> Self {
    Self {
      code: String::from(code.as_str()),
      scopes: scopes.as_str().split(",").map(String::from).collect(),
    }
  }
}

pub type AuthResult = Result<AuthInfo, String>;

// --

#[get("/?<code>&<scope>")]
fn success(code: &RawStr, scope: &RawStr) -> &'static str {
  println!("Code: {}", code);
  println!("Scope: {}", scope);
  "✅ You may close this browser tab and return to the terminal."
}

#[get("/?<error>", rank = 2)]
fn error(error: &RawStr) -> String {
  println!("{}", error);
  format!("Error: {}, please return to the terminal.", error)
}

// --

pub fn start() {
  let config = Config::build(Environment::Development)
    .log_level(LoggingLevel::Off)
    .finalize()
    .unwrap();
  rocket::custom(config)
    .mount("/", routes![success, error])
    .launch();
}
