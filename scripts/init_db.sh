#!/usr/bin/env bash
set -x
set -eo pipefail

# Check for dependencies
if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    exit 1
fi

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_HOST="${POSTGRES_HOST:=localhost}"
DB_PORT="${POSTGRES_PORT:=5432}"

# Launch Postgres using Docker
if [[ -z "${SKIP_DOCKER}" ]]
then
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}:5432" \
        -d \
        postgres \
        -N 1000 # increased number of connections for testing purposes
fi

# Keep pinging Postgres until it's ready to accept connections
export PGPASSWORD=${DB_PASSWORD} # env var for psql to read from
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}"

# sqlx create database
DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create # this is idempotent
sqlx migrate run

>&2 echo "Posgres has been migrated, ready to go!"