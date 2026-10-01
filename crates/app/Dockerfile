# syntax = docker/dockerfile:1
ARG BUN_VERSION=1.3.14
FROM oven/bun:${BUN_VERSION}-slim AS base
WORKDIR /app
ENV NODE_ENV="production"

FROM base AS build
COPY bun.lock package.json ./
RUN bun install --frozen-lockfile
COPY . .
RUN bun run build

FROM base
COPY --from=build /app/.output /app/.output
ENV PORT=3000
EXPOSE 3000
CMD [ "bun", ".output/server/index.mjs" ]
