# Doc Host

Automatically deploy [mdBook](https://github.com/rust-lang-nursery/mdBook) documentation into Github pages.

## Running

Requires Rust nightly, run using:

```
cargo run --release
```

## Configuration

The following values need to be configured as environment variables (or in a `.env` file).

```
TMP_DIR
GIT_AUTHOR
GIT_EMAIL
SSH_KEY_PATH
SRC_URL
SRC_BRANCH
DST_URL
DST_BRANCH
```

The webhook server can be configured with a `Rocket.toml` file:

```toml
[development]
address = "0.0.0.0"
port = 8081
```

### Should I use this?

Probably not, there are more mature solutions available (e.g. https://github.com/drdoctr/doctr)
that allow a much greater level of configuration, this mainly exists to experiment with the Rust
git2 bindings and mdBook.
