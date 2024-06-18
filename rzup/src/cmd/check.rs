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

use anyhow::Result;
use termcolor::{Color, ColorChoice, StandardStream};

use crate::utils::{get_updatable_items, pretty_print_message, pretty_println_message, UpdateInfo};

pub fn handle_check_all() -> Result<()> {
    let updates = get_updatable_items()?;
    print_updates(&updates);
    Ok(())
}

fn print_updates(updates: &[UpdateInfo]) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for update in updates {
        if update.up_to_date {
            pretty_print_message(&mut stdout, true, None, &format!("{} - ", update.name)).unwrap();
            pretty_print_message(&mut stdout, true, Some(Color::Green), "Up to date ").unwrap();
            pretty_println_message(
                &mut stdout,
                false,
                None,
                &format!(
                    ": {} ({})",
                    update.current_version, update.current_published_at
                ),
            )
            .unwrap();
        } else {
            pretty_print_message(&mut stdout, true, None, &format!("{} - ", update.name)).unwrap();
            pretty_print_message(&mut stdout, true, Some(Color::Yellow), "Update available ")
                .unwrap();
            pretty_println_message(
                &mut stdout,
                false,
                None,
                &format!(
                    ": {} ({}) -> {} ({})",
                    update.current_version,
                    update.current_published_at,
                    update.latest_version,
                    update.latest_published_at,
                ),
            )
            .unwrap();
        }
    }
}
