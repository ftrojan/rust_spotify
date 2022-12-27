/// Make HTTP request to Spotify API
/// https://blog.logrocket.com/making-http-requests-rust-reqwest/
/// get token: https://developer.spotify.com/console/get-search-item/?q=Muse&type=track&market=US&limit=5&offset=5&include_external=
use reqwest;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}

fn print_tracks(tracks: Vec<&Track>) {
    let num_tracks = tracks.len();
    println!("{} tracks found", num_tracks);
    for track in tracks {
        println!("track: {}", track.name);
        println!("album: {}", track.album.name);
        println!(
            "artists: {}",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        );
        println!("--------");
    }
}

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
        .header(AUTHORIZATION, "Bearer BQDTpY8GVFyGTLkixPynLQPisEwXeDcd_vchAW9cGd2P6Nz-fEwzx_kvN4BMJmouf6whpX-Df4INrO0jcWWD2WVliHMR08OxUvZdUlelxMZXmxmjWIAEgV_1IdzX4yHm2ew2Igr1wwcg7ewG7xaIflMd_AZJYSjwX9r7GRwt7cmzgxmC36iF9AZNNx87317_NQc")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
                Err(_) => println!("Hm, the response did not match the shape we expected."),
            };
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        },
        _ => {
            panic!("Uh oh! Something unexpected happened.");
        },
    };
}
