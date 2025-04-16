// Copyright 2025 Ian Lewis
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

use std::env;
use std::process;

fn available_numbers(pos: &str, existing: &[usize]) -> Vec<usize> {
    // ranges is a table of valid positions for each range of 10 numbers.
    // See: https://en.m.wikipedia.org/wiki/NFL_uniform_numbers#Current_system
    let ranges = vec![
        vec!["QB", "RB", "WR", "TE", "LB", "DB", "K", "P", "LS"], // 0-9
        vec!["QB", "RB", "WR", "TE", "LB", "DB", "K", "P", "LS"], // 10-19
        vec!["RB", "WR", "TE", "LB", "DB", "K", "P", "LS"],       // 20-29
        vec!["RB", "WR", "TE", "LB", "DB", "K", "P", "LS"],       // 30-39
        vec!["RB", "WR", "TE", "LB", "DB", "K", "P", "LS"],       // 40-49
        vec!["OL", "DL", "LB", "LS"],                             // 50-59
        vec!["OL", "DL", "LB", "LS"],                             // 60-69
        vec!["OL", "DL", "LB", "LS"],                             // 70-79
        vec!["RB", "WR", "TE", "LS"],                             // 80-89
        vec!["DL", "LB", "K", "P", "LS"],                         // 90-99
    ];

    let mut numbers = Vec::new();
    for (i, positions) in ranges.into_iter().enumerate() {
        for p in positions {
            if p == pos {
                // If the position is in the range, add the numbers to the list.
                for j in 0..10 {
                    let num = i * 10 + j;
                    if num > 0 && !existing.contains(&num) {
                        numbers.push(num);
                    }
                }
            }
        }
    }
    numbers
}

fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [POS] [NUMBER]...", args[0]);
        return process::ExitCode::from(1);
    }

    let numbers = match args[2..]
        .iter()
        .map(|arg| arg.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(numbers) => numbers,
        Err(e) => {
            eprintln!("Error: {}", e);
            return process::ExitCode::from(1);
        }
    };

    println!("{:?}", available_numbers(&args[1], &numbers));
    process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error;

    #[test]
    fn test_available_numbers() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            available_numbers("QB", &[1, 2, 3, 10, 19]),
            vec![4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18]
        );

        assert_eq!(
            available_numbers(
                "QB",
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
            ),
            vec![],
        );

        assert_eq!(
            available_numbers("OL", &[58, 66, 75, 79]),
            &[
                50, 51, 52, 53, 54, 55, 56, 57, 59, 60, 61, 62, 63, 64, 65, 67, 68, 69, 70, 71, 72,
                73, 74, 76, 77, 78
            ]
        );

        // Unknown position
        assert_eq!(available_numbers("AB", &[1, 2, 3, 10, 19]), vec![]);

        Ok(())
    }
}
