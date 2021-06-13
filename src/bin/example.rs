extern crate human_language_toolkit_chatbot;

use human_language_toolkit_chatbot::bots::eliza;

fn main() {
    // let suntsu = suntsu();
    // let suntsu = suntsu.unwrap().compile().unwrap();
    // // println!("{:?}", suntsu);
    // suntsu.converse();

    let eliza = eliza().unwrap();
    eliza.converse();
}
