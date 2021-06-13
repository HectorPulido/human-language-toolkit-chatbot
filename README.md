# HLTK chatbot
This is a regex powered chatbot made in rust, it is based in NLTK https://github.com/nltk/nltk/blob/develop/nltk/chat/util.py

## How to install
add to your Cargo.toml file 
```
[dependencies]
...
human_language_toolkit_chatbot = "0.1.0"
...
```
You can find the updated version at https://crates.io/crates/human_language_toolkit_chatbot

## How to use it 
You can read from a json file.
``` Rust
use human_language_toolkit_chatbot::{Chatbot, ChatbotError, CompiledChatbot};
...
let suntsu = Chatbot::from_file(String::from("bots/suntsu.json")).unwrap();
...
```
Also you can create the bot by programming.

``` Rust
...
let mut pairs: Vec<(String, Vec<String>)> = Vec::new();
pairs.push((
    String::from(r"Hello(.*)"),
    vec![
        String::from("Hello... I'm glad you could drop by today."),
        String::from("Hi there... how are you today?"),
        String::from("Hello, how are you feeling today?"),
    ],
));
pairs.push((
    String::from(r"(.*) sorry (.*)"),
    vec![
        String::from("There are many times when no apology is needed."),
        String::from("What feelings do you have when you apologize?"),
    ],
));
...
let fallback = vec![String::from("Sorry I didn't understand")];
let reflections = Chatbot::default_reflections();

let eliza = Chatbot {
    pairs,
    fallback,
    reflections,
};

// You can save your model in a .json file
match eliza.to_file(String::from("bots/eliza.json")) {
    Ok(_) => (),
    Err(e) => println!("error at {}", e),
}

let eliza = eliza.compile();
```

You can talk with the bot with the function `respond` but you can also use the `converse` function to make an infinity conversation.


<div align="center">
<h3 align="center">Let's connect 😋</h3>
</div>
<p align="center">
<a href="https://www.linkedin.com/in/hector-pulido-17547369/" target="blank">
<img align="center" width="30px" alt="Hector's LinkedIn" src="https://www.vectorlogo.zone/logos/linkedin/linkedin-icon.svg"/></a> &nbsp; &nbsp;
<a href="https://twitter.com/Hector_Pulido_" target="blank">
<img align="center" width="30px" alt="Hector's Twitter" src="https://www.vectorlogo.zone/logos/twitter/twitter-official.svg"/></a> &nbsp; &nbsp;
<a href="https://www.twitch.tv/hector_pulido_" target="blank">
<img align="center" width="30px" alt="Hector's Twitch" src="https://www.vectorlogo.zone/logos/twitch/twitch-icon.svg"/></a> &nbsp; &nbsp;
<a href="https://www.youtube.com/channel/UCS_iMeH0P0nsIDPvBaJckOw" target="blank">
<img align="center" width="30px" alt="Hector's Youtube" src="https://www.vectorlogo.zone/logos/youtube/youtube-icon.svg"/></a> &nbsp; &nbsp;

</p>