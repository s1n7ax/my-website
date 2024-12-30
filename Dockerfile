#--------------------------------------------------------------------#
#                            build image                             #
#--------------------------------------------------------------------#
FROM rustlang/rust:nightly-alpine AS builder

ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV PATH="/usr/local/cargo/bin:$PATH"


RUN apk update && \
  apk add --no-cache bash curl npm libc-dev binaryen musl-dev pnpm

# RUN wget -qO- https://get.pnpm.io/install.sh | sh -

SHELL [ "/bin/bash", "-exo", "pipefail", "-c" ]

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown \
  && cargo install stylance-cli

WORKDIR /work
COPY . .

RUN npm install \
  && npm run build

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
