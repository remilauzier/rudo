# Rudo

[![Crates.io](https://img.shields.io/crates/v/rudo?style=flat-square)](https://crates.io/crates/rudo)
[![Crates.io](https://img.shields.io/crates/d/rudo?style=flat-square)](https://crates.io/crates/rudo)
[![dependency status](https://deps.rs/crate/rudo/0.8.5/status.svg)](https://deps.rs/crate/rudo/0.8.5)

# CI

[![CI](https://github.com/remilauzier/rudo/actions/workflows/ci.yml/badge.svg)](https://github.com/remilauzier/rudo/actions/workflows/ci.yml)
[![CI-Analyze](https://github.com/remilauzier/rudo/actions/workflows/ci-analyze.yml/badge.svg)](https://github.com/remilauzier/rudo/actions/workflows/ci-analyze.yml)
[![Security-audit](https://github.com/remilauzier/rudo/actions/workflows/security-audit.yml/badge.svg)](https://github.com/remilauzier/rudo/actions/workflows/security-audit.yml)

# Description

**Rudo** "Rust User do" allows a system administrator to give certain users the ability to run some commands as **root**
or another user while logging all commands, and it's arguments.

# Rust version and operating system support

Compile with **rust** ``1.43`` and later, on ``ubuntu-20.04`` and ``macos-10.15``, as test in **CI**. ``2021-04-17``

# Security advisory

**Required** `serde_yaml` `>=0.8.4` because
of [RUSTSEC-2018-0005](https://rustsec.org/advisories/RUSTSEC-2018-0005.html) \
**Rudo** as use `serde_yaml` version `0.8.17` at its debut, so it has never been affected by it

# Package

[crate.io](https://crates.io/crates/rudo)
[Copr](https://copr.fedorainfracloud.org/coprs/remilauzier/rudo/)

# Functionality

[Rudo](https://github.com/remilauzier/rudo/blob/main/man/rudo.md)

# Configuration

[rudo.conf](https://github.com/remilauzier/rudo/blob/main/man/rudo.conf.md)

# Problem

You need to change the owner of the binary to root to make it work, except for the copr package

* `sudo chown root:root`
* `sudo chmod 4755`

# Warning

**No security audit was perform on Rudo**
