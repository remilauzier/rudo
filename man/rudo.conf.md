# rudo.conf 5 2021-04-04 0.7.0 "Rudo User manual"

# NAME
rudo.conf - Configuration file for Rudo

# SYNOPSIS
/etc/rudo.conf

# DESCRIPTION
Rudo is an utility to gain privilege access on Unix system with Pam.

The configuration file consists of items in the format "Option: Value". A description of each item follows:

rudo
  impuser: Unix name of the user you want to impersonate

user:
  \- username: your Unix user name
    group: the name of the group you must be a member to have privilege access
    password: true or false, if you want to give your password each session or not
    greeting: true or false if you want the hello message each time you run Rudo
  \- username: your Unix user name
    group: the name of the group you must be a member to have privilege access
    password: true or false, if you want to give your password each session or not
    greeting: true or false if you want the hello message each time you run Rudo

# FILES
/etc/rudo.conf

# SEE ALSO
rudo(1)

# AUTHOR
Rémi Lauzier <remilauzier@protonmail.com>
