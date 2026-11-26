// nsync
// Copyright (C) 2026 NexixSoftware

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

// src/main.rs

#![forbid(unsafe_code)]
mod config;
mod storage;

use clap::{Arg, ArgAction, Command};

// TODO: change this to be compile-time injected
pub const SEM_VER: &str = "0.0.1-alpha";

// TODO: expand with more commands
// TODO: create watch command
fn cli() -> Command {
    Command::new("nsync")
        .about("encrypted file sync to infrastructure you own")
        .version(SEM_VER)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("sync")
                .short_flag('S')
                .long_flag("sync")
                .about("Sync to your chosen provider")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .conflicts_with("directory")
                        .help("sync a file ")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("directory")
                        .short('d')
                        .long("directory")
                        .conflicts_with("file")
                        .help("sync a directory")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("recursive")
                        .short('r')
                        .long("recursive")
                        .conflicts_with("file")
                        .help("sync a directory recursively")
                        .action(ArgAction::SetTrue)
                        .num_args(0)
                        .default_missing_value("true")
                        .requires("directory"),
                ),
        )
}

// TODO: create watch implementation
// TODO: begin StorageBackend implementation for R2/S3
// TODO: pipe in StorageBackend implementation
#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("sync", sync_matches)) => {
            if sync_matches.contains_id("file") {
                todo!("StorageBackend function to sync a single file to the upstream provider")
            } else if sync_matches.contains_id("directory") {
                if sync_matches.get_flag("recursive") {
                    todo!(
                        "StorageBackend function to sync an entire directory and all of it's subdirectories to the upstream provider"
                    )
                }

                todo!(
                    "StorageBackend function to sync an entire directory to the upstream provider"
                )
            }

            unreachable!()
        }
        Some(("watch", _)) => {
            unimplemented!("for v0.5");
        }
        _ => unreachable!(),
    }
}
