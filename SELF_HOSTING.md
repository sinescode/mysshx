# Self-Hosting sshx

Run your own sshx server.

## Quick Start (Docker Compose)

```bash
git clone https://github.com/sinescode/my-sshx.git
cd my-sshx

# Generate a secret for session tokens
export SSHX_SECRET=$(openssl rand -hex 32)

# Start server + Redis
docker compose -f docker-compose.prod.yml up -d
```

The server will be available at `http://localhost`. Open the URL in a browser and use the CLI as usual:

```bash
sshx --server http://localhost
```

## Configuration

| Environment Variable | Default | Description |
|---|---|---|
| `SSHX_SECRET` | (auto-generated 22-char) | HMAC signing key for session tokens |
| `SSHX_REDIS_URL` | (none) | Redis connection for multi-server mesh |
| `SSHX_RATE_LIMIT` | `100` | Max requests per second |
| `RUST_LOG` | `info` | Logging level (debug, info, warn, error) |

## Reverse Proxy (nginx)

```nginx
server {
    listen 443 ssl http2;
    server_name sshx.example.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        grpc_pass grpc://127.0.0.1:8051;
        grpc_set_header Content-Type application/grpc;
    }

    location /api/ {
        proxy_pass http://127.0.0.1:8051;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }

    location / {
        proxy_pass http://127.0.0.1:8051;
    }
}
```

## Multi-Server Deployment

1. Start Redis: `docker run -d --name redis redis:7-alpine`
2. Start each sshx node:
```bash
docker run -d \
  -e SSHX_SECRET=$SSHX_SECRET \
  -e SSHX_REDIS_URL=redis://<redis-host>:6379 \
  -e SSHX_HOST=<node>.internal:8051 \
  sshx-server
```

Sessions are automatically transferred between nodes via Redis pub/sub.
