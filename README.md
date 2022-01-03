# paste

A tiny pastebin alternative for self-hosting.

My goals with this project are as follows:

1. Learn rust by getting it wrong the first time on a low-stakes project
2. Keep the feature set small.

## Creating new pastes

There is no form for creating a paste. Either use `curl` directly, or configure a program like [ShareX](https://getsharex.com/) on Windows.

Uploading is done via a `multipart/form-data` POST request to a path of `/`. The first part of the request will be used as the text content.

```sh
cat my-cool-file.txt > curl -H "Authorization: Bearer <your-secret-token>" -F file=@- http://localhost:80
```

## Running it yourself

Either build and run it yourself, or use the provided [Docker images](https://github.com/s-thom/paste/pkgs/container/paste).

```sh
cargo run

# OR

docker pull ghcr.io/s-thom/paste
```

### Configuration

Configuration is done through environment variables. Use of a `.env` file is supported for convenience.

| Variable Name      | Description                                         | Default     |
| ------------------ | --------------------------------------------------- | ----------- |
| PASTE_DIR          | Directory to store pastes                           | `pastes`    |
| PASTE_BEARER_TOKEN | A secret that must be provided to create new pastes |             |
| SERVER_HOST        | Host for the app to listen on                       | `127.0.0.1` |
| SERVER_PORT        | Port for the app to listen on                       | `80`        |

## Features / TODO List

- [x] Read files from directory
- [ ] Write files to directory
- [x] Configuration for directory (env variable)
- [ ] Simple bearer token authentication for creating new pastes
