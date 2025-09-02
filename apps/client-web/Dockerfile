# syntax=docker/dockerfile:1

############################
# Build via pnpm workspace
############################
FROM node:20-alpine AS build
WORKDIR /repo
RUN corepack enable && corepack prepare pnpm@9.0.0 --activate

# Build-time configuration for Vite
ARG VITE_API_BASE_URL=""
ARG VITE_WS_BASE_URL="/ws"
ENV VITE_API_BASE_URL=${VITE_API_BASE_URL}
ENV VITE_WS_BASE_URL=${VITE_WS_BASE_URL}

# Copy workspace manifests and lockfile from the repo root
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
# Copy the app's package.json so pnpm can resolve its dependencies
COPY apps/client-web/package.json apps/client-web/package.json

# Install ALL workspace deps using the single root lockfile
RUN pnpm install --frozen-lockfile

# Bring in the rest of the repo (source code)
COPY . .

# Build ONLY the client web app (filter by path)
RUN pnpm --filter ./apps/client-web build

############################
# Runtime: Nginx serving static SPA + proxy to Synapse
############################
FROM nginx:alpine
# Copy the built SPA
COPY --from=build /repo/apps/client-web/dist /usr/share/nginx/html
# Nginx config (history fallback + /api + /remote + /uploads + /ws proxy)
COPY apps/client-web/src/config/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
