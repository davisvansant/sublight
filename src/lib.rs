use hyper::client::connect::HttpConnector;
use hyper::Body;
use hyper::Client;
use hyper::Error;
use hyper::Method;
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

    pub async fn build_request(
        &self,
        method: Method,
        body: Body,
        path: Option<&str>,
    ) -> Request<Body> {
        let uri_part = self.endpoint.clone().into_parts();
        let uri = if path.is_some() {
            Uri::builder()
                .scheme(uri_part.scheme.unwrap())
                .authority(uri_part.authority.unwrap())
                .path_and_query(path.unwrap())
                .build()
                .unwrap()
        } else {
            Uri::builder()
                .scheme(uri_part.scheme.unwrap())
                .authority(uri_part.authority.unwrap())
                .path_and_query("/foo")
                .build()
                .unwrap()
        };
        Request::builder()
            .method(method)
            .uri(uri)
            .body(body)
            .unwrap()
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
    async fn build_request() {
        let test_runner = Runner::init().await;
        let test_method = Method::GET;
        let test_body = Body::empty();
        let test_request = test_runner
            .build_request(test_method, test_body, None)
            .await;
        assert_eq!(test_request.method().as_str(), "GET");
        assert_eq!(test_request.uri(), "http://example.com/foo");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn send() -> Result<(), Error> {
        let test_runner = Runner::init().await;
        let test_method = Method::GET;
        let test_body = Body::empty();
        let test_request = test_runner
            .build_request(test_method, test_body, None)
            .await;
        let test_response = test_runner.send(test_request).await?;
        assert_eq!(test_response.status().as_str(), "404");
        Ok(())
    }
}
