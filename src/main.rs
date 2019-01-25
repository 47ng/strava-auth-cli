// Required for Rocket code generation to work
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

use std::sync::mpsc;
use structopt::StructOpt;
use webbrowser;

mod server;
mod strava;

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

  let auth_url = strava::auth_url(cli_args.client_id);
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
      match strava::exchange_token(&auth_info.code, cli_args.client_id, &cli_args.client_secret) {
        Ok(login) => {
          println!("{:#?}", login);
          println!("Scopes {:#?}", auth_info.scopes);
        }
        Err(error) => eprintln!("Error: {:#?}", error),
      }
    }
    Err(error) => eprintln!("Error: {:#?}", error),
  }
}
