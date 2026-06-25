# RustKan - High-Performance Kanban Board

<p align="center">
  <img src="https://raw.githubusercontent.com/UberMetroid/RustKan/main/logo.png" alt="RustKan Logo" width="128" height="128">
</p>

RustKan is a clean, secure, and lightning-fast self-hosted Kanban board application built in Rust using Yew (WebAssembly frontend) and Axum (API backend).

---

## 🐳 Container Installation

### Option 1: Docker Compose (Recommended)

1. Create a `docker-compose.yml` file:

```yaml
version: '3'
services:
  rustkan:
    image: ubermetroid/rustkan:latest
    container_name: rustkan
    restart: unless-stopped
    ports:
      - 4405:4405
    environment:
      - PORT=4405
      - SITE_TITLE=RustKan
      - ALLOWED_ORIGINS=*
      - RUSTKAN_PIN=1234  # Optional: Set a PIN to lock board access
    volumes:
      - ./data:/app/data
```

2. Run the container:

```bash
docker compose up -d
```

3. Open your browser and navigate to `http://localhost:4405`.

### Option 2: Docker CLI

Run the following command to start the container:

```bash
docker run -d \
  --name rustkan \
  --restart unless-stopped \
  -p 4405:4405 \
  -e SITE_TITLE=RustKan \
  -v $(pwd)/data:/app/data \
  ubermetroid/rustkan:latest
```

---

## 📋 Configuration Options

Configure these settings inside your Docker Compose environment or container environment variables:

| Variable | Description | Default |
| :--- | :--- | :--- |
| `PORT` | The port number the backend HTTP server will bind to inside the container. | `4405` |
| `SITE_TITLE` | Custom website title rendered in navigation headers, browser tabs, and PWA manifest. *(Supports fallback `RUSTRUSTKAN_TITLE`)* | `RustKan` |
| `BASE_URL` | Application base URL. Essential when deploying behind reverse proxies to ensure redirect and websocket links are resolved correctly. | `http://localhost:4405` |
| `ALLOWED_ORIGINS` | Comma-separated list of allowed HTTP request origins (CORS filter). Use `*` to allow all origins. | `*` |
| `RUSTKAN_PIN` | Optional 4–10 digit PIN (numerical only) to lock access to the interface. Leave empty for public mode. | None |
| `TZ` | Timezone for the container processes and logs. | `UTC` |
| `ENABLE_TRANSLATION` | Enable the multi-language / translation selector in the navigation header (true/false). | `false` |
| `ENABLE_THEMES` | Enable the Super Metroid theme selector in the navigation header (true/false). | `true` |
| `ENABLE_PRINT` | Enable the print button in the navigation header (true/false). | `true` |
| `MAX_ATTEMPTS` | Number of failed PIN attempts permitted before locking out the user client IP address. | `5` |

## 📂 Repository Structure

```
.
├── backend/
│   ├── Cargo.toml
│   └── src
│       ├── auth.rs
│       ├── config.rs
│       ├── handlers.rs
│       ├── main.rs
│       ├── state.rs
│       ├── static_files.rs
│       └── utils.rs
└── frontend/
    ├── Assets
    │   ├── assets
    │   │   ├── logo.png
    │   │   └── logo.svg
    │   ├── login.css
    │   ├── service-worker.js
    │   └── styles.css
    ├── Cargo.toml
    ├── index.html
    └── src
        ├── app.rs
        ├── app_update.rs
        ├── app_update_handlers.rs
        ├── app_view.rs
        ├── header.rs
        ├── i18n
        │   ├── de.rs
        │   ├── en.rs
        │   ├── es.rs
        │   ├── fr.rs
        │   ├── ja.rs
        │   ├── mod.rs
        │   ├── pt.rs
        │   ├── ru.rs
        │   └── zh.rs
        ├── main.rs
        ├── storage.rs
        ├── types.rs
        └── utils.rs
```
