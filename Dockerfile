# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine as builder

# Install required build dependencies
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
    musl-dev

# Install Sass
RUN npm install -g sass

# Install cargo-leptos
RUN curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Set up the working directory
WORKDIR /work
COPY . .

# Build your Leptos application with proper OpenSSL configuration
# Static linking of OpenSSL
ENV OPENSSL_STATIC=1
ENV PKG_CONFIG_PATH=/usr/lib/pkgconfig
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl

# Run the build
RUN cargo leptos build --release -vv

# Start with a fresh image for the runner
FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app

# Install Supabase CLI and runtime dependencies in the runner image
RUN apk update && \
    apk add --no-cache \
    curl \
    bash \
    openssl \
    ca-certificates && \
    curl -LO "https://github.com/supabase/cli/releases/download/v2.22.12/supabase_2.22.12_linux_amd64.apk" && \
    apk add --allow-untrusted supabase_2.22.12_linux_amd64.apk && \
    rm supabase_2.22.12_linux_amd64.apk

# Copy built artifacts from builder stage
COPY --from=builder /work/target/release/ /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

# Set environment variables
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:10000"
ENV LEPTOS_SITE_ROOT=./site

EXPOSE 10000

# Run the application
CMD ["/app/supabasemm"]
