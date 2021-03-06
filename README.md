# domainbot-rs

A bot that lets you check domain availability while brainstorming names for an organization or event on Discord.

Rust rewrite of https://github.com/someshkar/domainbot

## Usage

This bot isn't public as of now, but to run it on yourself, you can follow these steps:

- copy `.env.example` to `.env` and add the discord token
- install rust-bindgen
  ```bash
  cargo install bindgen
  ```
- execute `bindgen.sh`, which builds the `whoisparser` as static binary and generates rust bindings.
  ```bash
  chmod +x bindgen.sh
  ./bindgen.sh
  ```
- run the bot
  ```rust
  cargo run
  ```

## Working

I couldn't find any whois output parser for rust or C(++). Most were written in scripting languages like ruby/python. The one with the least perf overhead seems to be https://github.com/likexian/whois-parser. domainbot-rs interops with this library with cgo and rust's ffi.
