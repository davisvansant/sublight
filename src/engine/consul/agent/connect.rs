use crate::{Body, Error, Method, Response, Runner};

const AGENT_CONNECT_BASE_URL: &str = "/v1/agent/connect";

impl Runner {
    pub async fn agent_connect_authorize(&self) -> Result<Response<Body>, Error> {
        let path = format!("{}/authorize", AGENT_CONNECT_BASE_URL);
        let uri = self.build_uri(&path).await;
        let method = Method::PUT;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_connect_ca_roots(&self) -> Result<Response<Body>, Error> {
        let path = format!("{}/ca/roots", AGENT_CONNECT_BASE_URL);
        let uri = self.build_uri(&path).await;
        let method = Method::GET;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_connect_ca_leaf(&self, service: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/ca/leaf/{}", AGENT_CONNECT_BASE_URL, service);
        let uri = self.build_uri(&path).await;
        let method = Method::GET;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_connect_authorize() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/agent/connect/authorize")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_connect_authorize().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_connect_ca_roots() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("GET", "/v1/agent/connect/ca/roots")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_connect_ca_roots().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_connect_ca_leaf() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("GET", "/v1/agent/connect/ca/leaf/test_service")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_connect_ca_leaf("test_service").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }
}
