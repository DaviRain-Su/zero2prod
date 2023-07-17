#!/usr/bin/env bash

set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi
if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Use:"
    echo >&2 " cargo install sqlx-cli --no-default-features --features native-tls,postgres"
    echo >&2 "to install it."
    exit 1
fi
# The rest of the script


# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

# Launch postgres using Docker
# Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
   # 启动 Postgres 并保存容器 ID
    CONTAINER_ID=$(docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -d postgres \
        postgres -N 1000 \
        -p "${DB_PORT}":5432)

fi

# 获取容器的 IP 地址
CONTAINER_IP=$(docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$CONTAINER_ID")


# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"


# 使用容器的 IP 地址来检查 Postgres 是否可用
until psql -h "$CONTAINER_IP" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done


>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

# 使用容器的 IP 地址来连接到 Postgres
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${CONTAINER_IP}:${DB_PORT}/${DB_NAME}

sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
