FROM node:24-trixie AS builder
COPY . /app
WORKDIR /app
RUN npm ci && npm run build

FROM node:24-trixie
COPY --from=builder /app/build /app
CMD ["node", "/app"]
