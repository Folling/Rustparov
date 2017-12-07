use serenity::Client;

use serenity::prelude::*;
use serenity::framework::standard::{Args, Command, StandardFramework, help_commands};
use serenity::model::{Message, Channel, permissions};
use exprust::{parse, evaluate, NONE, BIN, OCT, HEX, AS_DEGREES};

command!(calc(_context, message, args){
    let mut flags : u64 = 0x0;
    let expr = args.skip().unwrap();
    while !args.is_empty() {
        match args.skip().unwrap().as_str() {
            "bin" => flags |= BIN,
            "oct" => flags |= OCT,
            "hex" => flags |= HEX,
            "degrees" => flags |= AS_DEGREES,
            _ => break
        }
    }
    let mut to_send = parse(expr.as_str(), flags);
    let mut to_send = to_send.as_str();
    if to_send  == "" {
        to_send = "Unable to parse your input.";
        //TODO proper false input handling
    }
    message.channel_id.say(to_send).expect("Unable to send message");
    }
);

command!(eval(_context, message, args){
    let mut to_send = evaluate(args.full().as_str());
    let mut to_send = to_send.as_str();
    if to_send  == "" {
        to_send = "Unable to parse your input.";
        //TODO proper false input handling
    }
    message.channel_id.say(to_send).expect("Unable to send message");
    }
);