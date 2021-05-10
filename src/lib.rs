use http::uri::{Authority, Builder, Scheme};
use hyper::client::connect::HttpConnector;
use hyper::header::{HeaderName, HeaderValue};
use hyper::{Body, Client, Error, HeaderMap, Method, Request, Response, Uri};

use std::str::FromStr;

pub mod engine;

pub struct Runner {
    pub client: Client<HttpConnector, Body>,
    pub endpoint: Uri,
    default_headers: HeaderMap,
    scheme: Scheme,
    authority: Authority,
}

impl Runner {
    pub async fn init(uri: &str, header_name: Option<&str>, header_value: Option<&str>) -> Runner {
        let client = Client::new();
        let endpoint = Uri::from_str(uri).unwrap();
        let mut default_headers = HeaderMap::new();

        default_headers.reserve(5);

        let user_agent_name = http::header::USER_AGENT;
        let user_agent_value = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

        default_headers.insert(
            user_agent_name,
            HeaderValue::from_str(user_agent_value).unwrap(),
        );

        if header_name.is_some() && header_value.is_some() {
            let header = HeaderName::from_str(header_name.unwrap()).unwrap();
            let value = HeaderValue::from_str(header_value.unwrap());
            if let Ok(mut value) = value {
                value.set_sensitive(true);
                default_headers.insert(header, value);
            }
        }

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
            client,
            endpoint,
            default_headers,
            scheme,
            authority,
        }
    }

    async fn build_request(&self, method: Method, uri: Uri, body: Body) -> Request<Body> {
        let mut request = Request::builder()
            .method(method)
            .uri(uri)
            .body(body)
            .expect("Could not build Request!");

        let headers = request.headers_mut();

        for (name, value) in self.default_headers.iter() {
            headers.insert(name, value.to_owned());
        }

        request
    }

    async fn build_uri(&self, path_and_query: &str) -> Uri {
        Builder::new()
            .scheme(self.scheme.as_str())
            .authority(self.authority.as_str())
            .path_and_query(path_and_query)
            .build()
            .expect("Could not build URI!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn init() {
        let test_runner = Runner::init("http://example.com/", None, None).await;
        assert_eq!(test_runner.endpoint, "http://example.com/");
        assert_eq!(test_runner.default_headers.len(), 1);
        assert!(test_runner.default_headers.capacity() >= 5);
        for (name, value) in test_runner.default_headers.iter() {
            assert_eq!(name.as_str(), "user-agent");
            assert_eq!(value.to_str().unwrap(), "sublight/0.1.0");
            assert_eq!(value.is_sensitive(), false);
        }
        assert_eq!(test_runner.scheme.as_str(), "http");
        assert_eq!(test_runner.authority.as_str(), "example.com");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn init_with_headers() {
        let test_runner = Runner::init(
            "http://example.com/",
            Some("test_header_name"),
            Some("test_header_value"),
        )
        .await;
        assert_eq!(test_runner.endpoint, "http://example.com/");
        assert_eq!(test_runner.default_headers.is_empty(), false);
        assert_eq!(test_runner.default_headers.len(), 2);
        assert!(test_runner.default_headers.capacity() >= 5);
        assert_eq!(
            test_runner
                .default_headers
                .get(http::header::USER_AGENT)
                .unwrap(),
            "sublight/0.1.0",
        );
        assert_eq!(
            test_runner.default_headers.get("test_header_name").unwrap(),
            "test_header_value",
        );
        assert_eq!(test_runner.scheme.as_str(), "http");
        assert_eq!(test_runner.authority.as_str(), "example.com");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn build_request() {
        let test_runner = Runner::init("http://example.com/", None, None).await;
        let test_method = Method::GET;
        let test_uri = test_runner.endpoint.clone();
        let test_body = Body::empty();
        let test_request = test_runner
            .build_request(test_method, test_uri, test_body)
            .await;
        assert_eq!(test_request.method().as_str(), "GET");
        assert_eq!(test_request.uri(), "http://example.com/");
        assert_eq!(test_request.headers().len(), 1);
        for (key, value) in test_request.headers().iter() {
            assert_eq!(key.as_str(), "user-agent");
            assert_eq!(value.to_str().unwrap(), "sublight/0.1.0");
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn build_uri() -> Result<(), Error> {
        let test_runner = Runner::init("http://example.com/", None, None).await;
        let test_path_and_query = "/test_path_and_query";
        let test_build_uri = test_runner.build_uri(test_path_and_query).await;
        let test_parts = test_build_uri.into_parts();

        assert_eq!(test_parts.scheme.unwrap().as_str(), "http");
        assert_eq!(test_parts.authority.unwrap().as_str(), "example.com");
        assert_eq!(
            test_parts.path_and_query.unwrap().as_str(),
            "/test_path_and_query",
        );
        Ok(())
    }
}
