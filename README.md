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
[![dependency status](https://deps.rs/crate/rudo/0.8.2/status.svg)](https://deps.rs/crate/rudo/0.8.2)
# Description
**Rudo** "Rust User do" allows a system administrator to give certain users the ability to run some commands as **root**
or another user while logging all commands, and it's arguments. \
Compile with rust ``1.43`` and later, on ``ubuntu-20.04`` and ``macos-10.15``, as test in **CI**. ``2021-04-17``

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

You need to change the owner of the binary to root for now to make it work, except copr package

* `sudo chown root:root`
* `sudo chmod 4755`

# License
**GPLv2 or later**

# Warning
Required `serde_yaml` `>=0.8.4` because of [RUSTSEC-2018-0005](https://rustsec.org/advisories/RUSTSEC-2018-0005.html) \
**No security audit was perform on Rudo**
