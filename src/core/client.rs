use reqwest::Client;
use std::sync::Arc;
use tokio::sync::OnceCell;

static CLIENT: OnceCell<Arc<Client>> = OnceCell::const_new();

pub async fn get_client() -> &'static Arc<Client> {
    CLIENT
        .get_or_init(|| async {
            Arc::new(
                Client::builder()
                    .timeout(std::time::Duration::from_secs(30))
                    .pool_max_idle_per_host(20) // Переиспользует соединения!
                    .build()
                    .unwrap(),
            )
        })
        .await
}
