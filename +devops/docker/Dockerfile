FROM node:24-bookworm-slim AS frontend-builder

WORKDIR /build/frontend

RUN corepack enable

COPY frontend/package.json frontend/pnpm-lock.yaml ./
RUN pnpm install --no-frozen-lockfile

COPY frontend/ ./
RUN pnpm exec vp build


FROM rust:1.93-bookworm AS backend-builder

WORKDIR /build/backend

COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/migrations ./migrations
COPY backend/src ./src
RUN cargo build --release --bin backend --bin migrate


FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libgcc-s1 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend-builder /build/backend/target/release/backend /usr/local/bin/backend
COPY --from=backend-builder /build/backend/target/release/migrate /usr/local/bin/migrate
COPY --from=frontend-builder /build/frontend/build /app/static
COPY backend/migrations /app/migrations
COPY +devops/docker/entrypoint.sh /usr/local/bin/entrypoint.sh

RUN chmod +x /usr/local/bin/entrypoint.sh

ENV APP_HOST=0.0.0.0 \
    APP_PORT=4000 \
    STATIC_DIR=/app/static

EXPOSE 4000

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
CMD ["/usr/local/bin/backend"]
