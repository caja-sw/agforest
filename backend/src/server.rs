use actix_web::{App, HttpServer, web};
use anyhow::{Context, Ok, Result};
use sqlx::postgres::PgPoolOptions;
use tokio::runtime::Builder as TokioRuntimeBuilder;
use tracing_actix_web::TracingLogger;

use crate::{Config, services};

pub fn start_server(config: &Config) -> Result<()> {
    let runtime = TokioRuntimeBuilder::new_multi_thread()
        .worker_threads(config.worker_threads)
        .max_blocking_threads(config.max_blocking_threads)
        .enable_all()
        .build()
        .context("Failed to create Tokio runtime")?;

    runtime.block_on(async {
        let pool = PgPoolOptions::new()
            .max_connections(config.database_connection_limit)
            .connect(&config.database_url)
            .await
            .context("Failed to create database connection pool")?;

        HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .app_data(web::Data::new(pool.clone()))
                .service(services::get_categories)
                .service(services::get_posts)
                .service(services::get_post)
                .service(services::create_post)
                .service(services::create_comment)
                .service(services::delete_post)
                .service(services::delete_comment)
        })
        .bind((config.server_host.as_str(), config.server_port))
        .context("Failed to bind server")?
        .run()
        .await
        .context("Failed to run server")?;

        Ok(())
    })?;

    Ok(())
}
