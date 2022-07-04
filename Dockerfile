FROM rust:1.62.0 AS builder

RUN rustup target add x86_64-unknown-linux-musl
# hadolint ignore=DL3008
RUN apt-get update && \
  apt-get install -y musl-tools musl-dev --no-install-recommends && \
  update-ca-certificates

# Create user for app
ENV USER=app
ENV UID=10001
ENV GROUP=app
ENV GID=10001

RUN groupadd \
  -g "${GID}" \
  ${GROUP} && \
  adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  --gid "${GID}" \
  "${USER}"


WORKDIR /app

COPY ./Cargo.* ./
# Cache dependencies by writing a dummy main file and building it
RUN mkdir -p src && \
  sed -i 's#src/main.rs#dummy.rs#' Cargo.toml && \
  echo "fn main() {}" > dummy.rs && \
  cargo build --target x86_64-unknown-linux-musl --release && \
  sed -i 's#dummy.rs#src/main.rs#' Cargo.toml && \
  rm dummy.rs

COPY ./ ./

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.15 as release

# Add Tini
RUN apk add --no-cache tini=0.19.0-r0

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/paste ./

USER ${USER}:${GROUP}

EXPOSE 80

ENV RUST_LOG=info
ENV SERVER_HOST=0.0.0.0

CMD ["/sbin/tini", "--"]

ENTRYPOINT [ "/app/paste" ]
