use super::{Chatbot, ChatbotError, CompiledChatbot};

pub fn suntsu() -> Result<Chatbot, ChatbotError> {
    return Chatbot::from_file(String::from("bots/suntsu.json"));
}

pub fn spanish() -> Result<Chatbot, ChatbotError> {
    return Chatbot::from_file(String::from("bots/spanish.json"));
}

pub fn eliza() -> Result<CompiledChatbot, ChatbotError> {
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

    pairs.push((
        String::from(r"I think (.*)"),
        vec![
            String::from("Do you doubt \"%1?\""),
            String::from("Do you really think so?"),
            String::from("But you're not sure \"%1\"?"),
        ],
    ));

    pairs.push((
        String::from(r"How (.*)"),
        vec![
            String::from("How do you suppose?"),
            String::from("Perhaps you can answer your own question."),
            String::from("What is it you're really asking?"),
        ],
    ));

    let fallback = vec![String::from("Sorry I didn't understand")];

    let reflections = Chatbot::default_reflections();

    let eliza = Chatbot {
        pairs,
        fallback,
        reflections,
    };

    // match eliza.to_file(String::from("bots/eliza.json")) {
    //     Ok(_) => (),
    //     Err(e) => println!("error at {}", e),
    // }

    let eliza = eliza.compile();

    return eliza;
}
