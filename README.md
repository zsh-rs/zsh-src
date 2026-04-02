# zsh-src

This crate contains the logic for resolving the source code of zsh, which is used by the `zsh-ffi` crate to build against the correct version of zsh. Currently, it does this by downloading the source code from the official zsh repository on GitHub, but in the future, it may also support other sources, such as a local copy of the source code like provided through [`zsh-dev`](https://packages.debian.org/sid/zsh-dev) on Debian-based systems.

