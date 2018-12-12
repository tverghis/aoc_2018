# Advent Of Code 2018

These are my solutions to the [AOC 2018](https://adventofcode.com/2018/) challenges. All solutions are written in Rust in an attempt to learn the language.

## Setup

Run `setup.sh` from the root of this repository to create a new Cargo project for a day.

### Prerequisites

`setup.sh` assumes that a file named `session.txt` exists in the same directory. The file should contain the session cookie used by the AOC website. You can find this by examining the GET request executed when requesting a day's input from the AOC website.

### Example

```
$ . ./setup.sh 1
```

This will create a new Cargo project called `day_01`, and place the input for the day in a file called `input.txt` in the `day_01` directory. Additionally, it will create a blank (`.gitignore`d) file called `sample_input.txt`, that I typically use for testing the examples from the problem itself.

It will also subsequently open the newly-created project directory in VS Code.

## Running

In order to run the solution for a given day, `cd` into the directory for the day and `cargo run`, passing it the folder's `input.txt`.

### Example

```
$ cd day_01/
$ cargo run --release < input.txt
```
