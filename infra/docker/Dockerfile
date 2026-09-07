FROM node:20-bookworm AS frontend-builder
WORKDIR /app
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
COPY apps/web/package.json apps/web/
COPY packages/ packages/
RUN corepack enable && pnpm install --frozen-lockfile
COPY apps/web/ apps/web/
RUN pnpm --filter @paugeran/web build

FROM rust:1.75-bookworm AS backend-builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY apps/server/ apps/server/
COPY --from=frontend-builder /app/apps/web/dist apps/web/dist
RUN cargo build --release --bin paugeran

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*
COPY --from=backend-builder /app/target/release/paugeran /usr/local/bin/paugeran
RUN useradd -m paugeran && mkdir -p /data && chown -R paugeran:paugeran /data
ENV DATA_DIR=/data PORT=3000
EXPOSE 3000
USER paugeran
HEALTHCHECK CMD curl --fail http://127.0.0.1:3000/health || exit 1
ENTRYPOINT ["/usr/local/bin/paugeran"]
