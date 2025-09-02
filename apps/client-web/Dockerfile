# syntax=docker/dockerfile:1

############################
# Build deps with pnpm workspaces
############################
FROM node:20-alpine AS build
WORKDIR /client
RUN corepack enable && corepack prepare pnpm@9.0.0 --activate

# Build-time configuration for Vite
ARG VITE_API_BASE_URL
ARG VITE_WS_BASE_URL
ENV VITE_API_BASE_URL=$VITE_API_BASE_URL
ENV VITE_WS_BASE_URL=$VITE_WS_BASE_URL

# Copy only client manifests first for better caching
COPY apps/client-web/package.json apps/client-web/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

# Copy client source
COPY apps/client-web/ .
RUN pnpm build

############################
# Runtime: Nginx serving static SPA + proxy to Synapse
############################
FROM nginx:alpine
# SPA assets
COPY --from=build /client/dist /usr/share/nginx/html
# Nginx config (adds history fallback + /api + /uploads proxy)
COPY apps/client-web/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
