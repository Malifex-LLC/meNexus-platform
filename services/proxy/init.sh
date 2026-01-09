#!/usr/bin/env bash
set -euo pipefail

# Configuration from environment
SYNAPSE_HOST="${SYNAPSE_HOST:-synapse}"
SYNAPSE_PORT="${SYNAPSE_PORT:-3000}"
DOMAIN="${DOMAIN:-localhost}"
EMAIL="${LETSENCRYPT_EMAIL:-admin@example.com}"
ENABLE_SSL="${ENABLE_SSL:-false}"

export SYNAPSE_HOST SYNAPSE_PORT DOMAIN

is_localhost() { [[ -z "$DOMAIN" || "$DOMAIN" == "localhost" || "$DOMAIN" == "127.0.0.1" || "$DOMAIN" == "::1" ]]; }
is_ip()        { [[ "$DOMAIN" =~ ^[0-9.]+$ || "$DOMAIN" =~ : ]]; }  # IPv4 or IPv6

wait_for_synapse() {
  echo "[proxy] waiting for synapse at ${SYNAPSE_HOST}:${SYNAPSE_PORT}..."
  for i in {1..120}; do
    if curl -fsS "http://${SYNAPSE_HOST}:${SYNAPSE_PORT}/" >/dev/null 2>&1; then
      echo "[proxy] synapse is ready"
      return 0
    fi
    sleep 1
  done
  echo "[proxy] warning: synapse not responding yet; starting anyway"
}

render_template() {
  local template="$1"
  local output="$2"
  envsubst '${SYNAPSE_HOST} ${SYNAPSE_PORT} ${DOMAIN}' < "$template" > "$output"
}

start_nginx_http() {
  echo "[proxy] starting Nginx in HTTP-only mode"
  render_template /etc/nginx/templates/nginx.http.conf.template /etc/nginx/nginx.conf
  exec nginx -g 'daemon off;'
}

issue_cert_if_needed() {
  local cert_dir="/etc/letsencrypt/live/${DOMAIN}"
  if [[ -f "${cert_dir}/fullchain.pem" && -f "${cert_dir}/privkey.pem" ]]; then
    echo "[proxy] certificate already exists for ${DOMAIN}"
    return 0
  fi

  echo "[proxy] obtaining certificate for ${DOMAIN} via HTTP-01"
  # Serve ACME challenge over HTTP temporarily
  render_template /etc/nginx/templates/nginx.http.conf.template /etc/nginx/nginx.conf
  nginx

  certbot certonly --agree-tos --no-eff-email --email "${EMAIL}" \
    --webroot -w /var/www/certbot \
    -d "${DOMAIN}" --non-interactive

  nginx -s stop
}

start_nginx_https() {
  echo "[proxy] starting Nginx in HTTPS mode for ${DOMAIN}"
  render_template /etc/nginx/templates/nginx.https.conf.template /etc/nginx/nginx.conf

  # background renew loop
  ( while true; do
      sleep 12h
      certbot renew --quiet --deploy-hook "nginx -s reload"
    done ) &

  exec nginx -g 'daemon off;'
}

main() {
  wait_for_synapse
  
  if [[ "$ENABLE_SSL" == "true" ]] && ! is_localhost && ! is_ip; then
    echo "[proxy] SSL enabled for domain: ${DOMAIN}"
    issue_cert_if_needed
    start_nginx_https
  else
    if [[ "$ENABLE_SSL" == "true" ]]; then
      echo "[proxy][WARN] SSL requested but domain is localhost/IP - using HTTP"
    fi
    echo "[proxy] Starting in HTTP-only mode"
    start_nginx_http
  fi
}
main
