extern crate human_language_toolkit_chatbot;

use human_language_toolkit_chatbot::*;

pub fn suntsu() -> Result<Chatbot, ChatbotError> {
    return Chatbot::from_file(String::from("bots/suntsu.json"));
}

pub fn spanish() -> Result<Chatbot, ChatbotError> {
    return Chatbot::from_file(String::from("bots/spanish.json"));
}

fn main() {
    let eliza = bots::eliza().unwrap();
    eliza.converse();
}
