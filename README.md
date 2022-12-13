# Advent of Code 2022 :christmas_tree: :crab:

My solutions for Advent of Code 2022.

Project structure and setup kindly stolen from [AxlLind](https://github.com/AxlLind) and greatly expanded upon.

## Setup

Create a .env file containing two tokens: YEAR and AOC_SESSION.

AOC_SESSION is your session cookie, acquired from the advent of code website [like this](https://github.com/wimglenn/advent-of-code-wim/issues/1).

## Usage

```sh
just run          # run all days
just run <DAYS>   # run one or more specific days
                  # <DAY> is always two characters, e.g. 01, 25 etc.

just get <DAYS>   # fetch input and create .rs file from template
                  # input_examples/<DAY>.in has to be filled in manually

just post <DAY>   # submit output for automatic checking.
                  # submits part 2 if part 1 has already been submitted
```
