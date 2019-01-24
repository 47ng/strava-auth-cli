use structopt::StructOpt;
use webbrowser;

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
}
