FROM node:24-trixie AS builder
COPY . /app
WORKDIR /app
RUN npm ci && npm run build

FROM node:24-trixie-slim
WORKDIR /app
COPY --from=builder /app/build /app/package.json /app/package-lock.json ./
RUN npm ci --omit=dev
CMD ["node", "."]
