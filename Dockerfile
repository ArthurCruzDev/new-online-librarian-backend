FROM lukemathwalker/cargo-chef:latest-rust-1.83 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y
FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .

ENV SQLX_OFFLINE=true
# Build our project
RUN cargo build --release --bin new-online-librarian-backend

# our final base
FROM ubuntu:22.04 AS runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# copy the build artifact from the build stage
COPY --from=builder /app/target/release/new-online-librarian-backend new-online-librarian-backend

# copy configurations
COPY configuration configuration

#ENVS
ENV APP_ENVIRONMENT=production

# set the startup command to run your binary
CMD ["./new-online-librarian-backend"]