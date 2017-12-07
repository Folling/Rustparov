#[macro_use]
extern crate serenity;

extern crate exprust;

pub mod commands;

use serenity::framework::StandardFramework;
use serenity::prelude::*;
use serenity::model::*;
use serenity::Client;

use std::fs::File;
use std::io::prelude::*;
use std::str;

use commands::*;

mod convenience;
use convenience::*;

#[allow(unused_imports)]
use exprust::*;

struct Handler;

impl EventHandler for Handler {

    fn on_message(&self, _: Context, msg: Message) {

    }
    fn on_ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    use exprust::{parse, AS_DEGREES};
    let mut token_string = String::new();

    File::open("token.txt").unwrap()
        .read_to_string(&mut token_string)
        .expect("Unable to read token");

    let mut client = Client::new(&token_string, Handler);

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .prefix("!>")
            .allow_dm(true)
            .allow_whitespace(true)
            .case_insensitivity(true)
            .ignore_bots(true)
            .delimiter(" ")
            .on_mention(false)
            .owners(vec![UserId(157834823594016768), UserId(135818878176460801)].into_iter().collect())
        )
        .bucket("standard", 2, 10, 3)
        .command("calc", |c| c
            .desc("Calculates a given expression")
            .batch_known_as(vec!["calculate", "parse"])
            .example("4+4 => 8")
            .max_args(1)
            .bucket("standard")
            .owners_only(true)
            .exec(commands::parsing::calc)
        )
        .command("eval", |c| c
            .desc("Evaluates a given comparison")
            .batch_known_as(vec!["evaluate", "comp", "compare", "is"])
            .example("4+4 == 8 => true")
            .max_args(0)
            .bucket("standard")
            .owners_only(true)
            .exec(commands::parsing::eval)
        )
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
