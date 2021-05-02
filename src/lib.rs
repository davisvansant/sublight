use http::uri::{Authority, Scheme};
use hyper::client::connect::HttpConnector;
use hyper::Body;
use hyper::Client;
use hyper::Error;
use hyper::HeaderMap;
use hyper::Method;
use hyper::Request;
use hyper::Response;
use hyper::Uri;

pub mod engine;

pub struct Runner {
    pub client: Client<HttpConnector, Body>,
    pub endpoint: Uri,
    default_headers: Option<HeaderMap>,
    scheme: Scheme,
    authority: Authority,
}

impl Runner {
    pub async fn init(uri: &'static str) -> Runner {
        let client = Client::new();
        let endpoint = Uri::from_static(uri);
        let uri_part = endpoint.clone().into_parts();

        let scheme = match uri_part.scheme {
            Some(scheme) => scheme,
            None => panic!("Scheme could not be set!"),
        };

        let authority = match uri_part.authority {
            Some(authority) => authority,
            None => panic!("Authority could not be set!"),
        };

        Runner {
            endpoint,
            client,
            default_headers: None,
            scheme,
            authority,
        }
    }

    pub async fn build_request(
        &self,
        method: Method,
        body: Body,
        path: Option<&str>,
    ) -> Request<Body> {
        let uri = if path.is_some() {
            Uri::builder()
                .scheme(self.scheme.to_owned())
                .authority(self.authority.to_owned())
                .path_and_query(path.unwrap())
                .build()
                .unwrap()
        } else {
            Uri::builder()
                .scheme(self.scheme.to_owned())
                .authority(self.authority.to_owned())
                .path_and_query("")
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
        let test_runner = Runner::init("http://example.com/").await;
        assert_eq!(test_runner.endpoint, "http://example.com/");
        assert_eq!(test_runner.default_headers.is_none(), true);
        assert_eq!(test_runner.scheme.as_str(), "http");
        assert_eq!(test_runner.authority.as_str(), "example.com");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn build_request() {
        let test_runner = Runner::init("http://example.com/").await;
        let test_method = Method::GET;
        let test_body = Body::empty();
        let test_request = test_runner
            .build_request(test_method, test_body, None)
            .await;
        assert_eq!(test_request.method().as_str(), "GET");
        assert_eq!(test_request.uri(), "http://example.com/");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn send() -> Result<(), Error> {
        let test_runner = Runner::init("http://example.com/").await;
        let test_method = Method::GET;
        let test_body = Body::empty();
        let test_request = test_runner
            .build_request(test_method, test_body, None)
            .await;
        let test_response = test_runner.send(test_request).await?;
        assert_eq!(test_response.status().as_str(), "200");
        Ok(())
    }
}
