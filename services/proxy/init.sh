#!/usr/bin/env bash
set -euo pipefail

PUB_URL="${PUBLIC_URL:-http://localhost}"
EMAIL="${EMAIL:-admin@example.com}"

scheme="$(echo "$PUB_URL" | sed -E 's#^([a-z]+)://.*#\1#')"
# drop scheme, path, and any :port
DOMAIN="$(echo "$PUB_URL" | sed -E 's#^[a-z]+://##; s#/.*$##; s#:[0-9]+$##')"

is_localhost() { [[ -z "$DOMAIN" || "$DOMAIN" == "localhost" || "$DOMAIN" == "127.0.0.1" || "$DOMAIN" == "::1" ]]; }
is_ip()        { [[ "$DOMAIN" =~ ^[0-9.]+$ || "$DOMAIN" =~ : ]]; }  # IPv4 or IPv6

wait_for_client() {
  # Wait until the client container responds (API can come up later)
  for i in {1..60}; do
    if curl -fsS http://client:80/ >/dev/null 2>&1; then
      return 0
    fi
    echo "[proxy] waiting for client container..."
    sleep 1
  done
  echo "[proxy] warning: client not responding yet; continuing"
}

start_nginx_http() {
  echo "[proxy] starting Nginx in HTTP-only mode"
  cp /etc/nginx/templates/nginx.http.conf.template /etc/nginx/nginx.conf
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
  cp /etc/nginx/templates/nginx.http.conf.template /etc/nginx/nginx.conf
  nginx

  certbot certonly --agree-tos --no-eff-email --email "${EMAIL}" \
    --webroot -w /var/www/certbot \
    -d "${DOMAIN}" --non-interactive

  nginx -s stop
}

start_nginx_https() {
  echo "[proxy] starting Nginx in HTTPS mode for ${DOMAIN}"
  sed -e "s/\${DOMAIN}/${DOMAIN}/g" \
      /etc/nginx/templates/nginx.https.conf.template \
      > /etc/nginx/nginx.conf

  # background renew loop
  ( while true; do
      sleep 12h
      certbot renew --quiet --deploy-hook "nginx -s reload"
    done ) &

  exec nginx -g 'daemon off;'
}

main() {
  if is_localhost || is_ip || [[ "$scheme" != "https" ]]; then
    if [[ "$scheme" == "http" && ! ( "$DOMAIN" == "localhost" || "$DOMAIN" == "127.0.0.1" || "$DOMAIN" == "::1" ) ]]; then
      echo "[proxy][WARN] PUBLIC_URL is http on a non-localhost host ($DOMAIN)."
      echo "[proxy][WARN] Browsers will treat this as insecure; WebCrypto will be unavailable and login will fail."
    fi
    echo "[proxy] PUBLIC_URL='${PUB_URL}' -> HTTP-only (local/IP/plain-http)"
    wait_for_client
    start_nginx_http
  else
    echo "[proxy] PUBLIC_URL='${PUB_URL}' -> ACME domain mode"
    issue_cert_if_needed
    start_nginx_https
  fi
}
main
