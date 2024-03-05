# syntax=docker/dockerfile:1

###############################################################
# Builder
###############################################################
FROM rust:1.76 as build

WORKDIR /app

# Copy dependencies
COPY . .

# Build binary
RUN cargo build --release


###############################################################
# Final
###############################################################

# Copy artifacts to a clean image
FROM debian:stable-slim

RUN apt update && apt install -y openssl ca-certificates

COPY --from=build /app/target/release/rust_cli_template .

ENTRYPOINT [ "./rust_cli_template" ]