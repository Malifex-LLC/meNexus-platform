FROM nginx:1.29.1-alpine

# ---- OCI labels ----
ARG VERSION
ARG VCS_REF
ARG BUILD_DATE

LABEL org.opencontainers.image.title="meNexus Proxy" \
      org.opencontainers.image.description="Nginx proxy for meNexus" \
      org.opencontainers.image.url="https://github.com/malifex-llc/menexus-platform" \
      org.opencontainers.image.source="https://github.com/malifex-llc/menexus-platform" \
      org.opencontainers.image.documentation="https://github.com/malifex-llc/menexus-platform#readme" \
      org.opencontainers.image.licenses="AGPL-3.0-or-later" \
      org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.revision="${VCS_REF}" \
      org.opencontainers.image.created="${BUILD_DATE}"

# Tools for bootstrapping + trust store + envsubst for template rendering
RUN apk add --no-cache bash curl grep sed coreutils certbot ca-certificates gettext

# Ensure certbot dir exists
RUN mkdir -p /var/www/certbot

# Nginx templates + init
COPY nginx/nginx.http.conf.template /etc/nginx/templates/nginx.http.conf.template
COPY nginx/nginx.https.conf.template /etc/nginx/templates/nginx.https.conf.template
COPY init.sh /usr/local/bin/init.sh

EXPOSE 80 443
ENTRYPOINT [ "bash", "/usr/local/bin/init.sh" ]
