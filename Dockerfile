FROM rustlang/rust:nightly-alpine as builder

RUN apk update && \
  apk add --no-cache bash=5.2.15-r0 curl=8.5.0-r0 npm=9.1.2-r0 libc-dev=0.7.2-r3 binaryen=110-r0

RUN npm install -g sass@1.70.0

SHELL [ "/bin/bash", "-exo", "pipefail", "-c" ]

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo leptos build --release -vv

FROM scratch as runner

WORKDIR /app

COPY --from=builder /work/target/release/my-website /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

EXPOSE $PORT
ENV LEPTOS_SITE_ROOT=./site

CMD ["/app/my-website"]
