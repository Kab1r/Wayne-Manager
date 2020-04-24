# Wayne Manager Discord Bot

The Wayne Manager renames the voice channel that the Wayne User is to Wayne when Wayne joins and changes it back when they leave.

## Environment Variables

These environment variables are required to run the bot.

| Variables     | Description                   | Example                         |
| ------------- | ----------------------------- | ------------------------------- |
| DISCORD_TOKEN | Secret Token for Discord Bot  | 6qrZcUqja7812RVdnEKjpzOL4CvHBFG |
| WAYNE_ID      | ID of a Wayne User on Discord | 168376512272269313              |

## Setup & Execution

After installing the [Rust toolchain](https://rustup.rs), run `cargo build` to produce a executable.
Running this executable will start the bot.
