# syntax=docker/dockerfile:1.7
# Go stub service — replace with a real Go agent implementation.

FROM golang:1.23-bookworm AS build
WORKDIR /app
COPY go.mod ./
RUN go mod download
COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -trimpath -ldflags="-s -w" -o go-service .

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends wget && \
    rm -rf /var/lib/apt/lists/*
COPY --from=build /app/go-service /usr/local/bin/go-service
ENV GO_SERVICE_PORT=8080
EXPOSE 8080
ENTRYPOINT ["go-service"]
