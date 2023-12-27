Tool that gets the "current working directory" of the current i3 workspace.

It assumes that workspaces are formatted like this: `<name>:<working directory>`. Name cannot contain a colon. Working directory can use `~` for $HOME. The `:<working directory>` part is optional, if unset then the directory is `$HOME`.

This repo contains both the original `.sh` version, as well as the optimized `.rs` version of the script.

Installation can be done using the `install.sh` and `install_sh.sh` scripts. Both install the tool as `~/bin/i3-pwd`, overwriting whatever is there.

