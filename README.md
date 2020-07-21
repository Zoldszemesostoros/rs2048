# rs2048
A TUI 2048-game, written in Rust.
2048 is a simple logical game. If you don't know it yet, you can read it's [wikipedia page](https://en.wikipedia.org/wiki/2048_(video_game)).

## Features
 - Basic features: collapsing contacting and same tiles on swipe, and putting a new tile to the board.
 - Randomly choose between 2 and 4 when adding new tiles.
 - If the table hasn't changed after swipe, no new tiles appearing.
 - Tiles coming from collapsing collapse further only after the next swipe. (E.g. 2 2 2 2 collapse to 4 4, and only after a second swipe collapse to 8.)
 - Game Over detection

## Compatibility
As it has only cross-platform (and statically linked) depencies (ncurses and rand), it should work on almost every OS.
However, creating Windows builds is tedious, so I'll provide only Linux builds (I'll may reconsider).
If you don't have Linux installed - and don't want to install it - you can use [onworks.net](https://www.onworks.net), that provides a virtualized Linux system that you can control from your browser.

## Running
It don't have any runtime dependencies, so you can download the [binary]() from the Releases page, open a terminal where you've put the binary, and start it with `./rs2048`.

## Building from source
First, clone the repo via git (if you don't have git, and don't want to install it, you can download the source code from GitHub directly):<br>
`git clone https://github.com/Zoldszemesostoros/rs2048.git`<br>
Then install cargo (rust's build system), and the ncurses developement package:<br>
`sudo apt-get install cargo`<br>
`sudo apt-get install ncurses-dev`<br>
then go to the rs2048 dir:<br>
`cd rs2048`<br>
build the project<br>
`cargo build --release`<br>
and run the binary<br>
`build/target/release/rs2048`<br>
