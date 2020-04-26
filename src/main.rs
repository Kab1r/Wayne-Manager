//    Wayne Manager is a Discord bot that renames voice channels.
//    Copyright (C) 2020  Kabir Kwatra
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU Affero General Public License as published
//    by the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU Affero General Public License for more details.
//
//    You should have received a copy of the GNU Affero General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.

use serenity::client::Client;

use std::env;

mod handler;
pub use handler::Handler;

/// Starts the Wayne Manager Discord Bot.
fn main() {
    let mut client = Client::new(
        &env::var("DISCORD_TOKEN").expect("Missing Token in Environment"),
        Handler,
    )
    .expect("Error creating client");

    // Start with single shard
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why)
    }
}
