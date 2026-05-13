use actix_web::{App, HttpServer, web};
use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use tracing_actix_web::TracingLogger;

use crate::{Config, services};

pub fn start_server(config: &Config) -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(config.worker_threads)
        .max_blocking_threads(config.max_blocking_threads)
        .enable_all()
        .build()
        .context("Failed to create tokio runtime")?;

    runtime.block_on(async {
        let pool = PgPoolOptions::new()
            .max_connections(config.database_connection_limit)
            .connect(&config.database_url)
            .await
            .context("Failed to create database connection pool")?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .context("Failed to run database migrations")?;

        HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .app_data(web::Data::new(pool.clone()))
                .service(services::get_default_board)
                .service(services::get_board)
                .service(services::get_writables)
                .service(services::get_post)
                .service(services::create_post)
                .service(services::create_comment)
                .service(services::delete_post)
                .service(services::delete_comment)
                .service(services::like_post)
                .service(services::like_comment)
                .service(services::unlike_post)
                .service(services::unlike_comment)
                .service(services::view_post)
        })
        .bind((config.server_host.as_str(), config.server_port))
        .context("Failed to bind server")?
        .run()
        .await
        .context("Failed to run server")?;

        anyhow::Ok(())
    })?;

    Ok(())
}
