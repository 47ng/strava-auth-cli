// Required for Rocket code generation to work
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::sync::mpsc;
use structopt::StructOpt;
use webbrowser;

mod server;

fn make_strava_auth_url(client_id: u32) -> String {
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

#[derive(Debug, StructOpt)]
#[structopt(name = "strava-auth")]
/// Authorize and authenticate a Strava API app.
///
/// Requires a GUI web browser to be available.
struct CliArgs {
  #[structopt(short = "i", long = "id")]
  client_id: u32,

  #[structopt(short = "s", long = "secret")]
  client_secret: String,
}

fn main() {
  let cli_args = CliArgs::from_args();

  let auth_url = make_strava_auth_url(cli_args.client_id);
  if webbrowser::open(&auth_url).is_err() {
    // Try manually
    println!("Visit the following URL to authorize your app with Strava:");
    println!("{}\n", auth_url);
  }

  let (tx, rx) = mpsc::channel();
  std::thread::spawn(move || {
    server::start(tx);
  });

  // recv() is blocking, so the main thread will patiently
  // wait for data to be sent through the channel.
  // This way the server thread stays alive for as long as
  // it's needed.
  match rx.recv().unwrap() {
    Ok(auth_info) => {
      println!("{:#?}", auth_info);
      // Do something with the result
    }
    Err(error) => eprintln!("{}", error),
  }
}
