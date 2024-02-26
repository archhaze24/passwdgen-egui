#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::PasswdgenEgui;

pub struct PasswdgenRequest {
    length: u32,
    remove_uppercase: bool,
    remove_lowercase: bool,
    remove_numbers: bool,
    add_special_characters: bool,
    remove_similar: bool,
}

impl PasswdgenRequest {
    pub fn default() -> PasswdgenRequest {
        PasswdgenRequest {
            length: 16,
            remove_uppercase: false,
            remove_lowercase: false,
            remove_numbers: false,
            add_special_characters: true,
            remove_similar: true,
        }
    }
    pub fn new(
        length: u32,
        remove_uppercase: bool,
        remove_lowercase: bool,
        remove_numbers: bool,
        add_special_characters: bool,
        remove_similar: bool,
    ) -> PasswdgenRequest {
        PasswdgenRequest {
            length,
            remove_uppercase,
            remove_lowercase,
            remove_numbers,
            add_special_characters,
            remove_similar,
        }
    }
}

use rand::Rng;

fn compose_characters(args: &PasswdgenRequest) -> Vec<char> {
    let lowercase: &str = if !args.remove_lowercase {
        if args.remove_similar {
            "abcdefghjkmnpqrstuvwxyz"
        } else {
            "abcdefghijklmnopqrstuvwxyz"
        }
    } else {
        ""
    };

    let uppercase: &str = if !args.remove_uppercase {
        if args.remove_similar {
            "ABCDEFGHIJKMNPQRSTUVWXYZ"
        } else {
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        }
    } else {
        ""
    };

    let special: &str = if args.add_special_characters {
        "!\";#$%&'()*+,-./:;<=>?@[]^_`{|}~"
    } else {
        ""
    };

    let numbers: &str = if !args.remove_numbers {
        if args.remove_similar {
            "23456789"
        } else {
            "0123456789"
        }
    } else {
        ""
    };

    let characters = String::from("") + lowercase + uppercase + special + numbers;

    characters.chars().collect()
}

pub fn generate_passsword(args: PasswdgenRequest) -> Result<String, String> {
    if args.remove_uppercase
        && args.remove_lowercase
        && args.remove_numbers
        && !args.add_special_characters
    {
        return Err(String::from(
            "There is no characters to generate password from",
        ));
    }

    let length = args.length;
    let mut password = String::new();
    let mut rng = rand::thread_rng();
    let characters = compose_characters(&args);

    for _ in 0..length {
        let index = rng.gen_range(0..characters.len());
        password.insert(password.len(), characters[index]);
    }

    Ok(password)
}
