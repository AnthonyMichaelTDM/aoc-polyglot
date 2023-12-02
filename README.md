<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD041 -->

<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code {year}

Solutions for [Advent of Code](https://adventofcode.com/) in various languages, including [Rust](https://www.rust-lang.org/), [Zig](https://ziglang.org/), and [Python](https://www.python.org/)

<!--- advent_readme_stars table --->

<!--- benchmarking table --->

---

## Template setup

This template supports all major OS (macOS, Linux, Windows).

### Create your repository 📝

1. Open [the template repository](https://github.com/AnthonyMichaelTDM/aoc-polyglot-template) on Github.
2. Click [Use this template](https://github.com/AnthonyMichaelTDM/aoc-polyglot-template/generate) and create your repository.
3. Clone your repository to your computer.
4. If you are solving a previous year's advent of code, change the `AOC_YEAR` variable in `.cargo/config.toml` to reflect the year you are solving.

### Setup rust 💻

1. Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
2. (recommended) Install the [rust-analyzer](https://rust-analyzer.github.io/manual.html) extension for your code editor.
3. (optional) Install a native debugger. If you are using VS Code, [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) is a good option.

### Setup zig 💻

1. Install the [Zig toolchain](https://ziglang.org/download/).
2. (recommended) Install the [Zig Language Server](https://github.com/zigtools/zls/wiki/Installation) extension for your code editor.

### Setup python 💻

1. Install the [Python toolchain](https://www.python.org/downloads/), I recommend version 3.11 or higher for the `match` statement.
2. create a virtual environment with `python -m venv .venv` and activate it with `source .venv/bin/activate`. (the [Python])
3. Install recommended packages with `pip install -r requirements.txt`. (Do this in your virtual environment, so you don't pollute your global python installation.)

### Setting up your Editor (VS Code) 💻

- (recommended) set up separate profiles for each language you're working with ([VS Code Profiles](https://code.visualstudio.com/docs/editor/profiles)). This will allow you to have separate settings for each language, reduce extension clutter, and allow you to use different debuggers for each language.
- Install the relevant recommended extensions for your language. (`extensions.json`)

---

✨ You can start solving puzzles now! Head to the [Usage section](#usage) to see how to use this template. If you like, you can configure [some optional features](#optional-template-features).

## Usage

> [!TIP]
> in most cases, you can omit the `-y <year>` argument to default to the current/most recent AoC

### Scaffold a day

```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module file "src/bin/01.rs"
# Created empty input file "data/inputs/01.txt"
# Created empty example file "data/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

Individual solutions live in the `./src/bin/` directory as separate binaries. _Inputs_ and _examples_ live in the the `./data` directory.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/main/src/bin/scaffold.rs#L11-L41) has _unit tests_ referencing its _example_ file. Use these unit tests to develop and debug your solutions against the example input.

Tip: when editing a solution, `rust-analyzer` will display buttons for running / debugging unit tests above the unit test blocks.

### Download input & description for a day

> [!NOTE]  
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

```sh
# example: `cargo download 1 -y 2023`
cargo download <day> -y <year>

# output:
# [INFO  aoc] 🎄 aoc-cli - Advent of Code command-line tool
# [INFO  aoc_client] 🎅 Saved puzzle to 'data/puzzles/01.md'
# [INFO  aoc_client] 🎅 Saved input to 'data/inputs/01.txt'
# ---
# 🎄 Successfully wrote input to "data/inputs/01.txt".
# 🎄 Successfully wrote puzzle to "data/puzzles/01.md".
```

### Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day> -y <year> -l <lang>

# output:
#     Finished dev [unoptimized + debuginfo] target(s) in 0.13s
#     Running `target/debug/01`
# Part 1: 42 (166.0ns)
# Part 2: 42 (41.0ns)
```

The `solve` command runs your solution against real puzzle inputs.

By default, `solve` executes your code once and shows the execution time. If you append the `--time` flag to the command, the runner will run your code between `10` and `1,000` times (depending on execution time of first execution) and print the average execution time.

For example, running a benchmarked execution of your rust solution to day 1 would look like `cargo solve 1 -l rust --time`.

#### Submitting solutions

> [!IMPORTANT]  
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

In order to submit part of a solution for checking, append the `--submit <part>` option to the `solve` command.

### Run all solutions

> [!NOTE]
> This feature isn't implemented yet

```sh
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# Part 1: 42 (19.0ns)
# Part 2: 42 (19.0ns)
# <...other days...>
# Total: 0.20ms
```

This runs all solutions sequentially and prints output to the command-line.

#### Update readme benchmarks

> [!IMPORTANT] 
> Benchmark times include overhead like starting the process, reading the input file, etc., and therefore may not be representative of the actual execution time of your solution.

The template can output a table with solution times to your readme. Please note that these are not "scientific" benchmarks, understand them as a fun approximation. 😉

In order to generate a benchmarking table, run `cargo all --time`. If everything goes well, the command will output "_Successfully updated README with benchmarks._" after the execution finishes.

### Run all tests

```sh
./test.sh
```

### Format code

```sh
./fmt.sh
```

### Lint code

```sh
./lint.sh
```

### Read puzzle description in terminal

> [!IMPORTANT]
> This command requires [installing the aoc-cli crate](#configure-aoc-cli-integration).

```sh
# example: `cargo read 1`
cargo read <day> -y <year>

# output:
# Loaded session cookie from "/Users/<snip>/.adventofcode.session".
# Fetching puzzle for day 1, 2022...
# ...the input...
```

## Optional template features

### Configure aoc-cli integration

1. Install [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli/) via cargo: `cargo install aoc-cli --version 0.12.0`
2. Create an `.adventofcode.session` file in your home directory and paste your session cookie. To retrieve the session cookie, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in _Cookies_ under the _Application_ or _Storage_ tab, and copy out the `session` cookie value. [^1]

Once installed, you can use the [download command](#download-input--description-for-a-day) and automatically submit solutions via the [`--submit` flag](#submitting-solutions).

### Automatically track ⭐️ progress in the readme

This template includes [a Github action](https://github.com/k2bd/advent-readme-stars) that automatically updates the readme with your advent of code progress.

To enable it, complete the following steps:

#### 1. Create a private leaderboard

Go to the leaderboard page of the year you want to track and click _Private Leaderboard_. If you have not created a leaderboard yet, create one by clicking _Create It_. Your leaderboard should be accessible under `https://adventofcode.com/{year}/leaderboard/private/view/{aoc_user_id}`.

#### 2. Set repository secrets

Go to the _Secrets_ tab in your repository settings and create the following secrets:

- `AOC_USER_ID`: Go to [this page](https://adventofcode.com/settings) and copy your user id. It's the number behind the `#` symbol in the first name option. Example: `3031`.
- `AOC_YEAR`: the year you want to track. Example: `2021`.
- `AOC_SESSION`: an active session[^2] for the advent of code website. To get this, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in your Cookies under the Application or Storage tab, and copy out the `session` cookie.

✨ You can now run this action manually via the _Run workflow_ button on the workflow page. If you want the workflow to run automatically, uncomment the `schedule` section in the `readme-stars.yml` workflow file or add a `push` trigger.

Go to the _Variables_ tab in your repository settings and create the following variable:

- `AOC_ENABLED`: This variable controls whether the workflow is enabled. Set it to `true` to enable the progress tracker.

### Check code formatting / clippy lints in CI

Uncomment the respective sections in the `ci.yml` workflow.

## Footnotes

[^1]: The session cookie might expire after a while (~1 month) which causes the downloads to fail. To fix this issue, refresh the `.adventofcode.session` file.
[^2]: The session cookie might expire after a while (~1 month) which causes the automated workflow to fail. To fix this issue, refresh the AOC_SESSION secret.
