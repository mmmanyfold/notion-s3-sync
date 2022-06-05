# `mmmanyfold/notion-s3-sync`

Download your Notion's workspace content via Notion API and optionally uploads to s3

### DEPENDENCIES

- [rust](https://www.rust-lang.org/tools/install)

### SETUP

- `cargo build`

### TEST

- `cargo test`

### BUILD

- `cargo build --bin` 

### RUNNING
- as CLI
```shell
export NOTION_API_KEY="secret"
export NOTION_API_WORKSPACE="mmmanyfold"
./ns3 --database xyz

# during development
NOTION_API_KEY=secret NOTION_API_WORKSPACE=mmmanyfold \
  cargo run -- --database xyz
``` 
- as lambda fn
```shell
TODO
```