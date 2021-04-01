# rudo.conf 5 2021-04-01 0.6.1 "Rudo User manual"

# NAME
rudo.conf - Configuration file for rudo

# SYNOPSIS
/etc/rudo.conf

# DESCRIPTION
Rudo is an utility to gain privilege access on unix system with pam.

The configuration file consists of items in the format "Option: Value". A description of each item follows:

rudo
  impuser: unix name of the user you want to impersonate

user:
  \- username: your unix user name
    group: the name of the group you must be a member to have privilege access
    password: true or false, if you want to give your password each session or not
    greeting: true or false if you want the hello message each time you run rudo
  \- username: your unix user name
    group: the name of the group you must be a member to have privilege access
    password: true or false, if you want to give your password each session or not
    greeting: true or false if you want the hello message each time you run rudo

# FILES
/etc/rudo.conf

# SEE ALSO
rudo(1)

# AUTHOR
Rémi Lauzier <remilauzier@protonmail.com>
