#--------------------------------------------------------------------#
#                            build image                             #
#--------------------------------------------------------------------#
FROM nixos/nix:2.18.9 AS builder

ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"

WORKDIR /work
COPY . .
RUN nix --extra-experimental-features 'flakes nix-command' develop \
  --command pnpm install && pnpm build

#--------------------------------------------------------------------#
#                          deployment image                          #
#--------------------------------------------------------------------#
FROM alpine:3.19.1 AS runner

WORKDIR /app

COPY --from=builder /work/target/release/my-website /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

EXPOSE 3000

ENV LEPTOS_SITE_ROOT=./site

CMD ["/app/my-website"]
