# Zero2Prod

## Features

1. Blog reader to subscribe to a newletter.
    1. Name can be any valid string
    2. Email address needs to be a valid email address
    3. Information will be collected via a HTML form

## Backend

Using `axum` as the web server.

## Development

### Start Postgres

Start Postgres in Docker.

### Start service

```bash
cargo run
```

## Debug

### Access Postgres

```bash
psql -h localhost -p 5432 -U postgres
```
