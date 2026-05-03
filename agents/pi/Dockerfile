# syntax=docker/dockerfile:1.7
# Pi Coding Agent — minimal terminal coding harness
# Source: https://github.com/badlogic/pi-mono/tree/main/packages/coding-agent
#
# Telemetry: PI_TELEMETRY=0 is set at runtime via docker-compose to disable
# the anonymous install-version ping to pi.dev/api/report-install.

FROM node:22-bookworm-slim AS build

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        python3 make g++ git && \
    rm -rf /var/lib/apt/lists/*

# Copy package manifests first for layer-cached installs
COPY package.json ./

# Install all dependencies (including devDependencies needed for the build)
RUN npm install --prefer-offline --no-audit

# Copy source code
COPY . .

# Build the TypeScript sources
RUN npm run build

# ── Runtime stage ──────────────────────────────────────────────────────────────
FROM node:22-bookworm-slim AS runtime

WORKDIR /app

# Runtime system deps: git is needed for bash tool operations
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        git tini && \
    rm -rf /var/lib/apt/lists/*

# Copy built artefacts and runtime node_modules from build stage
COPY --from=build /app/dist ./dist
COPY --from=build /app/node_modules ./node_modules
COPY --from=build /app/package.json ./package.json

# Non-root user; pi writes sessions to /home/pi/.pi
RUN useradd -u 10001 -m -d /home/pi pi
USER pi

ENV NODE_ENV=production
# Disable anonymous install-version ping — see agents/pi/src/core/telemetry.ts
ENV PI_TELEMETRY=0

VOLUME ["/home/pi/.pi"]

ENTRYPOINT ["/usr/bin/tini", "--", "node", "dist/cli.js"]
