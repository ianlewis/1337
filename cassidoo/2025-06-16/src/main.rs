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

// parse_ordinal parses the roman numeral ordinal number and returns it as a u32.
fn parse_ordinal(name: &str) -> Result<u32, Box<dyn error::Error>> {
    if name.is_empty() {
        return Ok(0);
    }

    let mut total = 0;

    // Parse 1000s
    let mut i = 0;
    while i < name.len() {
        let c = name.chars().nth(i).ok_or("Invalid ordinal format")?;
        if c != 'M' {
            break; // Stop if we reach a character that is not 'M'
        }
        total += 1000;
        i += 1; // Move to the next character
    }

    // Parse 100s
    let dec_places = [
        ('C', 'D', 'M', 100),
        ('X', 'L', 'C', 10),
        ('I', 'V', 'X', 1),
    ];

    for (one_c, five_c, ten_c, base) in dec_places.iter() {
        println!(
            "Parsing: one_c: {}, five_c: {}, ten_c: {}, base: {}",
            one_c, five_c, ten_c, base
        );
        let c = name.chars().nth(i).ok_or("Invalid ordinal format")?;
        if c == *one_c {
            // Peek at the next character to determine if it's a subtractive notation
            let next_c = name.chars().nth(i + 1);
            if next_c == Some(*five_c) {
                total += 4 * base; // subtractive notation for IV, XL, CD, etc.
                i += 2; // Consume the one and five characters
            } else if next_c == Some(*ten_c) {
                total += 9 * base; // e.g. subtractive notation for IX, XC, CM
                i += 2; // Consume the one and ten characters
            } else {
                while i < name.len() {
                    let next_c = name
                        .chars()
                        .nth(i)
                        .ok_or(format!("Invalid ordinal format '{}' at pos {}", name, i))?;
                    if next_c != *one_c {
                        break; // Stop if we reach a character that is not 'I', 'X', or 'C'
                    }
                    total += base;
                    i += 1; // Move to the next character
                }
            }
        } else if c == *five_c {
            total += 5 * base; // D or L or V
            i += 1; // Move to the next character
            while i < name.len() {
                let next_c = name.chars().nth(i);
                if next_c != Some(*one_c) {
                    break; // Stop if we reach a character that is not 'I', 'X', or 'C'
                }
                total += base;
                i += 1; // Move to the next character
            }
        }

        if i >= name.len() {
            return Ok(total); // If we reached the end, return the total
        }
    }

    if i < name.len() {
        return Err(format!("Invalid ordinal format '{}': trailing characters", name).into());
    }

    Ok(total)
}

// sort_monarchs sorts a sequence of monarchs by their name and then their ordinal number.
fn sort_monarchs(monarchs: &[&str]) -> Result<Vec<String>, Box<dyn error::Error>> {
    let mut m = monarchs
        .iter()
        .map(
            |&name| -> Result<(String, String, u32), Box<dyn error::Error>> {
                let name = name.to_string();
                let parts: Vec<&str> = name.split_whitespace().collect();
                let name_part = parts[..parts.len() - 1].join(" ");
                let ordinal_part = parts.last().ok_or("missing value")?;
                println!(
                    "name: {}, name_part: {}, ordinal_part: {}",
                    name, name_part, ordinal_part
                );
                let ordinal_number = parse_ordinal(ordinal_part)?;
                Ok((name, name_part, ordinal_number))
            },
        )
        .collect::<Result<Vec<_>, Box<dyn error::Error>>>()?;

    m.sort_by_key(|k| (k.1.clone(), k.2));

    Ok(m.into_iter().map(|(name, _, _)| name).collect())
}

fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().collect();
    let inputs = args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    if inputs.len() < 2 {
        eprintln!("Usage: {} <name>...", inputs[0]);
        return process::ExitCode::from(1);
    }

    println!(
        "{:?}",
        match sort_monarchs(&inputs[1..inputs.len()]) {
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
    fn test_parse_ordinal() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(parse_ordinal("IX")?, 9);
        assert_eq!(parse_ordinal("CII")?, 102);
        assert_eq!(parse_ordinal("XXX")?, 30);
        assert_eq!(parse_ordinal("XL")?, 40);
        assert_eq!(parse_ordinal("MCCXLIV")?, 1244);
        assert_eq!(parse_ordinal("V")?, 5);
        assert_eq!(parse_ordinal("VI")?, 6);
        assert_eq!(parse_ordinal("I")?, 1);
        Ok(())
    }

    #[test]
    fn test_sort_monarchs() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            sort_monarchs(&["Louis IX", "Louis VIII", "Philip II", "Philip I"])?,
            vec!["Louis VIII", "Louis IX", "Philip I", "Philip II"]
        );

        assert_eq!(
            sort_monarchs(&["George VI", "George V", "Elizabeth II", "Edward VIII"])?,
            vec!["Edward VIII", "Elizabeth II", "George V", "George VI"]
        );
        Ok(())
    }
}
