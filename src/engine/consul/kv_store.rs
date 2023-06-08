use crate::{Body, Error, Method, Response, Runner};

const KV_STORE_BASE_URL: &str = "/v1/kv";

impl Runner {
    pub async fn key_get(&self, key: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/{}", KV_STORE_BASE_URL, key);
        let method = Method::GET;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn key_put(&self, key: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/{}", KV_STORE_BASE_URL, key);
        let method = Method::PUT;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn key_delete(&self, key: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/{}", KV_STORE_BASE_URL, key);
        let method = Method::DELETE;
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
    use mockito::Server;

    #[tokio::test(flavor = "multi_thread")]
    async fn key_get() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("GET", "/v1/kv/test_key")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.key_get("test_key").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn key_put() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("PUT", "/v1/kv/test_key")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.key_put("test_key").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn key_delete() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("DELETE", "/v1/kv/test_key")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.key_delete("test_key").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }
}
