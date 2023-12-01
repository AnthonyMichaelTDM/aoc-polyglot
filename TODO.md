# todo list for this repo

- [x] gitignore
- [x] update README.md
- [x] move cli tools code from /src to like /cli or something
- [ ] implement cli subcommands:
  - [x] scaffold
  - [x] download
  - [x] solve
  - [x] read
  - [ ] all
- [x] create and update `lint.sh`, `fmt.sh`, `test.sh` to work on all supported languages
  - [x] rust
  - [x] zig
  - [x] python
- [ ] make `lint.sh`, `fmt.sh`, `test.sh` navigate through solution directories and run the appropriate commands in each
  - [ ] lint
  - [ ] fmt
  - [x] test
- [x] CI for all supported languages
  - [x] rust
  - [x] zig
  - [x] python
- [x] re-organize repo structure to better support multiple languages
- [x] update the cli tool to support multiple languages, and multiple years
- [x] create templates for supported languages, which the tool can use to generate new days
  - [x] use tera as a templating engine
  - [x] rust
  - [x] zig
  - [x] python
- [ ] remove dependency on `aoc-cli` crate, if possible (we only need some of it's functionality, and it's MIT licsensed, so we can pretty safely just copy the code we need into our own cli tool)

## aside: planned languages to support

- rust
- zig
- python
- go
