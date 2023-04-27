# rmc

rmc is a rust program to move and copy files. rmc is not designed to be used in production or any meaningful use but rather a learning project on creating cli tools.

## Install

To install rmc visit the [Releases](https://github.com/toddmcintire/rmc/releases) page and download the latest version for your OS.

## Usage

to use rmc simply call rmc with the choice and input and output file.

```rmc --choice <choice> <FILE> <FILE>```

using ```--help``` will list all options for rmc.

```command line program to move and copy files

Usage: rmc --choice <choice> <FILE> <FILE>

Arguments:
  <FILE>  file input
  <FILE>  file output

Options:
  -c, --choice <choice>  move or copy
  -h, --help             Print help
  -V, --version          Print version```
  