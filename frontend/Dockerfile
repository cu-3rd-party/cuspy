FROM oven/bun:1 AS builder

WORKDIR /app

COPY package.json package-lock.json* bun.lock* ./
RUN bun install

COPY . .

RUN bun run build

FROM oven/bun:1-slim

WORKDIR /app

COPY package.json package-lock.json* bun.lock* ./
RUN bun install --production

COPY --from=builder /app/build ./build

EXPOSE 4173

CMD ["bun", "./build/index.js"]
