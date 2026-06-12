# syntax=docker/dockerfile:1
ARG BUN_VERSION=1.3.14

FROM oven/bun:${BUN_VERSION}-alpine AS builder
WORKDIR /home/bun/app

COPY package.json bun.lock ./
RUN bun install --frozen-lockfile

COPY . .
RUN bun run build

FROM oven/bun:${BUN_VERSION}-alpine AS production
WORKDIR /home/bun/app

ENV NODE_ENV=production
ENV HOST=0.0.0.0
ENV PORT=3000

COPY --from=builder --chown=bun:bun /home/bun/app/build ./

RUN chown -R bun:bun /home/bun/app

USER bun
EXPOSE 3000

CMD ["bun", "./index.js"]