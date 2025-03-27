use reqwest::Client;
use tracing::info;
use tracing::instrument;

#[instrument(name = "ALB Request")]
pub async fn make_alb_request(client: &Client) {
    make_request(client,
                 "http://mo-lb-23604495.us-west-2.elb.amazonaws.com".to_string()).await;
}

#[instrument(name = "APIGW Request")]
pub async fn make_apigw_request(client: &Client) {
    make_request(client,
                 "https://lo4bpy6kj7.execute-api.us-west-2.amazonaws.com/main/default".to_string()).await;
}

#[instrument(name = "HTTP Request")]
pub async fn make_http_request(client: &Client) {
    make_request(client,
                 "https://a6j2oy9zqk.execute-api.us-west-2.amazonaws.com/main/default".to_string()
                ).await;
}

async fn make_request(client: &Client, url: String) {
    let response = client
        .get(url)
        .send()
        .await;

    match response {
        Ok(r) => {
            info!("Response: {}", r.status());
        }
        Err(e) => {
            info!("Error: {}", e);
        }
    }
}