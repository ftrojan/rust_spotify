/// Make HTTP request to Spotify API
/// https://blog.logrocket.com/making-http-requests-rust-reqwest/
/// get token: https://developer.spotify.com/console/get-search-item/?q=Muse&type=track&market=US&limit=5&offset=5&include_external=
use reqwest;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};
use rust_spotify::data;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        // go check out her latest album. It's 🔥
        query = "Little Simz"
    );
    let response = client
        .get(url)
        .header(AUTHORIZATION, "Bearer BQCUo4fjU0yU2BvJWdaNqay0FRqoTtgiRHQ0zfLsEi6-F4NFywQLNF_soooazl7erMaDmr6FXEMPCEgMB8Fvu7SXop10MpCTW6bNzR7NwCN2xBRtKcnBkX6aIUDYtm8X9azgqvqh4y60ZOcSs_8hoH17GtHUq4tPp283x9TzWpVznWpNZC0tE4cNuax9SncXYwQ")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<data::APIResponse>().await {
                Ok(parsed) => data::print_tracks(parsed.tracks.items.iter().collect()),
                Err(_) => println!("Hm, the response did not match the shape we expected."),
            };
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token at https://developer.spotify.com/console/get-search-item/?q=Muse&type=track&market=US&limit=5&offset=5&include_external=");
        },
        _ => {
            panic!("Uh oh! Something unexpected happened.");
        },
    };
}
