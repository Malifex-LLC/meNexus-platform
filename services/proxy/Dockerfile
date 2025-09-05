FROM nginx:1.29.1-alpine

# Tools for bootstrapping + trust store
RUN apk add --no-cache bash curl grep sed coreutils certbot ca-certificates

# Ensure certbot dir exists
RUN mkdir -p /var/www/certbot

# Nginx templates + init
COPY nginx/nginx.http.conf.template /etc/nginx/templates/nginx.http.conf.template
COPY nginx/nginx.https.conf.template /etc/nginx/templates/nginx.https.conf.template
COPY init.sh /usr/local/bin/init.sh

EXPOSE 80 443
ENTRYPOINT [ "bash", "/usr/local/bin/init.sh" ]
