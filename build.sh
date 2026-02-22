#!/bin/bash
# Vercel build: switch default features from postgres to vercel (which includes sqlite)
# The vercel-rust builder runs `cargo build --bin handler` without --features,
# so we need the vercel feature to be the default for deployment builds.
sed -i 's/default = \["postgres"\]/default = ["vercel"]/' Cargo.toml

# SQLx offline mode — no database available during Vercel builds.
# Set via .cargo/config.toml so it persists into the cargo build process.
mkdir -p .cargo
cat > .cargo/config.toml << 'EOF'
[env]
SQLX_OFFLINE = "true"
EOF
