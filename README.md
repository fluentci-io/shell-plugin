# Shell Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/shell)](https://pkg.fluentci.io/shell)
[![ci](https://github.com/fluentci-io/shell-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/shell-plugin/actions/workflows/ci.yml)


This plugin allows you to run shell commands in your [FluentCI](https://fluentci.io) pipeline.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm shell fish echo "Hello, World!"
```

## Functions

| Name   | Description                               |
| ------ | ----------------------------------------- |
| bash   | Run bash commands                         |
| fish   | Run fish commands                         |
| zsh    | Run zsh commands                          |
| nu     | Run nushell commands                      |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/shell@v0.1.1?wasm=1", "bash", vec!["echo 'Hello, World!'"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: shell
    args: |
      fish echo "Hello, World!"
```
