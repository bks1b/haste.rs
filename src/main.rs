use std::{
    io::{Read, prelude::*},
    env,
    fs::{File, read_to_string},
};
use reqwest::blocking;
use serde::{Serialize, Deserialize};
use serde_json::{from_str};
use regex::Regex;

#[derive(Serialize, Deserialize, Debug)]
struct PostResult {
    key: String,
}

#[derive(Debug)]
enum RequestError {
    Reqwest(reqwest::Error),
    Std(std::io::Error),
}
impl From<reqwest::Error> for RequestError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}
impl From<std::io::Error> for RequestError {
    fn from(e: std::io::Error) -> Self {
        Self::Std(e)
    }
}

fn request(url: &String, body: &String) -> Result<String, RequestError> {
    let mut res = if body == "" { blocking::get(url)? } else {
        blocking::Client
            ::new()
            .post(url)
            .body(body.clone())
            .send()?
    };
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    Ok(body)
}

const DEFAULT_DOMAIN: &str = "https://hasteb.in";

fn help() {
    print!("haste.rs Commands
<required>, [optional]

help
Display this message.

about
Display project metadata and README.

get <key> [server] [output-path]
Retrieve a hastebin document's content, and optionally write it to a file.
<key> - The document's URL or key.
[server] - The hastebin server to retrieve data from. Defaults to the key URL's server or {}
[output-path] - The output file's path.

post <input-path> [server] [raw]
Create a hastebin document from a specified file's contents.
<input-path> - The input file's path.
[server] - The hastebin server to post to. Defaults to {}
[raw] - Whether to output a raw URL. If the argument is specified, it's interpreted as positive. Defaults to negative.", DEFAULT_DOMAIN, DEFAULT_DOMAIN)
}

fn main() {
    let args = env
        ::args()
        .skip(1)
        .collect::<Vec<String>>();

    if args.len() == 0 { help() } else {
        match &*args[0].to_lowercase() {
            "get" => {
                if args.len() == 1 {
                    return println!("Expected at least 1 argument. (key)");
                }
                let matches = Regex
                    ::new(r"(?:(http(?:s)://(?:\S+\.\S+)+)/(?:raw/)?)?(\S+)")
                    .unwrap()
                    .captures(&args[1])
                    .unwrap();
                let server = if matches.get(1).is_none() {
                    if args.len() > 2 {
                        &args[2]
                    } else {
                        return println!("Expected `server` argument.");
                    }
                } else {
                    matches
                        .get(1)
                        .unwrap()
                        .as_str()
                };
                let id = matches
                    .get(2)
                    .unwrap()
                    .as_str();
                let url = format!("{}/raw/{}", server, id);
                let res = request(&url, &"".to_string()).expect("Couldn't get document.");
                if res == "{\"message\":\"Document not found.\"}" {
                    return println!("Document not found.");
                }
                println!("{}", res);
                let arg_i = matches.get(1).map_or(3, |_| 2);
                if args.len() == arg_i + 1 {
                    let mut file = File::create(&args[arg_i]).expect("Couldn't create file.");
                    file.write_all(res.as_bytes()).unwrap();
                    println!("Successfully outputted content into {}", args[arg_i]);
                }
            },
            "post" => {
                if args.len() == 1 {
                    return println!("Expected at least 1 argument. (input)");
                }
                let server = if args.len() > 2 { args[2].clone() } else { DEFAULT_DOMAIN.clone().to_string() };
                let res: PostResult = from_str(
                    &request(
                        &format!("{}/documents", server),
                        &read_to_string(&args[1]).expect("Couldn't read file."),
                    ).expect("Couldn't post document.")
                ).unwrap();
                println!("{}/{}{}", server, if args.len() == 4 { "raw/" } else { "" }, res.key);
            },
            "about" => {
                println!(
                    "{}
{}
Version: {}
Repository: {}
{}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_DESCRIPTION"),
                    env!("CARGO_PKG_VERSION"),
                    env!("CARGO_PKG_REPOSITORY"),
                    read_to_string("./README.md").unwrap()
                );
            },
            "help" => help(),
            cmd => {
                println!("Unknown command {}.", cmd);
                help();
            },
        }
    }
}