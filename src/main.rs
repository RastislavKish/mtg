/*
* Copyright (C) 2023 Rastislav Kish
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, version 3.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use clap::{Args, Parser, Subcommand};
use notify_rust::Notification;

use mtg::MonitorState;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
//#[command(propagate_version=true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    }

#[derive(Subcommand)]
enum Commands {
    /// Prints the status of the monitor
    Status(StatusArgs),
    /// Toggles the monitor between on and off. Specific state can be requested using the according flag.
    Toggle(ToggleArgs),
    }

#[derive(Args)]
struct StatusArgs {
    /// Send a desktop notification about the monitor state
    #[arg(long)]
    notify: bool,
    /// Don't print any output to the terminal
    #[arg(short, long)]
    quiet: bool,
    }

#[derive(Args)]
struct ToggleArgs {
    /// Toggle the monitor on
    #[arg(long)]
    on: bool,
    /// Toggle the monitor off
    #[arg(long)]
    off: bool,
    /// Send a desktop notification about the monitor state
    #[arg(long)]
    notify: bool,
    /// Don't print any output to the terminal
    #[arg(short, long)]
    quiet: bool,
    }

fn main() {
    let cli=Cli::parse();

    match &cli.command {
        Commands::Status(args) => status(args),
        Commands::Toggle(args) => toggle(args),
        }
    }

fn status(args: &StatusArgs) {
    let monitor=mtg::primary_monitor().unwrap();

    let monitor_state=monitor.status();

    if !args.quiet {
        println!("monitor {:?}", monitor.status());
        }

    if args.notify {
        Notification::new()
        .body(&format!("Monitor {:?}", monitor_state))
        .show().unwrap();
        }
    }
fn toggle(args: &ToggleArgs) {
    let monitor=mtg::primary_monitor().unwrap();

    if args.off {
        monitor.toggle(MonitorState::Off);
        }
    else if args.on {
        monitor.toggle(MonitorState::On);
        }
    else {
        monitor.toggle(!monitor.status());
        }

    let monitor_state=monitor.status();

    if !args.quiet {
        println!("Monitor {:?}", monitor_state);
        }

    if args.notify {
        Notification::new()
        .body(&format!("Monitor {:?}", monitor_state))
        .show().unwrap();
        }
    }

