use structopt::StructOpt;

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
  println!("{:#?}", cli_args);
}
