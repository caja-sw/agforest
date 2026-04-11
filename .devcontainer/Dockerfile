FROM mcr.microsoft.com/devcontainers/rust:trixie
RUN apt-get update && apt-get install -y postgresql-client
USER vscode
RUN cargo install sqlx-cli --no-default-features --features postgres
RUN echo 'alias cdf="cd /workspaces/agforest/frontend"' | tee -a ~/.bashrc ~/.zshrc
RUN echo 'alias cdb="cd /workspaces/agforest/backend"' | tee -a ~/.bashrc ~/.zshrc
RUN echo 'alias f="cdf && npm run dev"' | tee -a ~/.bashrc ~/.zshrc
RUN echo 'alias b="cdb && cargo run"' | tee -a ~/.bashrc ~/.zshrc
RUN echo 'alias ci="cdf && npm ci"' | tee -a ~/.bashrc ~/.zshrc
RUN echo 'alias res="cdb && sqlx database reset -yf && PGPASSWORD=root psql -h database -d agforest -c \"INSERT INTO categories (name, readonly) VALUES ('\''자유 게시판'\'', false), ('\''문의사항'\'', false), ('\''공지사항'\'', true);\""' | tee -a ~/.bashrc ~/.zshrc
RUN echo 'alias pre="cdb && cargo sqlx prepare"' | tee -a ~/.bashrc ~/.zshrc
