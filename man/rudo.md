# rudo 1 2021-04-01 0.6.1 "Rudo User manual"

# NAME
Rudo is a utility to gain privilege access on unix system with pam.

# SYNOPSIS
`rudo` [`FLAGS`] [`OPTIONS`] *command* ...

`rudo` [`FLAGS`] [`OPTIONS`] `--edit` *file*

`rudo` [`FLAGS`] [`OPTIONS`] --shell

# DESCRIPTION
**Rudo** "RustUser do" allows a system administrator to give certain
users the ability to run some commands as **root** or another user while
logging all commands and its arguments.

# OPTIONS:
`-d`, `--debug`
Log debug messages

`-g`, `--greeting`
Greeting user

`-h`, `--help`
Prints help information

`-s`, `--shell`
Initialize a privilege shell

`-V`, `--version`
Prints version information

`-e`, `--edit edit`
Edit a document with the editor of user

`-u`, `--user user`
The user you want to impersonate

*command* ...
Pass the command to execute

# EXAMPLES
Run the command as privileged user
  $ *rudo* command arguments

Open a shell as a privileged user
  $ *rudo* --shell

# FILES
*/etc/rudo.conf*
  The system wide configuration file.

*/etc/pam.d/rudo*
  The PAM permission file

# SEE ALSO
rudo.conf(5)

# AUTHOR
Rémi Lauzier <remilauzier@protonmail.com>

# COPYRIGHT
Copyright (C) 2021  Rémi Lauzier <remilauzier@protonmail.com>

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program; if not, write to the Free Software Foundation, Inc.,
51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
