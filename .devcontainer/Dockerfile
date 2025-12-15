FROM mcr.microsoft.com/devcontainers/rust:trixie

RUN apt-get update && apt-get install -y postgresql-client

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
