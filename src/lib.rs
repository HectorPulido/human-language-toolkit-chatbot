pub mod bots;

extern crate custom_error;
use custom_error::custom_error;
use rand::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs, result::Result};

custom_error! { pub ChatbotError
    File{source: std::io::Error} = "File error {source}",
    Json{source: serde_json::Error} = "Json parse error {source}",
    Regex{source: regex::Error} = "Regex error {}",
}

pub struct CompiledChatbot {
    pairs: Vec<(Regex, Vec<String>)>,
    fallback: Vec<String>,
    reflections: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chatbot {
    pairs: Vec<(String, Vec<String>)>,
    fallback: Vec<String>,
    reflections: HashMap<String, String>,
}

impl Chatbot {
    pub fn from_file(path: String) -> Result<Chatbot, ChatbotError> {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => return Err(ChatbotError::File { source: e }),
        };

        let ser: serde_json::Result<Chatbot> = serde_json::from_str(&content.as_str());
        let chatbot = match ser {
            Ok(x) => x,
            Err(e) => return Err(ChatbotError::Json { source: e }),
        };

        return Ok(chatbot);
    }

    pub fn to_file(&self, path: String) -> Result<(), ChatbotError> {
        let json_chatbot = match serde_json::to_string(&self) {
            Ok(x) => x,
            Err(e) => return Err(ChatbotError::Json { source: e }),
        };

        let mut file = match fs::File::create(path.as_str()) {
            Ok(file) => file,
            Err(e) => return Err(ChatbotError::File { source: e }),
        };

        match file.write_all(json_chatbot.as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(ChatbotError::File { source: e }),
        };

        return Ok(());
    }

    pub fn compile(&self) -> Result<CompiledChatbot, ChatbotError> {
        let mut new_pairs: Vec<(Regex, Vec<String>)> = Vec::new();

        for pair in &self.pairs {
            match Regex::new(&pair.0) {
                Ok(re) => new_pairs.push((re, pair.1.clone())),
                Err(e) => return Err(ChatbotError::Regex { source: e }),
            }
        }

        let mut new_reflections: Vec<(String, String)> = Vec::new();

        for reflection in &self.reflections {
            new_reflections.push((reflection.0.clone().to_lowercase(), reflection.1.clone()));
        }

        new_reflections.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

        return Ok(CompiledChatbot {
            pairs: new_pairs,
            fallback: self.fallback.clone(),
            reflections: new_reflections,
        });
    }

    pub fn default_reflections() -> HashMap<String, String> {
        let mut reflections: HashMap<String, String> = HashMap::new();

        reflections.insert("i am".to_string(), "you are".to_string());
        reflections.insert("i was".to_string(), "you were".to_string());
        reflections.insert("i".to_string(), "you".to_string());
        reflections.insert("i'm".to_string(), "you are".to_string());
        reflections.insert("i'd".to_string(), "you would".to_string());
        reflections.insert("i've".to_string(), "you have".to_string());
        reflections.insert("i'll".to_string(), "you will".to_string());
        reflections.insert("my".to_string(), "your".to_string());
        reflections.insert("you are".to_string(), "I am".to_string());
        reflections.insert("you were".to_string(), "I was".to_string());
        reflections.insert("you've".to_string(), "I have".to_string());
        reflections.insert("you'll".to_string(), "I will".to_string());
        reflections.insert("your".to_string(), "my".to_string());
        reflections.insert("yours".to_string(), "mine".to_string());
        reflections.insert("you".to_string(), "me".to_string());
        reflections.insert("me".to_string(), "you".to_string());

        return reflections;
    }
}

impl CompiledChatbot {
    fn get_random_response(responses: &Vec<String>) -> String {
        let range = responses.len();
        let num = rand::thread_rng().gen_range(0..range);

        return responses[num].clone();
    }

    fn reflect(&self, bit: &String) -> String {
        let mut new_bit = bit.clone().to_lowercase();

        for reflection in &self.reflections {
            if !new_bit.contains(&reflection.0) {
                continue;
            }
            new_bit = new_bit.replace(&reflection.0, &reflection.1);
        }
        return new_bit;
    }

    fn format(&self, re: &Regex, response: &String, data: &String) -> String {
        let mut response = response.clone();

        for capture in re.captures_iter(data) {
            let mut indx = 0;
            for bit in capture.iter() {
                let indx_str = format!("%{}", indx);
                let bit = match bit {
                    Some(x) => self.reflect(&x.as_str().to_string()),
                    None => continue,
                };
                response = response.replace(&indx_str, bit.as_str());
                indx += 1;
            }
        }

        return response;
    }

    pub fn respond(&self, data: &String) -> String {
        let data = data.trim().to_string();
        for (re, responses) in &self.pairs {
            if re.is_match(&data) {
                let response = CompiledChatbot::get_random_response(&responses);
                return self.format(re, &response, &data);
            }
        }

        return CompiledChatbot::get_random_response(&self.fallback);
    }

    pub fn converse(&self) {
        use std::io::{stdin, stdout};
        loop {
            let mut data = String::new();
            print!("> ");
            let _ = stdout().flush();
            stdin()
                .read_line(&mut data)
                .expect("Did not enter a correct string");

            let response = self.respond(&data);
            println!("> {}", response);
        }
    }
}
