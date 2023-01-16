# Bongo Copy Cat

<img src="https://raw.githubusercontent.com/abjt14/bongo-copy-cat/main/main.gif" width=25% height=25%>

[![Build](https://github.com/abjt14/bongo-copy-cat/actions/workflows/main.yml/badge.svg)](https://github.com/abjt14/bongo-copy-cat/actions/workflows/main.yml) [![License](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

## Introduction

**Bongo Copy Cat** wants to be involved in everything you do but instead just imitates you hitting your keyboard all day. After all it's just a cat.

Supported platforms: `MacOS` and `Windows`.

This is a practice project for me to learn [**Rust**](https://www.rust-lang.org/) and [**Tauri**](https://tauri.app/).

## About the meme
Bongo Cat is a meme created by [DitzyFlama](https://twitter.com/DitzyFlama) ([tweet](https://twitter.com/DitzyFlama/status/993487015499853824)) using [StrayRogue](https://twitter.com/StrayRogue)'s [drawing of a cat](https://twitter.com/StrayRogue/status/992994454058381312).

## Libraries that made this possible
* [Rdev](https://github.com/Narsil/rdev) for listening to global input events on Mac OS.
* [InputBot](https://github.com/obv-mikhail/InputBot) for listening to global input events on Windows.

## Installation
* Download exectuables for `MacOS` and `Windows` from the [releases](https://github.com/abjt14/bongo-copy-cat/releases) page.
* Build it yourself.

## Building
You can always build the project youself. This will prevent the system from throwing warnings while installing an unsigned application.

* Install the [Rust](https://www.rust-lang.org/tools/install) toolchain.
* Install [Node Package Manager](https://nodejs.org/en/).

Run the following in a terminal.
```bash
git clone https://github.com/abjt14/bongo-copy-cat.git
cd bongo-copy-cat
npm install
npm run tauri build
```

*Don't just clone the project, give the cat a star ⭐️*
