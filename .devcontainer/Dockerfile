FROM mcr.microsoft.com/devcontainers/rust:trixie

RUN apt-get update && apt-get install -y postgresql-client

USER vscode

RUN cargo install sqlx-cli --no-default-features --features postgres
