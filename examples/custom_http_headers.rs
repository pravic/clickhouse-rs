use clickhouse::{error::Result, Client};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::default()
        .with_url("http://localhost:8123")
        // purposefully invalid credentials in the client configuration for the sake of this example
        .with_user("...")
        .with_password("...")
        // these custom headers will override the auth headers generated by the client
        .with_header("X-ClickHouse-User", "default")
        .with_header("X-ClickHouse-Key", "")
        // or, you could just add your custom headers, e.g., for proxy authentication
        .with_header("X-My-Header", "hello");

    let numbers = client
        .query("SELECT number FROM system.numbers LIMIT 1")
        .fetch_all::<u64>()
        .await?;
    println!("Numbers: {numbers:?}");

    Ok(())
}
