use crate::{Body, Error, Method, Response, Runner};

impl Runner {
    pub async fn list_members(&self) -> Result<Response<Body>, Error> {
        let path = "/v1/agent/members";
        let uri = self.build_uri(path).await;
        let method = Method::GET;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_self(&self) -> Result<Response<Body>, Error> {
        let path = "/v1/agent/self";
        let uri = self.build_uri(path).await;
        let method = Method::GET;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_reload(&self) -> Result<Response<Body>, Error> {
        let path = "/v1/agent/reload";
        let uri = self.build_uri(path).await;
        let method = Method::PUT;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_maintenance(&self) -> Result<Response<Body>, Error> {
        let path = "/v1/agent/maintentance";
        let uri = self.build_uri(path).await;
        let method = Method::PUT;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_metrics(&self) -> Result<Response<Body>, Error> {
        let path = "/v1/agent/metrics";
        let uri = self.build_uri(path).await;
        let method = Method::GET;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_monitor(&self) -> Result<Response<Body>, Error> {
        let path = "/v1/agent/monitor";
        let uri = self.build_uri(path).await;
        let method = Method::GET;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_join(&self, address: &str) -> Result<Response<Body>, Error> {
        let path = format!("/v1/agent/join/{}", address);
        let uri = self.build_uri(&path).await;
        let method = Method::PUT;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_leave(&self) -> Result<Response<Body>, Error> {
        let path = "/v1/agent/leave";
        let uri = self.build_uri(path).await;
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
    async fn list_members() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("GET", "/v1/agent/members")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.list_members().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_self() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("GET", "/v1/agent/self")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_self().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_reload() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/agent/reload")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_reload().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_maintenance() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/agent/maintentance")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_maintenance().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_metrics() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("GET", "/v1/agent/metrics")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_metrics().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_monitor() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("GET", "/v1/agent/monitor")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_monitor().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_join() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/agent/join/1.2.3.4")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_join("1.2.3.4").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_leave() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/agent/leave")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_leave().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }
}
