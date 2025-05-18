# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine as builder

RUN apk update && \
    apk add --no-cache pkgconfig openssl-dev bash curl npm libc-dev binaryen

RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app

# Install Supabase CLI in the runner image
RUN apk update && \
    apk add --no-cache curl bash && \
    curl -LO "https://github.com/supabase/cli/releases/download/v2.22.12/supabase_2.22.12_linux_amd64.apk" && \
    apk add --allow-untrusted supabase_2.22.12_linux_amd64.apk && \
    rm supabase_2.22.12_linux_amd64.apk

COPY --from=builder /work/target/release/ /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:10000"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 10000

CMD ["/app/supabasemm"]
