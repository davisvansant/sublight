use crate::{Body, Error, Method, Response, Runner};

impl Runner {
    pub async fn bootstrap(&self) -> Result<Response<Body>, Error> {
        let path = "/v1/acl/bootstrap";
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
    use mockito::mock;

    #[tokio::test(flavor = "multi_thread")]
    async fn bootstrap() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/acl/bootstrap")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.bootstrap().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }
}
