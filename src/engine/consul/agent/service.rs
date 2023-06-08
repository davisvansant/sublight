use crate::{Body, Error, Method, Response, Runner};

const AGENT_SERVICE_BASE_URL: &str = "/v1/agent/service";

impl Runner {
    pub async fn agent_services(&self) -> Result<Response<Body>, Error> {
        let path = format!("{}{}", AGENT_SERVICE_BASE_URL, "s");
        let uri = self.build_uri(&path).await;
        let method = Method::GET;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_service_configuration(
        &self,
        service_id: &str,
    ) -> Result<Response<Body>, Error> {
        let path = format!("{}/{}", AGENT_SERVICE_BASE_URL, service_id);
        let uri = self.build_uri(&path).await;
        let method = Method::GET;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_health_service_name(
        &self,
        service_name: &str,
    ) -> Result<Response<Body>, Error> {
        let path = format!(
            "{}/health/service/name/{}",
            AGENT_SERVICE_BASE_URL.trim_end_matches("/service"),
            service_name,
        );
        let uri = self.build_uri(&path).await;
        let method = Method::GET;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_health_service_id(&self, service_id: &str) -> Result<Response<Body>, Error> {
        let path = format!(
            "{}/health/service/id/{}",
            AGENT_SERVICE_BASE_URL.trim_end_matches("/service"),
            service_id,
        );
        let uri = self.build_uri(&path).await;
        let method = Method::GET;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_service_register(&self) -> Result<Response<Body>, Error> {
        let path = format!("{}/register", AGENT_SERVICE_BASE_URL);
        let uri = self.build_uri(&path).await;
        let method = Method::PUT;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_service_deregister(
        &self,
        service_id: &str,
    ) -> Result<Response<Body>, Error> {
        let path = format!("{}/deregister/{}", AGENT_SERVICE_BASE_URL, service_id);
        let uri = self.build_uri(&path).await;
        let method = Method::PUT;
        let body = Body::empty();
        let request = self.build_request(method, uri, body).await;
        let response = self.client.request(request).await?;
        Ok(response)
    }

    pub async fn agent_service_maintenance(
        &self,
        service_id: &str,
    ) -> Result<Response<Body>, Error> {
        let path = format!("{}/maintentance/{}", AGENT_SERVICE_BASE_URL, service_id);
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
    use mockito::Server;

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_services() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("GET", "/v1/agent/services")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_services().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_service_configuration() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("GET", "/v1/agent/service/test_service_id")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner
            .agent_service_configuration("test_service_id")
            .await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_health_service_name() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("GET", "/v1/agent/health/service/name/test_service_name")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner
            .agent_health_service_name("test_service_name")
            .await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_health_service_id() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("GET", "/v1/agent/health/service/id/test_service_id")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner
            .agent_health_service_id("test_service_id")
            .await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_service_register() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("PUT", "/v1/agent/service/register")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner.agent_service_register().await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_service_deregister() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("PUT", "/v1/agent/service/deregister/test_service_id")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner
            .agent_service_deregister("test_service_id")
            .await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn agent_service_maintenance() -> Result<(), Error> {
        let mut test_server = Server::new();
        let test_mock_url = test_server.url();
        let test_runner = Runner::init(&test_mock_url, None, None).await;
        let mock = test_server
            .mock("PUT", "/v1/agent/service/maintentance/test_service_id")
            .with_status(200)
            .with_header("user-agent", "sublight/0.1.0")
            .with_body("")
            .create();
        test_runner
            .agent_service_maintenance("test_service_id")
            .await?;
        mock.assert();
        assert!(mock.matched());
        Ok(())
    }
}
