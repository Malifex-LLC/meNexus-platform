#!/bin/sh
set -eu

ROOT_PW="$(cat /run/secrets/DB_ROOT_PW)"
APP_PW="$(cat /run/secrets/DB_USER_PW)"
ADMIN_PW="$(cat /run/secrets/DB_ADMIN_PW)"

mysql -uroot -p"$ROOT_PW" <<'SQL'
-- ensure schema exists (it should from MYSQL_DATABASE)
CREATE DATABASE IF NOT EXISTS `menexus_schema` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
SQL

mysql -uroot -p"$ROOT_PW" <<SQL
-- app user: DML only on this schema
CREATE USER IF NOT EXISTS 'menexus_user'@'%' IDENTIFIED BY '${APP_PW}';
GRANT SELECT, INSERT, UPDATE, DELETE
    ON \`menexus_schema\`.*
    TO 'menexus_user'@'%';

-- admin user: DDL + DML on this schema (not global)
CREATE USER IF NOT EXISTS 'menexus_admin'@'%' IDENTIFIED BY '${ADMIN_PW}';
GRANT CREATE, ALTER, DROP, INDEX, CREATE VIEW, SHOW VIEW, EVENT, TRIGGER,
    SELECT, INSERT, UPDATE, DELETE
    ON \`menexus_schema\`.*
    TO 'menexus_admin'@'%';

FLUSH PRIVILEGES;
SQL
