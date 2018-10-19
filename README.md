# music-player-rs
A music player written in Rust

## Requirements

* [Rust](https://www.rust-lang.org/en-US/install.html)
* [GTK+](https://www.gtk.org/download/linux.php) (v3.20)

## Compiling

```bash
$ cargo build
```

## Generating doc

```bash
$ cargo doc --no-deps --lib --open
```

## Usage

```bash
$ cargo run
```

### Log
Indicating an output log file (_music_player.log_ by default).

```bash
$ cargo run -- -o <ARCHIVO>
```
or

```bash
$ cargo run -- --output <ARCHIVO>
```
Verbosity

```bash
$ cargo run -- -v
```
Verbosity  | Log level
------------ | -------------
0 | _Off_
1 (_-v_) | _Info_
2 (_-vv \| -v -v_) | _Warn_
3 (_-vvv \| -v -v -v_) | _Max_

About the music player:

```bash
$ cargo run -- -h
```

## Searching songs

The music player is able to filter the music list, by title, performer, album or genre.

Column  | Prefix
------------ | -------------
Title | _T:_
Performer | _P:_
Album | _A:_
Genre | _G:_

For instance _T: title example_ is a valid query.
