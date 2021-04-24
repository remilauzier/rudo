# Rudo

![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/remilauzier/rudo?style=flat-square)
![Crates.io](https://img.shields.io/crates/v/rudo?style=flat-square)
![Crates.io](https://img.shields.io/crates/d/rudo?style=flat-square)
[![CI](https://github.com/remilauzier/rudo/actions/workflows/ci.yml/badge.svg)](https://github.com/remilauzier/rudo/actions/workflows/ci.yml)
[![CI-Analyze](https://github.com/remilauzier/rudo/actions/workflows/ci-analyze.yml/badge.svg)](https://github.com/remilauzier/rudo/actions/workflows/ci-analyze.yml)
[![Security-audit](https://github.com/remilauzier/rudo/actions/workflows/security-audit.yml/badge.svg)](https://github.com/remilauzier/rudo/actions/workflows/security-audit.yml)
[![GitHub issues](https://img.shields.io/github/issues/remilauzier/rudo?style=flat-square)](https://github.com/remilauzier/rudo/issues)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/remilauzier/rudo?style=flat-square)
![Lines of code](https://img.shields.io/tokei/lines/github/remilauzier/rudo?style=flat-square)
[![dependency status](https://deps.rs/crate/rudo/0.8.5/status.svg)](https://deps.rs/crate/rudo/0.8.5)

# Description

**Rudo** "Rust User do" allows a system administrator to give certain users the ability to run some commands as **root**
or another user while logging all commands, and it's arguments.

# Rust version and operating system support

Compile with **rust** ``1.43`` and later, on ``ubuntu-20.04`` and ``macos-10.15``, as test in **CI**. ``2021-04-17``

# Security rules apply to Rudo via clippy and rust lints

[Rule LANG-NAMING](https://anssi-fr.github.io/rust-guide/04_language.html#LANG-NAMING) of anssi
with `nonstandard_style` \
[C-METADATA](https://rust-lang.github.io/api-guidelines/documentation.html#c-metadata) of rust api guideline
with `clippy::cargo_common_metadata` \
[Rule LANG-NOPANIC](https://anssi-fr.github.io/rust-guide/04_language.html#LANG-NOPANIC) of anssi
with `clippy::expect_used`, `clippy::unwrap_in_result` and `clippy::unwrap_used` \
[Rule LANG-ARRINDEXING](https://anssi-fr.github.io/rust-guide/04_language.html#LANG-ARRINDEXING) of anssi
with `clippy::get_unwrap` and `clippy::indexing_slicing` \
**MISRA-C:2004 Rule 14.10** of **MISRA** with `clippy::else_if_without_else` \
[Rule MEM-FORGET](https://anssi-fr.github.io/rust-guide/05_memory.html#MEM-FORGET)
and [Recommendation MEM-FORGET-LINT](https://anssi-fr.github.io/rust-guide/05_memory.html#recommendation-a-idmem-forget-lintamem-forget-lint)
of anssi with `clippy::mem_forget` \
[Rule LANG-ARITH](https://anssi-fr.github.io/rust-guide/04_language.html#LANG-ARITH) of anssi
with `clippy::integer_arithmetic` \

# Security advisory

**Required** `serde_yaml` `>=0.8.4` because
of [RUSTSEC-2018-0005](https://rustsec.org/advisories/RUSTSEC-2018-0005.html) \
**Rudo** as use `serde_yaml` version `0.8.17` at its debut, so it has never been affected by it \

# Package

https://copr.fedorainfracloud.org/coprs/remilauzier/rudo/

# Functionality

* You can give **Rudo** a command to execute like `rudo some-command with-args`
* You can invoke a shell with `rudo -s` or `rudo --shell`
* You can change the user to impersonate with `rudo -u some-user` or `rudo --user some-user`
* You can edit document with the editor specify in your environment variable with `rudo -e some-document`
  or `rudo --edit some-document`
* You can log debug journal with `rudo -d` or `rudo --debug`
* You can start the user greeting with `rudo -g` or `rudo --greeting`
* You can log debug or info messages to ``journald`` on **Linux** or to `oslog` on **macOS**

# Configuration
* The config file is in **YAML** and must be at `/etc/rudo.conf` or it will be created
* **Invalid** file will be **REMOVE** and **REPLACED** with default
* You can change the user to impersonate
* You can change the group the user must be member to have authorization
* You can remove the password obligation **at your own risk**
* You can remove the greeting of the user
* You can decide which user is authorized to use **Rudo**

# Problem

You need to change the owner of the binary to root to make it work, except for the copr package
* `sudo chown root:root`
* `sudo chmod 4755`

# Warning
**No security audit was perform on Rudo**
