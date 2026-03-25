#!/bin/sh
set -eu

if [ -z "${DATABASE_URL:-}" ]; then
  echo "DATABASE_URL must be set" >&2
  exit 1
fi

/usr/local/bin/migrate "$DATABASE_URL" /app/migrations

exec "$@"
