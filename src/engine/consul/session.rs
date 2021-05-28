use crate::{Body, Error, Method, Response, Runner};

const SESSION_BASE_URL: &str = "/v1/session";

impl Runner {
    pub async fn session_create(&self) -> Result<Response<Body>, Error> {
        let path = format!("{}/create", SESSION_BASE_URL);
        let method = Method::PUT;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn session_destroy(&self, uuid: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/destroy/{}", SESSION_BASE_URL, uuid);
        let method = Method::PUT;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn session_info(&self, uuid: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/info/{}", SESSION_BASE_URL, uuid);
        let method = Method::GET;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn session_node(&self, node: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/node/{}", SESSION_BASE_URL, node);
        let method = Method::GET;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn session_list(&self) -> Result<Response<Body>, Error> {
        let path = format!("{}/list", SESSION_BASE_URL);
        let method = Method::GET;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn session_renew(&self, uuid: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/renew/{}", SESSION_BASE_URL, uuid);
        let method = Method::PUT;
        let uri = self.build_uri(&path).await;
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
    async fn session_create() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/session/create")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.session_create().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn session_destroy() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/session/destroy/test_uuid")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.session_destroy("test_uuid").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn session_info() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("GET", "/v1/session/info/test_uuid")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.session_info("test_uuid").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn session_node() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("GET", "/v1/session/node/test_node")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.session_node("test_node").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn session_list() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("GET", "/v1/session/list")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.session_list().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn session_renew() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/session/renew/test_uuid")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.session_renew("test_uuid").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }
}
