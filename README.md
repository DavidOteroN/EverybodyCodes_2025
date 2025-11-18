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

| Quest | Solution | Notes |
| --- | --- | --- |
| [Whispers in the shell](https://everybody.codes/event/2025/quests/1) | [Solution](https://github.com/DavidOteroN/EverybodyCodes2025/blob/master/quest_01/src/main.rs) | To be ported to this repo at some point. |
| [From complex to clarity](https://everybody.codes/event/2025/quests/2) | [quest_01.rs](https://github.com/DavidOteroN/EverybodyCodes2025/blob/master/quest_01/src/main.rs) | To be ported to this repo at some point. |
| [The deepest fit](https://everybody.codes/event/2025/quests/3) | [quest_01.rs](/src/bin/quest_03.rs) | |
| [Teeth of the wind](https://everybody.codes/event/2025/quests/3) | [quest_03.rs](/src/bin/quest_03.rs) | |
| [Fishbone order](https://everybody.codes/event/2025/quests/4) | [quest_04.rs](/src/bin/quest_04.rs) | |
| [Mentorship matrix](https://everybody.codes/event/2025/quests/6) | [quest_06.rs](/src/bin/quest_06.rs) | |
| [Namegraph](https://everybody.codes/event/2025/quests/7) | [quest_07.rs](/src/bin/quest_07.rs) | |
| [The art of connection](https://everybody.codes/event/2025/quests/8) | [quest_08.rs](/src/bin/quest_08.rs) | |
| [Encoded in the scales](https://everybody.codes/event/2025/quests/9) | [quest_09.rs](/src/bin/quest_09.rs) | |
