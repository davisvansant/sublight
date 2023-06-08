use crate::{Body, Error, Method, Response, Runner};

pub mod auth_method;
pub mod binding_rule;
pub mod policy;
pub mod role;
pub mod token;

const ACL_BASE_URL: &str = "/v1/acl";

impl Runner {
    pub async fn acl_bootstrap(&self) -> Result<Response<Body>, Error> {
        let path = format!("{}/bootstrap", ACL_BASE_URL);
        let method = Method::PUT;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn acl_replication(&self) -> Result<Response<Body>, Error> {
        let path = format!("{}/replication", ACL_BASE_URL);
        let method = Method::GET;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn acl_login(&self) -> Result<Response<Body>, Error> {
        let path = format!("{}/login", ACL_BASE_URL);
        let method = Method::POST;
        let uri = self.build_uri(&path).await;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn acl_logout(&self) -> Result<Response<Body>, Error> {
        let path = format!("{}/logout", ACL_BASE_URL);
        let method = Method::POST;
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
    async fn acl_bootstrap() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("PUT", "/v1/acl/bootstrap")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.acl_bootstrap().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn acl_replication() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("GET", "/v1/acl/replication")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.acl_replication().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn acl_login() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("POST", "/v1/acl/login")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.acl_login().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn acl_logout() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("POST", "/v1/acl/logout")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.acl_logout().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }
}
