# freepass-cli [![unlicense](https://img.shields.io/badge/un-license-green.svg?style=flat)](http://unlicense.org)

The free password manager for power users: UNIX edition (command line… but not just command line).

## Installation

You need to install [libsodium](https://download.libsodium.org/doc/) on your system first.

Your system's package manager probably has it:

```bash
$ sudo pkg install libsodium
```

Building from source is easy with [cargo](https://crates.io):

```bash
$ cargo build --release
$ strip target/release/freepass 
$ mv target/release/freepass ~/.local/bin
```

(or something... `~/.local/bin` must be on your path of course)

Binary builds will probably be available in the future.

## Usage

The `freepass` binary takes your full name as the `-n` / `--name` option or the `$FREEPASS_NAME` environment variable.  
And the file you want to use as the `-f` / `--file` option or the `$FREEPASS_FILE` environment variable.
If the file doesn't exist, it will be created!

The `freepass` binary has several subcommands, you can see them all by running:
```bash
$ freepass help
```

But the most important one (and the default one) is `interact`.  
It launches *interactive mode*: you'll interact with `freepass` using a menu, a password prompt and a text prompt.

It has built-in command line implementations of all three interactions, but that's not fun.
You can replace each one with an external program!

That's where the "not just command line" bit comes from.
On the command line, you can use the built-in password prompt (which is pretty nice, it displays a [colorhash] of your password after you've entered enough characters) and a fuzzy finder tool like [peco] or [fzf] for the menus.
(See [interactor]'s README for a list of these programs.)
But in X11, you can use [rofi] or [dmenu] for the menus and text prompts, and something like `x11-ssh-askpass` for the password prompt!

Here's an example shell script for a fully X11 experience with [rofi], `x11-ssh-askpass`, `xclip` and `notify-send`:

```bash
#!/bin/sh

NAME="Chloe Price"
FILES_DIR="$HOME/Personal"
ROFI_SETTINGS="-dmenu -fuzzy"

if [ "$FREEPASS_MODE" = "MENU" ]; then
	exec rofi $ROFI_SETTINGS -p 'freepass> '
elif [ "$FREEPASS_MODE" = "TEXT" ]; then
	exec rofi $ROFI_SETTINGS -p "$FREEPASS_PROMPT: "
else
	FREEPASS_NAME="$NAME" \
	FREEPASS_MENU="freepass-x11" \    # Path to THIS SCRIPT!!!
	FREEPASS_ASKTEXT="freepass-x11" \ # Same
	FREEPASS_ASKPASS="x11-ssh-askpass" \
	FREEPASS_FILE="$FILES_DIR/`ls $FILES_DIR | grep .fpass | rofi $ROFI_SETTINGS -p 'freepass file: '`" \ # Select .fpass files from $FILES_DIR
	freepass | (while read -r line; do
		echo -n "$line" | xclip
		notify-send -u low "Copied to clipboard!"
	done)
fi
```

Put it into `~/.local/bin` as `freepass-x11`, make sure `~/.local/bin` is on your `$PATH`, bind it to a hotkey using [sxhkd]...
Do whatever you want, really :-)

[colorhash]: https://github.com/valpackett/colorhash256
[peco]: https://github.com/peco/peco
[fzf]: https://github.com/junegunn/fzf
[interactor]: https://github.com/valpackett/interactor
[rofi]: https://github.com/DaveDavenport/rofi
[dmenu]: http://tools.suckless.org/dmenu/
[sxhkd]: https://github.com/baskerville/sxhkd

## Project-related stuff

See `../README.md`.
