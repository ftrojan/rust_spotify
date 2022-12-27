/// Make HTTP request to Spotify API
/// https://blog.logrocket.com/making-http-requests-rust-reqwest/
/// get token: https://developer.spotify.com/console/get-search-item/?q=Muse&type=track&market=US&limit=5&offset=5&include_external=
use rust_spotify::spotify::data;
use rust_spotify::spotify;

fn main() {
    let s = spotify::Spotify::create();
    let tracks = s.search_for_tracks("Little Simz");
    match tracks {
        None => {},
        Some(parsed) => data::print_tracks(parsed.tracks.items.iter().collect()),
    }
}
