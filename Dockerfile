# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine as builder

# Install system dependencies first (cached layer)
RUN apk update && \
    apk add --no-cache \
    pkgconfig \
    openssl-dev \
    openssl-libs-static \
    bash \
    curl \
    npm \
    libc-dev \
    binaryen \
    build-base \
    musl-dev && \
    rm -rf /var/cache/apk/*

# Install global tools (cached layer)
RUN npm install -g sass && \
    curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh && \
    rustup target add wasm32-unknown-unknown

# Set up working directory
WORKDIR /work

# Copy only dependency files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY .cargo/ ./.cargo/

# Create a dummy main.rs to satisfy cargo build for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Set OpenSSL environment variables
ENV OPENSSL_STATIC=1
ENV PKG_CONFIG_PATH=/usr/lib/pkgconfig
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl

# Build dependencies only (this layer will be cached)
RUN cargo build --release
RUN rm -rf src/

# Now copy the actual source code
COPY . .

# Build the application (only this runs when source changes)
RUN cargo leptos build --release

# Runtime stage - use distroless or minimal base
FROM alpine:latest as runner

# Install only runtime dependencies
RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy only the necessary files
COPY --from=builder /work/target/release/supabasemm /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

# Set environment variables
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:10000"
ENV LEPTOS_SITE_ROOT=./site

EXPOSE 10000

# Run the application
CMD ["/app/supabasemm"]
