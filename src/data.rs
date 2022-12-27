/// Data structs and function to print tracks found.
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
pub struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items<T> {
    pub items: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse {
    pub tracks: Items<Track>,
}

pub fn print_tracks(tracks: Vec<&Track>) {
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
