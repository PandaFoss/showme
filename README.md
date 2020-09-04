<p align="center">
  <img src="/logo/logo.png">
</p>
<p align="center">🎨 Prints color patterns on the terminal</p>


<p align="center">
<a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
<a href="https://github.com/PandaFoss/showme/releases"><img src="https://img.shields.io/github/release/PandaFoss/showme.svg"></a>
<a><img src="https://img.shields.io/badge/language-Rust-brown.svg"></a>
</p>

### Index

- [About](#about)
- [Usage](#usage)
    - [Use in Bash scripts](#use-in-bash-scripts)
- [Compiling from Source](#compiling-from-source)
- [FAQ](#faq)
- [Contributing](#contributing)
- [License](#license)

## About

ShowMe is a small (but effective) command line utility written in Rust for users of \*nix operating systems and followers of the [Unixporn](https://www.reddit.com/r/unixporn/) community. It is intended to help you to embellish the screenshots of your desktop environment or window manager.

> **Note: Currently ShowMe is in an early stage, with a high level of development and many new features to come.**

## Usage

So far the patterns only use the 8 basic colors (black, red, green, yellow, blue, purple, cyan and white) in order to achieve better compatibility. However, there are plans to add another type of pattern (or table) of 256 colors.
The black and white colors can be removed by adding the `--no-blackwhite` flag.
Another point to note is that the resolution of the terminal is not taken into account, so some patterns may be distorted if the size of the pattern exceeds that of the terminal.

With the `-h` or `--help` flag we can print the following help message:

```bash
ShowMe 1.0
Max Ferrer <maxi.fg13@gmail.com>
Prints color patterns on the terminal

USAGE:
    showme [FLAGS] --patterns <PATTERN1,PATTERN2,...>

FLAGS:
    -h, --help             Prints help information
        --no-blackwhite    Removes black and white colors from the color palette.
    -V, --version          Prints version information

OPTIONS:
    -p, --patterns <PATTERN1,PATTERN2,...>    Prints the specified patterns

Available patterns: bars, blocks1, bloks, crunch, panes
```

### Patterns

Patterns are sequences of identical objects that vary in color. ShowMe can display them using the `-p` or `--patterns` option, in any order and repeatedly. For example, `showme -p bars,panes,bars` will display the bar pattern twice and the panel pattern once, in the order described and including the colors black and white. Run `showme --patterns bars,panes,bars` or `showme --patterns="bars,panes,bars"` (with or without quotes) is equivalent and produce the same output.

**Note:** Go to the [images](images) folder to view screenshots of each pattern.

### Use in Bash scripts

If you want to use any of the patterns in a script in Bash you have two options:
1. Implement showme (obviously recommended)
2. Redirect the showme output to a file. For example, if you want to use the bars, you would do something like this:

`showme -p bars > file`

This will create a file with the respective pattern and escape sequences (can be read with `cat` or within a script using the `echo -e` command).

## Compiling from Source

ShowMe requires the following tools and packages to build:
- `git`
- `cargo`
- `rustc`

Clone this repository and enter it:

`git clone https://github.com/PandaFoss/showme.git`
`cd showme`

Finally, simply run:

`cargo build --release`

## FAQ

<details>
<summary>**Why doesn't it show information like neofetch?**</summary>

While at first that was the intention, it seemed pertinent to me to "keep it simple, stupid". For a fetching tool written in pure Rust I recommend [rsfetch](https://github.com/Phate6660/rsfetch).

</details>

<details>
<summary>**Why not do it in a simple Bash script instead of a Rust executable?**</summary>

First of all, because `showme` was thought as a personal project to practice Rust.
Secondly, because although for the purpose for which it was conceived, the fact that it is twice as fast (or even faster) than a Bash script is not very relevant, once a certain complexity or number of lines is reached, a Bash script becomes unfeasible, and it is necessary to resort to programming languages conceived for the purpose in question.

</details>

## Contribute

Any contribution to the project is welcome! See the [CONTRIBUTING.md](CONTRIBUTING.md) file for more details.

## License

ShowMe is distributed primarily under the terms of the MIT license.

See [LICENSE](LICENSE.md) for details.
