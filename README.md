# blacepos.xyz/meta

A webserver module for blacepos.xyz which provides information about blacepos.xyz

## Usage

```rust
cargo run -- --log "DEBUG" --web-interface "127.0.0.1" --http-bind 8001
```

## Slot

This server implements the Slot protocol which allows it to be unified with other modules in blacepos.xyz. See [Slot](https://github.com/blacepos/slot) for more information about what this means and how it works.

To enable the Slot client, specify the Slot server's port on localhost with `--slot-port`.

```rust
cargo run -- --log "DEBUG" --web-interface "127.0.0.1" --http-bind 8001 --slot-port 7568
```

## Content Licenses

All audio files in this repository © 2025 Joshua Austin (CC-BY-NC 4.0)