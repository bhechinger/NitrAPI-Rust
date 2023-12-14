use reqwest::StatusCode;
use serde::de::DeserializeOwned;

const API_URL: &str = "https://api.nitrado.net";

// Build the URL for all calls
async fn build<T>(url: String) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(url).await;

    match &response {
        Ok(r) => {
            if r.status() != StatusCode::OK {
                return Err(r.status());
            }
        }
        Err(e) => {
            if e.is_status() {
                return Err(e.status().unwrap());
            } else {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }

    // Parse the response body as Json
    let content = response.unwrap().json::<T>().await;

    match content {
        Ok(s) => Ok(s),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

// Make call without parameters nor filters
pub async fn all<T>(call: &str) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    let url = format!("{}/{}", API_URL, call);
    build(url).await
}

#[cfg(test)]
mod tests {
    use crate::{ping, query_builder};
    use reqwest::StatusCode;

    #[tokio::test]
    async fn error_404_not_found() {
        let not: Result<ping::Ping, StatusCode> = query_builder::all("success").await;
        assert_eq!(not.unwrap_err(), StatusCode::NOT_FOUND);
    }
}
