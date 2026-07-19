<p align="center">
  <a href="https://github.com/studio2201">
    <img src="assets/header.jpg" alt="studio2201 banner" width="100%">
  </a>
</p>

# <img src="assets/icon.png" width="32" height="32" valign="middle"> Grid

[![CI](https://github.com/studio2201/grid/actions/workflows/ci.yml/badge.svg)](https://github.com/studio2201/grid/actions/workflows/ci.yml)

Clean, secure, and lightning-fast self-hosted Kanban board in Rust.

## Quick Start

### Self-Hosting (Docker)
Pull and run the official Docker container:
```bash
docker run -d -p 4405:4405 -v /path/to/appdata:/app/data ghcr.io/studio2201/grid:latest
```
