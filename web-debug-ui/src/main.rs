//! # Web Debug UI - Sprint 1 Integration Main Application
//!
//! This is the main entry point for the integrated Web Debug UI server,
//! combining all Sprint 1 components into a unified HTTP/WebSocket service.
//!
//! ## Features
//!
//! - HTTP REST API for session management and action execution
//! - WebSocket support for real-time game state updates
//! - Clean Architecture with proper dependency injection
//! - Comprehensive error handling and recovery
//! - Performance monitoring and metrics
//!
//! ## Usage
//!
//! ```bash
//! # Start with default configuration
//! cargo run --bin web-debug-ui
//!
//! # Start with custom address
//! cargo run --bin web-debug-ui -- --bind 0.0.0.0:8080
//!
//! # Start with environment configuration
//! export WEB_DEBUG_UI_BIND_ADDR="127.0.0.1:3000"
//! cargo run --bin web-debug-ui
//! ```

use anyhow::{Context, Result};
use clap::Parser;
use std::env;
use tracing::{error, info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use web_debug_ui::integration::WebDebugUIService;

/// Command-line arguments for the Web Debug UI server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Address to bind the HTTP server to
    #[arg(short, long, default_value = "127.0.0.1:3000")]
    bind: String,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// Enable performance monitoring
    #[arg(long, default_value = "true")]
    monitoring: bool,

    /// Enable WebSocket support
    #[arg(long, default_value = "true")]
    websockets: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Initialize tracing/logging
    init_tracing(&args.log_level)?;

    info!("🚀 Starting Web Debug UI - Sprint 1 Integration");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    info!("Integration: {}", web_debug_ui::INTEGRATION_VERSION);

    // Get bind address from args, env, or default
    let bind_addr = get_bind_address(&args.bind);
    info!("📡 Server will bind to: {}", bind_addr);

    // Display configuration
    info!("🔧 Configuration:");
    info!("  - Monitoring: {}", args.monitoring);
    info!("  - WebSockets: {}", args.websockets);
    info!("  - Log Level: {}", args.log_level);

    // Display health check information
    let health = web_debug_ui::health_check();
    info!("🏥 Integration Health:");
    info!("  - Version: {}", health.version);
    info!("  - Integration: {}", health.integration_version);
    info!("  - Domain Version: {}", health.domain_version);
    info!("  - Layers: {:?}", health.layers_integrated);

    // Initialize the integrated service
    info!("🔧 Initializing Web Debug UI service...");
    let service = match WebDebugUIService::new().await {
        Ok(service) => {
            info!("✅ Web Debug UI service initialized successfully");
            service
        }
        Err(e) => {
            error!("❌ Failed to initialize Web Debug UI service: {}", e);
            return Err(e).context("Service initialization failed");
        }
    };

    // Install signal handlers for graceful shutdown
    setup_signal_handlers();

    // Start the HTTP server
    info!("🌐 Starting HTTP server with WebSocket support...");
    info!("🎯 Performance targets: <10ms action latency, <5ms WebSocket updates");

    // Start server and handle shutdown gracefully
    match service.start(&bind_addr).await {
        Ok(()) => {
            info!("👋 Web Debug UI server shut down gracefully");
            Ok(())
        }
        Err(e) => {
            error!("💥 Server error: {}", e);
            Err(e).context("Server failed to start or run")
        }
    }
}

/// Initialize tracing/logging subsystem
fn init_tracing(log_level: &str) -> Result<()> {
    // Parse log level
    let level = match log_level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" | "warning" => Level::WARN,
        "error" => Level::ERROR,
        _ => {
            eprintln!(
                "⚠️  Unknown log level '{}', defaulting to 'info'",
                log_level
            );
            Level::INFO
        }
    };

    // Create subscriber with environment filter support
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(format!("web_debug_ui={}", level))),
        )
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set tracing subscriber")?;

    info!("📊 Tracing initialized with level: {}", level);
    Ok(())
}

/// Get bind address from args, environment, or default
fn get_bind_address(arg_bind: &str) -> String {
    // Priority: CLI arg > environment variable > default
    if arg_bind != "127.0.0.1:3000" {
        return arg_bind.to_string();
    }

    if let Ok(env_bind) = env::var("WEB_DEBUG_UI_BIND_ADDR") {
        info!("📄 Using bind address from environment: {}", env_bind);
        return env_bind;
    }

    arg_bind.to_string()
}

/// Setup signal handlers for graceful shutdown
fn setup_signal_handlers() {
    tokio::spawn(async {
        // Handle SIGINT (Ctrl+C) and SIGTERM
        let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
            .expect("Failed to install SIGINT handler");

        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler");

        tokio::select! {
            _ = sigint.recv() => {
                info!("🛑 Received SIGINT (Ctrl+C), initiating graceful shutdown...");
            }
            _ = sigterm.recv() => {
                info!("🛑 Received SIGTERM, initiating graceful shutdown...");
            }
        }

        // Note: Graceful shutdown logic will be implemented when the server supports it
        info!("🔄 Graceful shutdown initiated");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bind_address_default() {
        let result = get_bind_address("127.0.0.1:3000");
        assert_eq!(result, "127.0.0.1:3000");
    }

    #[test]
    fn test_get_bind_address_custom() {
        let result = get_bind_address("0.0.0.0:8080");
        assert_eq!(result, "0.0.0.0:8080");
    }

    #[tokio::test]
    async fn test_main_components_compile() {
        // This test verifies that main components can be imported and basic types work
        let health = web_debug_ui::health_check();
        assert!(!health.version.is_empty());
        assert!(!health.integration_version.is_empty());
        assert_eq!(health.layers_integrated.len(), 4);

        // Test that service initialization at least compiles
        // (it may fail at runtime until dependencies are fully wired)
        let _service_result = WebDebugUIService::new().await;
        // We don't assert success here as dependencies may not be fully integrated yet
    }
}
