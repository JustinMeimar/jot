
jot
===

dated note taker with configurable variants. install with cargo.

after running `jot init`, some note variants can be created in
~/.jot/config.toml, which will appear in the command list displayed
by `jot --help`.


```
[jot.note]
subcommand = "n"

[jot.prompt]
subcommand = "p"
```


usage: jot <COMMAND>

commands:
  init             initialize ~/.jot
  list             list variants and their most recent notes
  open             open ~/.jot in $EDITOR
  rename           rename a variant
  remove           remove a variant
  search           search entries by tag or content
  note             [alias: n]
  prompt           [alias: p]
  help             Print this message or the help of the given subcommand(s)

options:
  -h, --help  Print help

