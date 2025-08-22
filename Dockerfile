# --- build stage ---
FROM rust:1.82-slim AS build

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates pkg-config openssl libssl-dev binaryen && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
RUN cargo install perseus-cli --locked && cargo install wasm-bindgen-cli --locked

WORKDIR /app
COPY . .
RUN perseus deploy && chmod +x ./pkg/server

# --- runtime stage ---
FROM archlinux:latest

RUN mkdir -p /etc/ssl/certs
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

WORKDIR /app
COPY --from=build --chown=65534:65534 /app/pkg /app/pkg
USER 65534:65534

ENV PERSEUS_HOST=0.0.0.0
ENV PERSEUS_PORT=3000
EXPOSE 3000

CMD ["sh","-c","PERSEUS_HOST=0.0.0.0 PERSEUS_PORT=3000 exec /app/pkg/server"]
