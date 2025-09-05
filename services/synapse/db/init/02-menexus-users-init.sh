#!/bin/sh
set -eu

# Provided by the mysql image even when using *_FILE:
ROOT_PW="${MYSQL_ROOT_PASSWORD:?MYSQL_ROOT_PASSWORD not set}"

# These two are still files; mysql user must be able to read them
APP_PW="$(cat /run/secrets/DB_USER_PW)"
ADMIN_PW="$(cat /run/secrets/DB_ADMIN_PW)"

mysql -uroot -p"$ROOT_PW" <<'SQL'
CREATE DATABASE IF NOT EXISTS `menexus_schema`
  CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
SQL

mysql -uroot -p"$ROOT_PW" <<SQL
CREATE USER IF NOT EXISTS 'menexus_user'@'%' IDENTIFIED BY '${APP_PW}';
CREATE USER IF NOT EXISTS 'menexus_admin'@'%' IDENTIFIED BY '${ADMIN_PW}';

-- keep in sync if secrets rotate:
ALTER USER 'menexus_user'@'%' IDENTIFIED BY '${APP_PW}';
ALTER USER 'menexus_admin'@'%' IDENTIFIED BY '${ADMIN_PW}';

GRANT SELECT, INSERT, UPDATE, DELETE
  ON \`menexus_schema\`.* TO 'menexus_user'@'%';

GRANT CREATE, ALTER, DROP, INDEX, CREATE VIEW, SHOW VIEW, EVENT, TRIGGER,
       SELECT, INSERT, UPDATE, DELETE
  ON \`menexus_schema\`.* TO 'menexus_admin'@'%';

FLUSH PRIVILEGES;
SQL
