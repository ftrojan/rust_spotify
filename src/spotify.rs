use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

pub mod data;


pub fn url_for_query(query: &str) -> String {
    let result = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = query
    );
    result
}

async fn parse_response(response: reqwest::Response) -> Option<data::APIResponse> {
    let result = match response.json::<data::APIResponse>().await {
        Ok(parsed) => Some(parsed),
        Err(_) => {
            println!("Hm, the response did not match the shape we expected.");
            None
        }
    };
    result
}

pub struct Spotify {
    client: reqwest::Client,
    auth_token: String,
}

impl Spotify {

    pub fn create() -> Self {
        let token = "BQC4CYMB5rHeM27LtQd5u1lOv5btNiSSmDVtC8Mqd_6LPHvWkVrzJb7smjJHX1F-ojatcjfpa1JLaz0wipXIiTQ1KCWaGb__yrQ7jU-ql3g2n7KQSHkgCEnGa7ze5_86B_lBUEzxjUpPUMSi8yBni2tPZ7es0kU-eGlmb7lT9ILZ30fjlmaxKyS8TPE7dABnf_c";
        let instance = Spotify {
            client: reqwest::Client::new(),
            auth_token: token.parse().unwrap(),
        };
        instance
    }

    #[tokio::main]
    pub async fn search_for_tracks(self, query: &str) -> Option<data::APIResponse> {
        let url = url_for_query(query);
        let auth = format!("Bearer {}", self.auth_token);
        let response = self.client
            .get(url)
            .header(AUTHORIZATION, auth)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await
            .unwrap();
        let result = match response.status() {
            reqwest::StatusCode::OK => parse_response(response).await,
            reqwest::StatusCode::UNAUTHORIZED => {
                println!("Need to grab a new token at https://developer.spotify.com/console/get-search-item/?q=Muse&type=track&market=US&limit=5&offset=5&include_external=");
                None
            },
            _ => {
                println!("Uh oh! Something unexpected happened.");
                None
            },
        };
        result
    }
}
