# dev env runtime bin

```text
cargo install cargo-watch
```

## the dev web hot update

```txt
->boss
cargo watch -x 'run --bin bot'

```

## local .env

```env
SERVER.ADDR=0.0.0.0:17778
SERVER.PAGE_SIZE=30
SERVER.CLIENT_ADDR=0.0.0.0:17776
SERVER.BOSS_SECRET="passwd"
SERVER.API_SECRET="passwd"

RUST_LOG=debug

DATABASE_URL=mysql://user:passwd@ip addr/use db
REDIS_URL=redis://:passwd@ip addr:port

```
