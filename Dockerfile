# --- build stage ---
FROM rust:1.79-slim AS build

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates pkg-config openssl libssl-dev binaryen && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
RUN cargo install perseus-cli --locked && cargo install wasm-bindgen-cli --locked

WORKDIR /app
COPY . .

RUN perseus deploy && chmod +x ./pkg/server

# --- runtime stage ---
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
RUN useradd -m -u 10001 -s /usr/sbin/nologin appuser

WORKDIR /app
COPY --from=build /app/pkg /app/pkg

USER appuser

EXPOSE 3000

CMD ["sh","-c","PERSEUS_HOST=0.0.0.0 PERSEUS_PORT=3000 exec /app/pkg/server"]
