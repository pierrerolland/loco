# loco
**This software provides syntax coloration to your log files. Stands for LOg COlorizer**

_Written in Rust._

## Installation

#### Debian-based
`sudo dpkg -i loco.deb`

#### Other systems
`cargo build --release && sudo cp target/release/loco /usr/bin/`

## Usage

The -d option is required, to specify the log definition loco will use. Log definitions are stored in the configuration file (default /etc/loco/loco.yml)

You can override this configuration file by using the -c option

`cat app/logs/dev.log | loco -d sf`

`tail app/logs/dev.log | loco -d sf`

`loco -d sf app/logs/dev.log`

#### Watches at the end of the file for new logs
`loco -d sf -w app/logs/dev.log`

#### Colors the whole line instead of highlighting matches
`loco -d sf -l app/logs/dev.log`

#### Displays help
`loco -h`

## Definitions

The loco.yml file currently contains one definition : Symfony 2+ logs. It's built like this:

```yaml
def_name: # Replace with any name, it will be the one you'll use after the -d option
  lines: # An array containing the main matches. Those which color the whole line if the -l option is specified (no difference with "partials" otherwise)
    - :
      regex: "SOME_REGEX"
      color: "red" # Can be either red, green, yellow, blue, magenta, cyan, light gray or dark gray
    - :
      regex: "SOME_OTHER_REGEX"
      color: "cyan"
  partials: # Entries under this section will be ignored if the -l option is specified. The match will be highlighted otherwise
    - :
      regex: "A_THIRD_REGEX"
      color: "light gray"
    - :
      regex: "A_FOURTH_REGEX"
      color: "magenta"
```
