use crate::{Body, Error, Method, Response, Runner};

const ACL_AUTH_METHOD_BASE_URL: &str = "/v1/acl/auth-method";

impl Runner {
    pub async fn acl_auth_method_create(&self) -> Result<Response<Body>, Error> {
        let path = ACL_AUTH_METHOD_BASE_URL;
        let method = Method::PUT;
        let uri = self.build_uri(path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn acl_auth_method_read(&self, name: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/{}", ACL_AUTH_METHOD_BASE_URL, name);
        let method = Method::GET;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn acl_auth_method_update(&self, name: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/{}", ACL_AUTH_METHOD_BASE_URL, name);
        let method = Method::PUT;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn acl_auth_method_delete(&self, name: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/{}", ACL_AUTH_METHOD_BASE_URL, name);
        let method = Method::DELETE;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn acl_auth_methods(&self) -> Result<Response<Body>, Error> {
        let path = format!("{}{}", ACL_AUTH_METHOD_BASE_URL, "s");
        let method = Method::GET;
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
    async fn acl_auth_method_create() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("PUT", "/v1/acl/auth-method")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.acl_auth_method_create().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn acl_auth_method_read() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("GET", "/v1/acl/auth-method/test_auth_method_name")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner
            .acl_auth_method_read("test_auth_method_name")
            .await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn acl_auth_method_update() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("PUT", "/v1/acl/auth-method/test_auth_method_name")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner
            .acl_auth_method_update("test_auth_method_name")
            .await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn acl_auth_method_delete() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("DELETE", "/v1/acl/auth-method/test_auth_method_name")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner
            .acl_auth_method_delete("test_auth_method_name")
            .await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn acl_auth_methods() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("GET", "/v1/acl/auth-methods")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.acl_auth_methods().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }
}
