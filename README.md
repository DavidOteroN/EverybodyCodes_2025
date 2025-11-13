# Everybody Codes 2025

<!--toc:start-->
- [Everybody Codes 2025](#everybody-codes-2025)
  - [Basic usage](#basic-usage)
  - [Directory structure](#directory-structure)
<!--toc:end-->

Here are my solutions for [Everybody Codes](https://everybody.codes/home) 2025 quests, written in Rust.

This repo is based on [this awesome template](https://github.com/JarroVGIT/ec-rust-template) that adds functionality to automatically submit the answers and generates the base `.rs` files.

## Basic usage

```bash
cargo scaffold q [p] # Generates template for quest q, part p (optional) and downloads the input notes.
cargo solve q [p] # Runs the code for quest q, and if part number p is provided, submits the answer.
cargo test --bin quest_qq # where qq is the quest number. Runs the tests for the given quest.
```

## Directory structure

```
inputs
  | notes
  |   | 01-1.txt
  |   | ...
  | examples
  |   | 01-1.txt
  |   | ...
src
  | bin
  |   | quest_01.rs
  |   | ...
  | ec
  |   | ...
| | utils
| |   | ...
  | lib.rs
  | main.rs
```

## Quests and solutions

- [Quest 1](https://everybody.codes/event/2025/quests/1): [solution](https://github.com/DavidOteroN/EverybodyCodes2025/blob/master/quest_01/src/main.rs) (older repo before I started using the template. To be ported here).
- [Quest 2](https://everybody.codes/event/2025/quests/2): [solution](https://github.com/DavidOteroN/EverybodyCodes2025/blob/master/quest_02/src/main.rs) (older repo before I started using the template. To be ported here).
- [Quest 3](https://everybody.codes/event/2025/quests/3): [solution](src/bin/quest_03.rs).
- [Quest 4](https://everybody.codes/event/2025/quests/4): [solution](src/bin/quest_04.rs).
- [Quest 5](https://everybody.codes/event/2025/quests/5): [solution](src/bin/quest_05.rs).
  - Custom type `Fishbone` implemented in [`src/utils/fishbone.rs`](src/utils/fishbone.rs)
- [Quest 6](https://everybody.codes/event/2025/quests/6): [solution](src/bin/quest_06.rs).
