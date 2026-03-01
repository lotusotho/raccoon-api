# The Raccoon API 🦝


A Rust-based REST API built with [Axum](https://github.com/tokio-rs/axum) dedicated to serving raccoon-related content, including images, memes, facts, videos, 3D models, etc...

Thanks to the [api.racc.lol repo](https://github.com/raccoonOrg/api.racc.lol) and Venqoi for a lot of the assets used in this project.

The API uses docker and make commands to easily host locally or remotely an instance

## API Endpoints

The API is versioned (default `v1`).

| Endpoint | Description |
| :--- | :--- |
| `GET /v1/` | API root |
| `GET /v1/health` | Health check |
| `GET /v1/coon` | Random raccoon image |
| `GET /v1/thiscoon/{id}` | Get raccoon image by ID |
| `GET /v1/meme` | Random raccoon meme |
| `GET /v1/thismeme/{id}` | Get meme by ID |
| `GET /v1/vid` | Random raccoon video |
| `GET /v1/thisvid/{id}` | Get video by ID |
| `GET /v1/rotd` | Raccoon of the Day |
| `GET /v1/fact` | Random raccoon fact |
| `GET /v1/model` | Random 3D model URL and attribution |
| `GET /v1/sound` | Random raccoon sound |
| `GET /v1/wiki` | Raccoon Wikipedia redirect |
| `GET /v1/games` | Raccoon-related game release info |

## Contribute

New cool features will be added overtime! Feel free to open a new PR or issue if you'd like something added or have any ideas for a change!

New media is always welcomed!!!