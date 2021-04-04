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
# Description
**Rudo** "Rust User do" allows a system administrator to give certain
users the ability to run some commands as **root** or another user while 
logging all commands and it's arguments.
Compile with rust 1.43 and later, test manually for now.

# Package
https://copr.fedorainfracloud.org/coprs/remilauzier/rudo/

# Functionality
* You can give **Rudo** a command to execute like `rudo some-command with-args`
* You can invoke a shell with `rudo -s` or `rudo --shell`
* You can change the user to impersonate with `rudo -u some-user` or `rudo --user some-user`
* You can edit document with the editor specify in your environment variable with `rudo -e some-document` or `rudo --edit some-document`
* You can log debug journal to **Journald** with `rudo -d` or `rudo --debug`
* You can start the user greeting with `rudo -g` or `rudo --greeting`

# Configuration
* The config file is in **YAML** and must be at `/etc/rudo.conf` or it will be create
* **Invalid** file will be **REMOVE** and **REPLACED** with default
* You can change the user to impersonate
* You can change the group the user must be member to have authorization
* You can remove the password obligation **at your own risk**
* You can remove the greeting of the user
* You can decide which user is authorized to use **Rudo**

# Problem
You need to change the owner of the binary to root for now to make it work
* `sudo chown root:root`
* `sudo chmod 4755`

# License
**GPLv2 or later**

# Warning
**No guaranty of security for now**
