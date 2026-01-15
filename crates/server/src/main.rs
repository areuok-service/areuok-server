use api::{create_router, SseManager};
use db::{create_pool, run_migrations};
use log::{debug, error, info, warn};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Initialize env_logger
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info,server=debug,api=debug,db=debug"),
    )
    .format_timestamp_millis()
    .init();

    info!("===========================================");
    info!("       AreUOK Server Starting Up           ");
    info!("===========================================");

    // Log environment info
    debug!("Checking environment configuration...");

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "NOT SET".to_string());
    let masked_url = if database_url != "NOT SET" {
        // Mask password in URL for logging
        if let Some(at_pos) = database_url.find('@') {
            if let Some(colon_pos) = database_url[..at_pos].rfind(':') {
                format!(
                    "{}:****{}",
                    &database_url[..colon_pos],
                    &database_url[at_pos..]
                )
            } else {
                database_url.clone()
            }
        } else {
            database_url.clone()
        }
    } else {
        database_url.clone()
    };
    info!("DATABASE_URL: {}", masked_url);

    let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "NOT SET".to_string());
    debug!("RUST_LOG: {}", rust_log);

    // Connect to database
    info!("Connecting to database...");
    let pool = match create_pool().await {
        Ok(pool) => {
            info!("✓ Database connection established successfully");
            pool
        },
        Err(e) => {
            error!("✗ Failed to connect to database: {}", e);
            error!("Please check your DATABASE_URL environment variable");
            return Err(e.into());
        },
    };

    // Run migrations
    info!("Running database migrations...");
    match run_migrations(&pool).await {
        Ok(_) => {
            info!("✓ Database migrations completed successfully");
        },
        Err(e) => {
            warn!("✗ Migration warning: {}", e);
            warn!("Continuing anyway - tables may already exist");
        },
    }

    let sse_manager = Arc::new(SseManager::new());
    debug!("✓ SSE manager created");

    debug!("Creating application router...");
    let app = create_router(pool, sse_manager)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());
    debug!("✓ Router created with CORS and tracing layers");

    // Bind to address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Binding to address: {}", addr);

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("✓ Successfully bound to {}", addr);
            listener
        },
        Err(e) => {
            error!("✗ Failed to bind to {}: {}", addr, e);
            return Err(e.into());
        },
    };

    info!("===========================================");
    info!("  Server is now listening on port 3000     ");
    info!("  Ready to accept connections!             ");
    info!("===========================================");

    // Print available endpoints
    info!("Available endpoints:");
    info!("  POST   /devices/register");
    info!("  GET    /devices/:id");
    info!("  POST   /devices/:id/signin");
    info!("  GET    /devices/:id/status");
    info!("  POST   /supervision/request");
    info!("  GET    /supervision/pending/:id");
    info!("  POST   /supervision/accept");
    info!("  POST   /supervision/reject");
    info!("  GET    /supervision/list/:id");
    info!("  DELETE /supervision/:relation_id");

    // Start server
    if let Err(e) = axum::serve(listener, app).await {
        error!("Server error: {}", e);
        return Err(e.into());
    }

    Ok(())
}
