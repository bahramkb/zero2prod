#!/usr/bin/env bash
set -eo pipefail

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "cargo install sqlx-cli"
  echo >&2 "to install it."
  exit 1
fi

# Check if a custom user has been set, otherwise default to 'mysql'
DB_USER=${MYSQL_USER:=app}
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${MYSQL_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${MYSQL_DB:=db}"
# Check if a custom port has been set, otherwise default to '3306'
DB_PORT="${MYSQL_PORT:=3306}"

echo "Setting up MySQL Database..."

echo "Launching MySQL Server..."
if [ "$(docker ps -q -f name=dev-mysql)" ]; then
  echo "MySQL container is already running."
else
  docker run \
    --name dev-mysql \
    --rm \
    -e MYSQL_USER=${DB_USER} \
    -e MYSQL_PASSWORD=${DB_PASSWORD} \
    -e MYSQL_ROOT_PASSWORD=${DB_PASSWORD} \
    -e MYSQL_DATABASE=${DB_NAME} \
    -p "${DB_PORT}":3306 \
    -d mysql:9

  # Wait for the MySQL Server to come up
  until docker exec dev-mysql mysqladmin ping --silent &> /dev/null; do
    echo "$(date +'%H:%M:%S'): Waiting for MySQL Server to become ready..."
    sleep 5
  done

  sleep 10
fi

echo "Checking if the database already exists..."
if docker exec dev-mysql mysql -u${DB_USER} -p${DB_PASSWORD} -e "USE ${DB_NAME}" --silent; then
  echo "Database ${DB_NAME} already exists."
else
  echo "Creating the database..."
  export DATABASE_URL=mysql://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
  sqlx database create
fi

echo "Running migrations..."
sqlx migrate run