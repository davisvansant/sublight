use hyper::client::connect::HttpConnector;
use hyper::Body;
use hyper::Client;
use hyper::Error;
use hyper::Request;
use hyper::Response;
use hyper::Uri;

pub struct Runner {
    pub client: Client<HttpConnector, Body>,
    pub endpoint: Uri,
}

impl Runner {
    pub async fn init() -> Runner {
        let endpoint = Uri::from_static("http://example.com/foo");
        let client = Client::new();

        Runner { endpoint, client }
    }

    pub async fn send(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
        let response = self.client.request(request).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn init() {
        let test_runner = Runner::init().await;
        assert_eq!(test_runner.endpoint, "http://example.com/foo");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn send() -> Result<(), Error> {
        let test_runner = Runner::init().await;
        let request = Request::builder()
            .method("GET")
            .uri(&test_runner.endpoint)
            .body(Body::empty())
            .unwrap();
        let response = test_runner.send(request).await?;
        assert_eq!(response.status().as_str(), "404");
        Ok(())
    }
}
