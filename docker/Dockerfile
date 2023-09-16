FROM debian:buster-slim

RUN apt-get update && apt-get install -y curl build-essential

# Install rustup
ENV RUSTUP_HOME=/opt/rustup
ENV CARGO_HOME=/opt/cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="${PATH}:/opt/cargo/bin"

ENV RUST_TOOLCHAIN="stable"

RUN rustup toolchain install "${RUST_TOOLCHAIN}"
RUN rustup default "${RUST_TOOLCHAIN}"

RUN rustup component add clippy
RUN rustup component add rustfmt

RUN curl -fsSL https://download.docker.com/linux/debian/gpg | apt-key add -
RUN apt-get install -y software-properties-common apt-transport-https ca-certificates curl gnupg2
RUN add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/debian buster stable"
RUN apt-get update
RUN apt-get install -y docker-ce docker-ce-cli containerd.io

RUN cargo install --git="https://github.com/cross-rs/cross.git" --branch="main" cross
