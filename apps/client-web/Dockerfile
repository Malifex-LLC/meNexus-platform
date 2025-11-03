FROM node:20-alpine

# Enable pnpm via Corepack and align with repo's packageManager version
RUN corepack enable && corepack prepare pnpm@10.20.0 --activate

WORKDIR /workspace

EXPOSE 5173

# Install workspace deps and run Vite dev server for client-web
CMD ["sh", "-lc", "pnpm install && pnpm -C apps/client-web dev --host 0.0.0.0 --port 5173"]


