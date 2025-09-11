#!/usr/bin/env bash
set -x
set -eo pipefail

# Check if the pre-requisites executables have been installed
# $(some_command) is command substitution. shell runs some_command, captures its standard output, and replaces the whole $( … ) with that output
# [ condition ] checks condition, returns 0 if true, non-zero if false
# [ -x file ] returns 0 if file exists and you have permission to execute it, else non-zero
if ! [ -x "$(command -v psql)" ]; then
    echo "Error: psql is not installed." >&2
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then 
    echo "Error: sqlx is not installed." >&2
    exit 1
fi

# Check if a custom value has been set, otherwise default to its default value
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"

# Launch postgres using Docker
# Allow to skip Docker if a dockerized Postgres database is already running
# [[ ... ]] is like [ ... ] but modern, actually should use this instead
# -z returns true if string is empty, false otherwise
if [[ -z "${SKIP_DOCKER}" ]]; then
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d postgres \
        postgres -N 1000
fi

# Keep pinging Postgres until it's ready to accept commands
# `until` runs the COMMANDS repeatedly until they succeed. "Success"
# in this case means that the command exits with status code 0, which 
# psql will after it manages to connect.
export PGPASSWORD=${DB_PASSWORD} # this is required for psql to connect to the DB in order to execute whatever we put after -c, which is quit
until psql -h "${DB_HOST}" -p "${DB_PORT}" -U "${DB_USER}" -d "postgres" -c '\q'; do
    echo "Postgres is still unavailable - sleeping" >&2
    sleep 1
done

echo "Postgres is up and running on port ${DB_PORT} - running migrations now!" >&2 

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run

echo "Postgres has been migrated, ready to go!" >&2