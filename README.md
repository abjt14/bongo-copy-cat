# Bongo Copy Cat

<img src="https://raw.githubusercontent.com/abjt14/bongo-copy-cat/main/main.gif" width=25% height=25%>

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/) [![Build](https://github.com/abjt14/bongo-copy-cat/actions/workflows/main.yml/badge.svg)](https://github.com/abjt14/bongo-copy-cat/actions/workflows/main.yml) [![License](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

## Introduction

**Bongo Copy Cat** wants to be involved in everything you do but instead just imitates you hitting your keyboard all day. After all it's just a cat.

A desktop app built using the [**Tauri**](https://tauri.app/) framework. Supported platforms: `MacOS` and `Windows`.

This is a practice project for me to learn **Rust** and **Tauri**. Read more about the idea on [dev.to](https://dev.to/abjt14/bongo-copy-cat-a-tauri-cat-that-wants-to-be-involved-in-everything-you-do-5bge).

Here's the app in action.
<img src="https://raw.githubusercontent.com/abjt14/bongo-copy-cat/main/main-2.gif">

## About the meme
Bongo Cat is a meme created by [DitzyFlama](https://twitter.com/DitzyFlama) ([tweet](https://twitter.com/DitzyFlama/status/993487015499853824)) using [StrayRogue](https://twitter.com/StrayRogue)'s [drawing of a cat](https://twitter.com/StrayRogue/status/992994454058381312).

## Libraries that made this possible
* [Rdev](https://github.com/Narsil/rdev) for listening to global input events on Mac OS.
* [InputBot](https://github.com/obv-mikhail/InputBot) for listening to global input events on Windows.

## Installation Methods
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

## Some OS specific caveats
Since this application is not signed, your operating system may warn you about the same.

Here's how you can fix the issues on your `OS`. Please note that these errors occur only on the "first run".

### MacOs
* In your `System Preferences` app, go to `Security & Privacy > Privacy > Accessibility`, and click on `Open Anyway`.
* Please note that this app requires access to `Input Monitoring`.
* When you open the application, the system will ask you to allow it.
* Even if you deny access the first time, you can always give the app access manially by going to `System Preferences > Security & Privacy > Privacy > Input Monitoring`.

#### Windows
* When you first open the app, you will see a `popup` saying "Windows Defender SmartScreen prevented an unregonized app from starting.", and only one option saying "Don't run".
* At the end of the `warning`, you'll see a link called `More Info`.
* Once you click on that, you should see a new option at the botom called "Run anyway". Click on that button and the app will work perfectly.

*Don't just clone the project, give the cat a star ⭐️*
