use momento::CacheClient;
use tracing::instrument;

#[instrument(name = "Momomento GET/SET")]
pub async fn get_set_momento(cache_client: &CacheClient) {
    let set_result = cache_client.set("CacheableTable", "Hello", "World").await;
    match set_result {
        Ok(_) => println!("Successfully set cache value for key my-cache-key!"),
        Err(e) => println!("Uh-oh. Failed to set cache key: {}", e),
    }

    let get_result: String = cache_client.get("CacheableTable", "Hello")
        .await
        .expect("Failed to get cache value for key my-cache-key")
        .try_into()
        .expect("Failed to convert cache value to String");

    println!("Successfully retrieved cache value for key my-cache-key: {}", get_result);
}