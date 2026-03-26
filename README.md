# anti-spam-matrix

This is a simple Matrix spam banning bot.

Its logic is currently straightforward:

If a user triggers the specified keyword in a certain number of consecutive messages, they will be banned.

If a user triggers the keyword in numerous (the spam_limit) consecutive messages in different groups, they will also be banned.

The bot will ban the spammer in all rooms where it has permissions.

## Build

To get a regular build:

```bash
cargo build --release
```

To get a statically-linked build:

```bash
cargo build --release --no-default-features \
    -F eyra-as-std \
    -F rustls-tls \
    -F socks
```

## Usage

### Configuration

The bot looks for `config.toml` in the platform-specific config directory (e.g. `~/.config/anti-spam-matrix/config.toml` on Linux). If the file does not exist, a template is generated automatically and the bot exits with instructions.

Example `config.toml`:

```toml
username = "@bot:example.org"
spam_limit = 3
spam_regex_exprs = [
    "buy cheap",
    "click here",
]

[auth]
type = "password"
password = "VeryHardPassword"
```

### Authorization

Currently we support two authorization methods, `sso` and `password`.

```toml
[auth]
type = "password"
password = "VeryHardPassword"
```

> Note: In SSO login, the username part of the user ID is ignored. The actual username will be determined by the SSO provider during authentication.

```toml
[auth]
type = "sso_login"
```

### Setup a proxy

```toml
proxy = "socks5://114.51.41.191:9810"
```
or
```toml
proxy = "http://name:passwd@114.51.41.191:9810"
```

### Configuration reference

| Key | Type | Default | Description |
|---|---|---|---|
| `username` | string | — | Full Matrix user ID of the bot account (e.g. `@bot:example.org`) |
| `spam_limit` | integer | `3` | Number of matching messages before a user is banned (count resets on non-matching messages or after a ban) |
| `spam_regex_exprs` | list of strings | `[]` | List of regular expressions to match spam messages |
| `proxy` | string | — | Optional HTTP or SOCKS5 proxy URL |

### Logging

Set the `RUST_LOG` environment variable to control log verbosity. For example:

```bash
RUST_LOG=info anti-spam-matrix
```
