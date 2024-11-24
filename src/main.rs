use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "debug".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration."); 
    tracing::info!(
        "Starting application in {} mode", 
        std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "local".into())
    );
    tracing::info!(
        "Database settings - Host: {}, Database: {}, User: {}",
        configuration.database.host,
        configuration.database.database_name,
        configuration.database.username
    );

    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    // Test database connection explicitly
    match sqlx::query("SELECT 1").execute(&connection_pool).await {
        Ok(_) => tracing::info!("✓ Database connection test successful"),
        Err(e) => {
            tracing::error!("✗ Database connection test failed: {:?}", e);
            panic!("Cannot continue without database connection");
        }
    }

    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    tracing::info!("Starting server at {}", address);
    let listener = TcpListener::bind(address)?;
    
    run(listener, connection_pool)?.await?;
    Ok(())
}

