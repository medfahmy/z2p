#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "psql is not installed"
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "sqlx is not installed"
    echo >&2 "cargo install --version='~0.7' sqlx-cli --no-default-features --features rustls,postgres"
    exit 1
fi


DB_USER="${POSTGRES_USER:=postgres}"
DB_PASS="${POSTGRES_PASS:=postgres}"
DB_NAME="${POSTGRES_DB:=tempo}"
DB_HOST="${POSTGRES_HOST:=localhost}"
DB_PORT="${POSTGRES_PORT:=5432}"
NB_CONN="${NB_CONN:=1000}"

if [[ -z "${SKIP_DOCKER}" ]]; then
    if ! [ -x docker ps -f name=postgres ]; then
        docker stop postgres
        docker rm postgres
    fi

    docker run -d \
        --name postgres \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASS} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        postgres -N "${NB_CONN}"

    export PGPASSWORD="${DB_PASS}"
    until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
        # >&2 echo "postgres is still unavailable"
        sleep 2
    done
fi

# >&2 echo "postgres is up and running on port ${DB_PORT}"


DATABASE_URL=postgres://${DB_USER}:${DB_PASS}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run
