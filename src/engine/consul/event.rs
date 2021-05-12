use crate::{Body, Error, Method, Response, Runner};

impl Runner {
    pub async fn event_fire(&self, name: &str) -> Result<Response<Body>, Error> {
        let path = format!("/v1/event/fire/{}", name);
        let method = Method::PUT;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn event_list(&self) -> Result<Response<Body>, Error> {
        let path = "/v1/event/list";
        let method = Method::GET;
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
    async fn event_fire() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("PUT", "/v1/event/fire/test_event")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.event_fire("test_event").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn event_list() -> Result<(), Error> {
        let test_mock_url = mockito::server_url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = mock("GET", "/v1/event/list")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.event_list().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }
}
