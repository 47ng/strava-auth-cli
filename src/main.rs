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
struct Arguments {
  #[structopt(short = "i")]
  id: u32,

  #[structopt(short = "s")]
  secret: String,
}

fn main() {
  let args = Arguments::from_args();

  let auth_url = strava::auth_url(args.id);
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
    Ok(auth_info) => match strava::exchange_token(&auth_info.code, args.id, &args.secret) {
      Ok(login) => {
        println!("{:#?}", login);
        println!("Scopes {:#?}", auth_info.scopes);
      }
      Err(error) => eprintln!("Error: {:#?}", error),
    },
    Err(error) => {
      eprintln!("Error: {:#?}", error);
      // Let the async server send its response
      // before the main thread exits.
      std::thread::sleep(std::time::Duration::from_secs(1));
    }
  }
}
