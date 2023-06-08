use crate::{Body, Error, Method, Response, Runner};

const SNAPSHOT_BASE_URL: &str = "/v1/snapshot";

impl Runner {
    pub async fn snapshot_generate(&self) -> Result<Response<Body>, Error> {
        let path = SNAPSHOT_BASE_URL;
        let method = Method::GET;
        let uri = self.build_uri(path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn snapshot_restore(&self) -> Result<Response<Body>, Error> {
        let path = SNAPSHOT_BASE_URL;
        let method = Method::PUT;
        let uri = self.build_uri(path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test(flavor = "multi_thread")]
    async fn snapshot_generate() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("GET", "/v1/snapshot")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.snapshot_generate().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn snapshot_restore() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("PUT", "/v1/snapshot")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.snapshot_restore().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }
}
