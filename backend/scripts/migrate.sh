#!/usr/bin/env sh

set -eu

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
MIGRATIONS_DIR="$ROOT_DIR/migrations"

if [ -f "$ROOT_DIR/.env" ]; then
  set -a
  . "$ROOT_DIR/.env"
  set +a
fi

if [ -z "${DATABASE_URL:-}" ]; then
  printf '%s\n' 'DATABASE_URL is required' >&2
  exit 1
fi

mkdir -p "$MIGRATIONS_DIR"

exec cargo run --bin migrate -- "$DATABASE_URL" "$MIGRATIONS_DIR"
