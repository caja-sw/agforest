FROM node:24-trixie AS builder
WORKDIR /app
COPY . ./
RUN npm ci && npm run build

FROM node:24-trixie
WORKDIR /app
COPY --from=builder /app/build /app/package.json /app/package-lock.json ./
RUN npm ci --omit=dev
CMD ["node", "."]
