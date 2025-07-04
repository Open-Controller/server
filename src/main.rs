/*
Copyright (C) 2022 PJTSearch

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use anyhow::{Context, Result};
use hotwatch::{Event, EventKind, Hotwatch};
use log::info;
use once_cell::sync::Lazy;
use protobuf::Message;
use serde_derive::Deserialize;
use std::{fs, path::PathBuf, str::FromStr, sync::Mutex};
use structopt::StructOpt;
use tide::{log::LevelFilter, Request};
use OpenControllerLib::*;

mod OpenControllerLib;

static HOUSE: Lazy<Mutex<Option<Vec<u8>>>> = Lazy::new(|| Mutex::new(None));

fn default_port() -> i32 {
    3612
}

#[derive(Deserialize, Debug)]
struct Environment {
    #[serde(default = "default_port")]
    port: i32,
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

    #[structopt(
        short = "v",
        help = "Sets the level of verbosity",
        default_value = "INFO"
    )]
    verbosity: String,
}

async fn get_home(_: Request<()>) -> tide::Result {
    Ok(tide::Response::builder(200)
        .content_type("application/x-protobuf")
        .body(HOUSE.lock().unwrap().as_ref().unwrap().clone())
        .build())
}

#[async_std::main]
async fn main() -> Result<()> {
    let opts = Opts::from_args();
    let env = envy::from_env::<Environment>()?;

    env_logger::builder()
        .filter_level(LevelFilter::from_str(&opts.verbosity)?)
        .init();

    let bytes = fs::read(&opts.input).context("Could not read file")?;
    Module::parse_from_bytes(&bytes).context("Invalid file")?;
    *HOUSE.lock().unwrap() = Some(bytes);

    let mut input_watcher = Hotwatch::new().context("Hotwatch failed to initialize")?;
    input_watcher
        .watch(&opts.input, |event: Event| {
            if let EventKind::Modify(_) = event.kind {
                if let Some(path) = event.paths.first() {
                    let bytes = fs::read(path).expect("Could not read file");
                    Module::parse_from_bytes(&bytes).expect("Invalid file");
                    *HOUSE.lock().unwrap() = Some(bytes);
                    info!("Reloaded file");
                }
            }
        })
        .context("Failed to watch file")?;

    let mut server = tide::new();
    server.at("/").get(get_home);
    server
        .listen("0.0.0.0:".to_string() + &env.port.to_string())
        .await
        .context("Failed to start server")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use async_std::task::sleep;
    use predicates::prelude::*;
    use std::{fs, process::Command, time::Duration};

    #[test]
    fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("opencontroller-server")?;

        cmd.arg("test/file/doesnt/exist");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("No such file or directory"));

        Ok(())
    }

    #[async_std::test]
    async fn serves_correct_data() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("opencontroller-server")?;

        cmd.arg("./test/house.ocbin");
        let mut child = cmd.spawn()?;

        sleep(Duration::from_millis(300)).await;

        let bytes = surf::get("http://0.0.0.0:3612").recv_bytes().await?;
        child.kill()?;

        assert_eq!(bytes, fs::read("./test/house.ocbin")?);

        Ok(())
    }
}
