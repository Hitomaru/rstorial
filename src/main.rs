use rstorial::domains::work::Work;
use std::{io::{Read, Write, stdout}, path::Path};
use std::io;
#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    info!("rstorial running...");
    let work = match get_parameter("new/load", None).as_str() {
        "new" => init(),
        "load" => load(),
        _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "error"))
    };

    work
        .and_then(|work| work.add_entry("第1章"))
        .and_then(|work| work.save_description()).unwrap();
}

fn init() -> Result<Work, std::io::Error> {
    info!("rstorial is initializing your workspace...");
    let work = Work::new(
        get_parameter("Work ID", None).as_str(),
        get_parameter("Author", None).as_str(),
        get_parameter("Title", None).as_str(),
        get_parameter("Description", Some("")).as_str(),
        get_parameter("Workspace path", Some("./")).as_str()
    );
    work.init()
        .and_then(|work| work.save_description())
        .map(|_| work.clone())
}

fn load() -> Result<Work, std::io::Error> {
    info!("rstorial will load your project");
    let path = get_parameter("Your project path", Some("./"));
    let path = Path::new(&path).join("description.yml");
    Work::load_description(&path)
}

fn get_parameter(param: &str, default: Option<&str>) -> String {
    match &default {
        Some(default) => print!("{}(default: {}): ", param, default),
        None => print!("{}: ", param)
    };
    io::stdout().flush().unwrap();
    let form = io::stdin();
    let mut buf = String::new();
    form.read_line(&mut buf).unwrap();
    let parameter_value = buf.trim();

    match parameter_value {
        "" if default.is_none() => get_parameter(param, default),
        "" if default.is_some() => default.unwrap().to_string(),
        value => value.to_string()
    }
}