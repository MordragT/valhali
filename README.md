<div align=center>

# ❄️ Valhali 🦀

[![NixOS](https://img.shields.io/badge/Made_for-Rust-orange.svg?logo=rust&style=for-the-badge)](https://www.rust-lang.org/) [![NixOS](https://img.shields.io/badge/Flakes-Nix-informational.svg?logo=nixos&style=for-the-badge)](https://nixos.org) ![License](https://img.shields.io/github/license/mordragt/nix-templates?style=for-the-badge)

Simple Tokio Service to publish MDNS services and domain aliases

</div>

## About

This is a simple tokio service which will watch for config file changes,
and publish services and aliases by sending the requests to avahi via dbus.

## Usage - Nixos

Use the provided nixos module defined in `flake.nix`

## Usage - Linux WIP

At the moment you have to create your own service file for e.g. systemd.
First install the valhali daemon and then use the following exec:

```bash
valhalid /etc/valhali/config.toml
```

You can look at the provided config under `etc/valhali/config.toml` to see how services and aliases can be defined


## Reference

1. [wiki/Flakes](https://nixos.wiki/wiki/Flakes)
2. [Avahi](https://avahi.org/)