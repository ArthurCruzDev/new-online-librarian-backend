FROM rust:1.83

# create a new empty shell project
RUN USER=root cargo new --bin onlinelibrarian-backend
WORKDIR /onlinelibrarian-backend

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release && rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/onlinelibrarian-backend*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /onlinelibrarian-backend/target/release/onlinelibrarian-backend .

# set the startup command to run your binary
CMD ["./onlinelibrarian-backend"]