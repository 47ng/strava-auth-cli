use rocket::config::{Config, Environment, LoggingLevel};
use rocket::http::RawStr;

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
