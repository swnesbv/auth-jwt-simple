use redis::AsyncCommands;

pub async fn open() -> MultiplexedConnection {
    match redis::Client::open("redis://localhost") {
        Ok(expr) => match expr.get_multiplexed_async_connection().await {
            Ok(connection) => {
                tracing::info!("Connected to redis");
                connection
            }
            Err(err) => {
                tracing::error!("Could not connect to redis: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            tracing::error!("Could not open redis: {}", e);
            std::process::exit(1);
        }
    }
}