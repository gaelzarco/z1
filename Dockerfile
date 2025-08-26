# --- build stage ---
FROM --platform=$BUILDPLATFORM rust:1.82-slim AS build
ARG BINARYEN_VERSION=123

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates pkg-config openssl libssl-dev curl tar xz-utils && \
    rm -rf /var/lib/apt/lists/*

# Install recent ver of wasm-opt (Bookworm's triggers "failed to grow table")
RUN curl -fsSL \
  -o /tmp/binaryen.tgz \
  https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
  && mkdir -p /opt/binaryen \
  && tar -xzf /tmp/binaryen.tgz -C /opt/binaryen --strip-components=1 \
  && ln -s /opt/binaryen/bin/wasm-opt /usr/local/bin/wasm-opt

RUN rustup target add wasm32-unknown-unknown
RUN cargo install perseus-cli --locked && cargo install wasm-bindgen-cli --locked

WORKDIR /app
COPY . .

RUN perseus deploy && strip ./pkg/server && chmod +x ./pkg/server

# --- runtime stage ---
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 10001 -s /usr/sbin/nologin appuser
WORKDIR /app
COPY --from=build /app/pkg /app/pkg
COPY --from=build /app/translations /app/translations

USER appuser
EXPOSE 3000

CMD ["sh","-c","PERSEUS_HOST=0.0.0.0 PERSEUS_PORT=3000 exec /app/pkg/server"]
