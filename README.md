# domainbot-rs

A bot that lets you check domain availability while brainstorming names for an organization or event on Discord.

Rust rewrite of https://github.com/someshkar/domainbot

## Working

I couldn't find any whois output parser for rust or C(++). Most were written in scripting languages like ruby/python. The one with the least perf overhead seems to be https://github.com/likexian/whois-parser. domainbot-rs interops with this library with cgo and rust's ffi.
