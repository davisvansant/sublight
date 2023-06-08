use crate::{Body, Error, Method, Response, Runner};

const ACL_ROLE_BASE_URL: &str = "/v1/acl/role";

impl Runner {
    pub async fn acl_role_create(&self) -> Result<Response<Body>, Error> {
        let path = ACL_ROLE_BASE_URL;
        let method = Method::PUT;
        let uri = self.build_uri(path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn acl_role_read(&self, id: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/{}", ACL_ROLE_BASE_URL, id);
        let method = Method::GET;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn acl_role_read_name(&self, name: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/name/{}", ACL_ROLE_BASE_URL, name);
        let method = Method::GET;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn acl_role_update(&self, id: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/{}", ACL_ROLE_BASE_URL, id);
        let method = Method::PUT;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn acl_role_delete(&self, id: &str) -> Result<Response<Body>, Error> {
        let path = format!("{}/{}", ACL_ROLE_BASE_URL, id);
        let method = Method::DELETE;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn acl_roles(&self) -> Result<Response<Body>, Error> {
        let path = format!("{}{}", ACL_ROLE_BASE_URL, "s");
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
    async fn acl_role_create() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("PUT", "/v1/acl/role")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.acl_role_create().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn acl_role_read() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("GET", "/v1/acl/role/test_role_id")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.acl_role_read("test_role_id").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn acl_role_read_name() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("GET", "/v1/acl/role/name/test_role_name")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.acl_role_read_name("test_role_name").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn acl_role_update() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("PUT", "/v1/acl/role/test_role_id")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.acl_role_update("test_role_id").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn acl_role_delete() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("DELETE", "/v1/acl/role/test_role_id")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.acl_role_delete("test_role_id").await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn acl_roles() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("GET", "/v1/acl/roles")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.acl_roles().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }
}
