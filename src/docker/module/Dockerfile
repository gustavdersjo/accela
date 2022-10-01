FROM rust

# Set shell
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

ENV DEBIAN_FRONTEND="noninteractive"

ENV channel="stable"

RUN apt-get update && apt-get install -y \
    curl \
    gcc \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m app -d /app
RUN usermod -p '!!' root  # Disable all passwords for root
USER app
ENV USER=app
ENV PATH=/app/.cargo/bin:$PATH
WORKDIR /app

#RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain "${channel}"

#RUN cargo install cargo-prefetch && cargo prefetch --top-downloads=50

ADD --chown=app entrypoint.sh /app/
RUN chmod 775 /app/entrypoint.sh
RUN mkdir -p /app/bin/

ENTRYPOINT ["/app/entrypoint.sh"]