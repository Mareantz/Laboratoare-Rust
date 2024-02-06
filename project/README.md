# K9 - A Doctor Who Discord Bot

K9 is a Discord bot built with Rust and the Shuttle library. This project was built for the Rust course at Faculty of Computer Science Iași. It's a fun way to learn and practice Rust and Discord bot development.

## Features

- **Quote**: Sends a random Doctor Who quote.
- **Doctor**: Sends a picture of the n-th Doctor.
- **Episode**: Searches all the titles of all episodes for the given text, and prints the information about that episode (title, runtime, season/episode number). The episodes are up to the 12th season (March 2020).
- **Points**: Prints a leaderboard showing how many points each user has.

In addition to these commands, K9 also posts random trivia questions about the show at regular intervals. The first user to reply with the correct answer gets a point.

## How to Use

To use K9, you need to use shuttle to deploy or run the bot locally on your device. You can find more about this on the shuttle documentation [Shuttle](https://www.shuttle.rs)

## Built With

- [Rust](https://www.rust-lang.org/): A language empowering everyone to build reliable and efficient software.
- [Shuttle](https://github.com/LunaticBots/Shuttle): A Discord API library in Rust.