#--------------------------------------------------------------------#
#                            build image                             #
#--------------------------------------------------------------------#
FROM rustlang/rust:nightly-alpine AS builder

ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV PATH="/usr/local/cargo/bin:$PATH"


RUN apk update && \
  apk add --no-cache bash curl libc-dev binaryen musl-dev pnpm

SHELL [ "/bin/bash", "-exo", "pipefail", "-c" ]

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown \
  && cargo install stylance-cli

RUN cargo install --locked cargo-leptos@0.2.24

WORKDIR /work
COPY . .

RUN pnpm install \
  && pnpm build

#--------------------------------------------------------------------#
#                          deployment image                          #
#--------------------------------------------------------------------#
FROM busybox:stable AS runner

WORKDIR /app

COPY --from=builder /work/target/release/my-website /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

EXPOSE 3000

ENV LEPTOS_SITE_ROOT=./site

CMD ["/app/my-website"]
