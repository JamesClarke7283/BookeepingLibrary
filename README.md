# Bookeeping Library
A bookeeping program for psysical libraries to use, written with Leptos in Rust.

# Badges
This program was made by a human.

<image src="https://brainmade.org/white-logo.svg"></image>

This program is licensed as AGPLv3-or-later.

<image src="https://www.gnu.org/graphics/agplv3-with-text-162x68.png"></image>

# Reference Guide

## Environment variables

| Environment variable | Description | Default | Possible values |
|---|---|---|---|
| `LOG_LEvEL` | Controls which log messages `env_logger` displays. Values are case-insensitive. Per-target filters use Rust module paths, and multiple directives can be separated with commas. | `info` | **Global levels:** `off` (disable all logging), `error`, `warn`, `info`, `debug`, `trace` (most verbose).<br>**Per-target:** `bookeeping_core=debug`, `bookeeping_backend=trace`, or `bookeeping_core` to enable all levels for that target.<br>**Combined:** `error,bookeeping_core=debug` sets the global level to `error` and the core crate to `debug`.<br>**Message filtering:** append `/pattern`, such as `debug/failed`, to show only messages matching the pattern. |