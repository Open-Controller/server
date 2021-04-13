use std::{fs, path::PathBuf, sync::Mutex};
use hotwatch::{Event, Hotwatch};
use protobuf::Message;
use structopt::StructOpt;
use serde_derive::Deserialize;
use OpenControllerLib::*;
use once_cell::sync::Lazy;
use tide::Request;

mod OpenControllerLib;

static HOUSE: Lazy<Mutex<Option<Vec<u8>>>> = Lazy::new(||
    Mutex::new(None)
);

fn default_port() -> i32 { 3612 }

#[derive(Deserialize, Debug)]
struct Environment {
    #[serde(default="default_port")]
    port: i32
}
#[derive(Debug, StructOpt)]
#[structopt(
    name = "OpenController server",
    version = "0.1.0",
    about = "Serves ocbin files.",
    author = "PJTSearch <pjtsignups@gmail.com>"
)]
struct Opts {
    #[structopt(parse(from_os_str), help = "Sets the input file to use")]
    input: PathBuf,

    #[structopt(short = "v", multiple = true, help = "Sets the level of verbosity")]
    out_type: Option<String>,
}

async fn get_home(_: Request<()>) -> tide::Result {
    Ok(tide::Response::builder(200)
        .content_type("application/x-protobuf")
        .body(HOUSE.lock().unwrap().as_ref().unwrap().clone())
        .build()
    )
}

#[async_std::main]
async fn main() {
    let opts = Opts::from_args();
    let env = envy::from_env::<Environment>().unwrap();

    let bytes = fs::read(&opts.input).expect("Could not read file");
    House::parse_from_bytes(&bytes).expect("Invalid file");
    *HOUSE.lock().unwrap() = Some(bytes);

    let mut input_watcher = Hotwatch::new().expect("Hotwatch failed to initialize");
    input_watcher.watch(&opts.input, |event: Event| {
        if let Event::Write(path) = event {
            let bytes = fs::read(path).expect("Could not read file");
            House::parse_from_bytes(&bytes).expect("Invalid file");
            *HOUSE.lock().unwrap() = Some(bytes);
            println!("Reloaded file");
        }
    }).expect("Failed to watch file");

    let mut server = tide::new();
    server.at("/").get(get_home);
    server.listen("0.0.0.0:".to_string() + &env.port.to_string()).await.unwrap();
}