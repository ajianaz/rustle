# rustle

Minimal TCP stream proxy — "hanya lewat saja."

Routes any TCP traffic from a listening port to a target address. Protocol-agnostic: HTTP, SMTP, IMAP, WebSocket — anything.

## Usage

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `LISTEN_PORT` | `8080` | Port to listen on |
| `TARGET_ADDR` | `127.0.0.1:8081` | Target `host:port` to forward to |

### Docker

```bash
docker run -d \
  -e LISTEN_PORT=32145 \
  -e TARGET_ADDR=stalwart:8080 \
  -p 32145:32145 \
  ghcr.io/ajianaz/rustle:latest
```

### Docker Compose (Traefik → rustle → backend)

```yaml
services:
  rustle:
    build: .
    environment:
      - LISTEN_PORT=32145
      - TARGET_ADDR=stalwart:8080
    networks:
      - proxy

  # Traefik labels point to rustle instead of backend directly
  stalwart:
    image: stalwart:latest
    networks:
      - proxy
```

### Local

```bash
cargo run -- -e LISTEN_PORT=32145 TARGET_ADDR=localhost:8080
```

## Why?

Traefik can sometimes have issues with certain backends (SMTP, IMAP, custom protocols). Rustle sits in between as a dumb TCP pipe — no parsing, no inspection, just forwarding bytes.

- ~5MB binary (release, stripped)
- ~1MB RAM idle
- Zero protocol awareness = zero protocol bugs

## License

MIT
