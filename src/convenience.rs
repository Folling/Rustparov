use serenity::prelude::*;
use serenity::model::*;
use serenity::Client;

#[allow(dead_code)]
pub fn send_dm(id : u64, message : &str){
    if let Err(why) = UserId(id).create_dm_channel().unwrap().say(message) {
        println!("Unable to send message in private-channel with UserID <{}> because of {}", id, why);
    }
}