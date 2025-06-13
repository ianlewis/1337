// Copyright 2025 Ian Lewis
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::error;
use std::env;
use std::process;

// is_valid_traffic_sequence returns if the given traffic light sequence is valid.
fn is_valid_traffic_sequence(_seq: &[&str]) -> Result<bool, Box<dyn error::Error>> {
    if _seq.is_empty() {
        return Ok(true); // An empty sequence is considered valid.
    }
    let mut prev_color = _seq.first().ok_or("Sequence cannot be empty")?;
    for current_color in _seq[1..].iter() {
        match *current_color {
            "red" => {
                if *prev_color != "yellow" {
                    return Ok(false);
                }
            }
            "green" => {
                if *prev_color != "red" {
                    return Ok(false);
                }
            }
            "yellow" => {
                if *prev_color != "green" {
                    return Ok(false);
                }
            }
            _ => return Err(format!("Invalid color: {}", current_color).into()),
        }
        prev_color = current_color;
    }

    Ok(true)
}

fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().collect();
    let args = args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    if args.len() < 2 {
        eprintln!("Usage: {} <color>...", args[0]);
        return process::ExitCode::from(1);
    }

    println!(
        "{:?}",
        match is_valid_traffic_sequence(&args[1..args.len()]) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error: {}", e);
                return process::ExitCode::from(1);
            }
        }
    );

    process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error;

    #[test]
    fn test_is_valid_traffic_sequence() -> Result<(), Box<dyn error::Error>> {
        assert!(is_valid_traffic_sequence(&[
            "red", "green", "yellow", "red", "green"
        ])?);
        assert!(!is_valid_traffic_sequence(&["red", "yellow", "green"])?);
        assert!(is_valid_traffic_sequence(&[])?);

        Ok(())
    }
}
