use crate::{Body, Error, Method, Response, Runner};

const AGENT_CHECKS_BASE_URL: &str = "/v1/agent/check";

impl Runner {
    pub async fn agent_checks(&self) -> Result<Response<Body>, Error> {
        let path = format!("{}{}", AGENT_CHECKS_BASE_URL, "s");
        let uri = self.build_uri(&path).await;
        let method = Method::GET;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_check_register(&self) -> Result<Response<Body>, Error> {
        let path = format!("{}/register", AGENT_CHECKS_BASE_URL);
        let uri = self.build_uri(&path).await;
        let method = Method::PUT;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_check_deregister(&self, check_id: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/deregister/{}", AGENT_CHECKS_BASE_URL, check_id);
        let uri = self.build_uri(&path).await;
        let method = Method::PUT;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_check_pass(&self, check_id: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/pass/{}", AGENT_CHECKS_BASE_URL, check_id);
        let uri = self.build_uri(&path).await;
        let method = Method::PUT;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_check_warn(&self, check_id: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/warn/{}", AGENT_CHECKS_BASE_URL, check_id);
        let uri = self.build_uri(&path).await;
        let method = Method::PUT;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_check_fail(&self, check_id: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/fail/{}", AGENT_CHECKS_BASE_URL, check_id);
        let uri = self.build_uri(&path).await;
        let method = Method::PUT;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_check_update(&self, check_id: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/update/{}", AGENT_CHECKS_BASE_URL, check_id);
        let uri = self.build_uri(&path).await;
        let method = Method::PUT;
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
    async fn agent_checks() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("GET", "/v1/agent/checks")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_checks().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_check_register() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/agent/check/register")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_check_register().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_check_deregister() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/agent/check/deregister/test_check_id")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_check_deregister("test_check_id").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_check_pass() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/agent/check/pass/test_check_id")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_check_pass("test_check_id").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_check_warn() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/agent/check/warn/test_check_id")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_check_warn("test_check_id").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_check_fail() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/agent/check/fail/test_check_id")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_check_fail("test_check_id").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_check_update() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/agent/check/update/test_check_id")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_check_update("test_check_id").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }
}
