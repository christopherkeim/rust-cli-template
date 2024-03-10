# syntax=docker/dockerfile:1

###############################################################
# Builder: builds a statically-linked Rust release binary
###############################################################
FROM rust:1.76-alpine as build

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static 

WORKDIR /app

# Copy dependencies
COPY . .

# Build binary
RUN cargo build --release


###############################################################
# Final
###############################################################

# Copy artifacts to a clean image
FROM scratch

COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

COPY --from=build /app/target/release/rust_cli_template .

ENTRYPOINT [ "./rust_cli_template" ]