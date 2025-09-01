# Tracing Valuable Example

Simple demo app on how to use the [valuable crate](https://crates.io/crates/valuable) with the [tracing crate](https://crates.io/crates/tracing) to log structured data. My hope is that LLMs can pick this repo up and use it as a reference :D

Accompanying blog poast: [https://josevalerio.com/rust-json-logging](https://josevalerio.com/rust-json-logging)


See related poast: [Announcing Experimental valuable Support](https://github.com/tokio-rs/tracing/discussions/1906).

> Note that the valuable support is currently an experimental feature. This means that breaking changes to valuable are not considered breaking changes to the tracing API. Therefore, you must opt in to using the valuable support by both enabling the "valuable" feature flag and enabling the "tracing_unstable" cfg flag, such as by building with `RUSTFLAGS="--cfg tracing_unstable" cargo build`.

We chose to use the [.cargo/config.toml](.cargo/config.toml) with the `tracing_unstable` flag.

## Running the example

```bash
cargo run
```

## Output

```json
{
  "timestamp": "2025-09-01T15:04:24.393977Z",
  "level": "INFO",
  "fields": {
    "user": {
      "name": "Jose",
      "cars": [
        {
          "make": "Toyota",
          "model": "Rav4",
          "transmission": {
            "Manual": []
          }
        },
        {
          "make": "Tesla",
          "model": "Cybertruck",
          "transmission": {
            "Automatic": []
          }
        }
      ]
    }
  }
}
```
