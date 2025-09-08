# syntax=docker/dockerfile:1

############################
# Build deps with pnpm workspaces
############################
FROM node:22-alpine AS deps
WORKDIR /repo
RUN corepack enable && corepack prepare pnpm@9.0.0 --activate

# Workspace manifests
COPY pnpm-workspace.yaml package.json pnpm-lock.yaml ./
COPY services/synapse/package.json services/synapse/package.json


# Install all workspace deps
RUN pnpm install --frozen-lockfile

# Copy the whole repo
COPY . .

############################
# Runtime image
############################
FROM node:22-alpine
WORKDIR /app
ENV NODE_ENV=production

# Bring runtime node_modules (workspace + service)
COPY --from=deps /repo/node_modules /app/node_modules
COPY --from=deps /repo/services/synapse/node_modules /app/services/synapse/node_modules

# Copy source
COPY . .

# Entrypoint script
COPY services/synapse/docker/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Persistent data
VOLUME ["/app/services/synapse/src/config", "/synapse/uploads", "/data/secrets"]

# API + libp2p + orbitdb
EXPOSE 3001 4001 4002

ENTRYPOINT ["/entrypoint.sh"]
