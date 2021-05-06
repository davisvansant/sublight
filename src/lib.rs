use http::uri::{Authority, Builder, PathAndQuery, Scheme};
use hyper::client::connect::HttpConnector;
use hyper::header::{HeaderName, HeaderValue};
use hyper::{Body, Client, Error, HeaderMap, Method, Request, Response, Uri};

pub mod engine;

pub struct Runner {
    pub client: Client<HttpConnector, Body>,
    pub endpoint: Uri,
    default_headers: HeaderMap,
    scheme: Scheme,
    authority: Authority,
}

impl Runner {
    pub async fn init(
        uri: &'static str,
        header_name: Option<&'static str>,
        header_value: Option<&str>,
    ) -> Runner {
        let client = Client::new();
        let endpoint = Uri::from_static(uri);
        let mut default_headers = HeaderMap::new();

        if header_name.is_some() && header_value.is_some() {
            let header = HeaderName::from_static(header_name.unwrap());
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
            endpoint,
            client,
            default_headers,
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

    async fn build_uri(&self, path_and_query: &'static str) -> Uri {
        Builder::new()
            .scheme(self.scheme.as_str())
            .authority(self.authority.as_str())
            .path_and_query(PathAndQuery::from_static(path_and_query))
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
        assert_eq!(test_runner.default_headers.is_empty(), true);
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
        assert_eq!(test_runner.default_headers.len(), 1);
        for (name, value) in test_runner.default_headers.iter() {
            assert_eq!(name.as_str(), "test_header_name");
            assert_eq!(value.to_str().unwrap(), "test_header_value");
            assert_eq!(value.is_sensitive(), true);
        }
        assert_eq!(test_runner.scheme.as_str(), "http");
        assert_eq!(test_runner.authority.as_str(), "example.com");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn build_request() {
        let test_runner = Runner::init("http://example.com/", None, None).await;
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
        let test_runner = Runner::init("http://example.com/", None, None).await;
        let test_method = Method::GET;
        let test_body = Body::empty();
        let test_request = test_runner
            .build_request(test_method, test_body, None)
            .await;
        let test_response = test_runner.send(test_request).await?;
        assert_eq!(test_response.status().as_str(), "200");
        Ok(())
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
