# paste

A tiny pastebin alternative for self-hosting.

My goals with this project are as follows:

1. Learn rust by getting it wrong the first time on a low-stakes project
2. Keep the feature set small.

## Configuration

Configuration is done through environment variables. Use of a `.env` file is supported for convenience.

| Variable Name | Description                   | Default     |
| ------------- | ----------------------------- | ----------- |
| PASTE_DIR     | Directory to store pastes     | `pastes`    |
| SERVER_HOST   | Host for the app to listen on | `127.0.0.1` |
| SERVER_PORT   | Port for the app to listen on | `80`        |

## Features / TODO List

- [x] Read files from directory
- [ ] Write files to directory
- [x] Configuration for directory (env variable)
- [ ] Simple bearer token authentication for creating new pastes
