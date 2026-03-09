FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# Install Node.js 20 and PostgreSQL client tools
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs postgresql-client \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Install sqlx-cli for migrations and fix registry ownership for vscode user
RUN cargo install sqlx-cli --no-default-features --features postgres \
    && chown -R vscode:rustlang /usr/local/cargo/registry
