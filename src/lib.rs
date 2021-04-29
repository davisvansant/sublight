pub struct Runner {}

impl Runner {
    pub async fn init() -> Runner {
        Runner {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn init() {
        Runner::init().await;
    }
}
