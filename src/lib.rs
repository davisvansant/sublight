use hyper::client::connect::HttpConnector;
use hyper::Body;
use hyper::Client;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn init() {
        let test_runner = Runner::init().await;
        assert_eq!(test_runner.endpoint, "http://example.com/foo");
    }
}
