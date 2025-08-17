FROM rust:1.79-slim

RUN apt-get update
RUN apt-get install -y --no-install-recommends \
    ca-certificates pkg-config openssl libssl-dev binaryen
RUN rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN cargo install perseus-cli --locked
RUN cargo install wasm-bindgen-cli --locked

WORKDIR /app
COPY . .

RUN perseus deploy

ENV PERSEUS_HOST=0.0.0.0
ENV PERSEUS_PORT=3000
EXPOSE 3000 

CMD ["sh", "-c", "PERSEUS_HOST=0.0.0.0 PERSEUS_PORT=${PORT:-3000} exec ./pkg/server"]
