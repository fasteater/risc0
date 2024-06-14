// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::utils::CommandExt;
use crate::{help, utils::rzup_home};
use anyhow::Result;
use clap::Subcommand;
use std::io::Write;
use std::{fs, process::Command};
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Debug, Subcommand)]
pub enum ShowSubcmd {
    /// Show the active toolchain
    #[command(after_help = help::SHOW_ACTIVE_TOOLCHAIN_HELP)]
    ActiveToolchain {
        /// Enable verbose output with toolchain information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Display the computed value or RZUP_HOME
    Home,
}

pub fn handle_show(_verbose: bool, subcmd: Option<ShowSubcmd>) {
    match subcmd {
        Some(ShowSubcmd::ActiveToolchain { .. }) => {
            println!("Active toolchain logic not implemented yet.");
        }
        Some(ShowSubcmd::Home) => {
            println!("RZUP_HOME logic not implemented yet.");
        }
        None => {
            show().unwrap();
        }
    }
}

fn show() -> anyhow::Result<()> {
    fn print_header(s: String) -> Result<()> {
        let mut stdout = StandardStream::stderr(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_bold(true))?;
        writeln!(&mut stdout, "{}", s)?;
        writeln!(&mut stdout, "{}", "-".repeat(s.len()))?;
        writeln!(&mut stdout)?;
        stdout.reset()?;
        Ok(())
    }

    let mut stdout = StandardStream::stderr(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_bold(true))?;
    write!(&mut stdout, "rzup home: ")?;
    stdout.reset()?;
    writeln!(&mut stdout, "{}", rzup_home().unwrap().to_str().unwrap())?;

    let extension_version = get_installed_extension_version().unwrap();
    stdout.set_color(ColorSpec::new().set_bold(true))?;
    write!(&mut stdout, "cargo-risczero: ")?;
    stdout.reset()?;
    writeln!(&mut stdout, "{}", extension_version)?;
    print_header("\ninstalled toolchains".to_string())?;
    show_installed_toolchains(false)?;

    Ok(())
}

pub fn get_installed_extension_version() -> Result<String> {
    let out = Command::new("cargo")
        .args(["risczero", "--version"])
        .capture_stdout()
        .expect("Error getting cargo-risczero version")
        .split_whitespace()
        .last()
        .expect("Error parsing cargo-risczero version")
        .to_string();
    Ok(out)
}

pub fn show_installed_toolchains(verbose: bool) -> Result<()> {
    let toolchains_dir = rzup_home()?.join("toolchains");

    if !toolchains_dir.exists() {
        eprintln!("No toolchains directory found.");
        return Ok(());
    }

    let entries = fs::read_dir(&toolchains_dir)?
        .filter_map(|res| res.ok())
        .filter(|entry| entry.path().is_dir())
        .collect::<Vec<_>>();

    if entries.is_empty() {
        println!("No installed toolchains found.");
        return Ok(());
    }

    for entry in entries {
        let toolchain_name = entry.file_name().to_string_lossy().to_string();
        if verbose {
            println!("Toolchain: {}", toolchain_name);
        } else {
            eprintln!("{}", toolchain_name);
        }
    }

    Ok(())
}
