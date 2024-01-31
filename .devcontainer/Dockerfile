FROM ubuntu:jammy

RUN apt-get update \
  && apt-get -y install \
  --no-install-recommends \
  curl=7.81.0-1ubuntu1.15 \
  ca-certificates=20230311ubuntu0.22.04.1  \
  && rm -rf /var/lib/apt/lists/*


RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN /root/.cargo/bin/rustup default  nightly

RUN /root/.cargo/bin/rustup target add wasm32-unknown-unknown

RUN apt-get update && apt-get -y install build-essential

RUN /root/.cargo/bin/cargo install --locked cargo-leptos

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
