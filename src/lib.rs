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

use std::ops::Not;
use std::process::Command;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RESOLUTION_REGEX: Regex=Regex::new(r"[^\d](\d+x\d+)[^\d]").unwrap();
    }

pub trait Monitor {

    fn status(&self) -> MonitorState;
    fn toggle(&self, state: MonitorState);
    }

pub struct XMonitor {
    name: String,
    }
impl XMonitor {

    pub fn primary() -> Result<XMonitor, String> {
        let xrandr=XMonitor::xrandr();

        for line in xrandr.lines() {
            if line.contains("primary") {
                let name=line.split(" ").next().unwrap().to_string();

                return Ok(XMonitor { name });
                }
            }

        Err(format!("Primary monitor not found"))
        }

    fn turn_on(&self) {
        Command::new("xrandr")
        .arg("--output")
        .arg(&self.name)
        .arg("--auto")
        .output().unwrap();
        }
    fn turn_off(&self) {
        if let Some(resolution)=self.resolution() {
            Command::new("xrandr")
            .arg("--output")
            .arg(&self.name)
            .arg("--fb")
            .arg(&resolution)
            .arg("--off")
            .output().unwrap();
            }
        }

    fn resolution(&self) -> Option<String> {
        let xrandr_entry=self.xrandr_entry();

        if RESOLUTION_REGEX.is_match(&xrandr_entry) {
            let captures=RESOLUTION_REGEX.captures(&xrandr_entry).unwrap();
            let resolution=captures.get(1).unwrap().as_str().to_string();

            return Some(resolution);
            }

        None
        }
    fn xrandr_entry(&self) -> String {
        for line in XMonitor::xrandr().lines() {
            if line.starts_with(&self.name) {
                return line.to_string();
                }
            }

        panic!("Monitor {} not found", self.name);
        }

    fn xrandr() -> String {
        String::from_utf8(
        Command::new("xrandr")
        .output().unwrap().stdout
        ).unwrap()
        }
    }
impl Monitor for XMonitor {

    fn status(&self) -> MonitorState {
        let xrandr_entry=self.xrandr_entry();

        if RESOLUTION_REGEX.is_match(&xrandr_entry) {
            MonitorState::On
            }
        else {
            MonitorState::Off
            }
        }
    fn toggle(&self, state: MonitorState) {
        if state==self.status() {
            return;
            }

        match state {
            MonitorState::On => self.turn_on(),
            MonitorState::Off => self.turn_off(),
            }
        }

    }

#[derive(Clone, Debug, PartialEq)]
pub enum MonitorState {
    On,
    Off,
    }
impl Not for MonitorState {

    type Output=Self;

    fn not(self) -> Self::Output {
        match self {
            MonitorState::On => MonitorState::Off,
            MonitorState::Off => MonitorState::On,
            }
        }
    }

pub fn primary_monitor() -> Result<Box<dyn Monitor>, String> {
    Ok(Box::new(XMonitor::primary()?))
    }
