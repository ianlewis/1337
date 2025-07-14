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

use std::env;
use std::error;
use std::process;

#[derive(Debug, Copy, Clone)]
struct Fireworks {
    height: i64,
    size: i64,
    velocity: i64,
}

// grand_finale_start finds the index of the first firework in the longest subarray that meets the
// conditions:
//    1. The average size of the fireworks in the subarray is at least 5.
//    2. The velocity of each firework in the subarray is at least 3.
//    3. The difference between the maximum and minimum height of the fireworks in the subarray is
//       at most 10.
fn grand_finale_start(arr: &[Fireworks]) -> usize {
    if arr.is_empty() {
        return 0;
    }

    let mut longest_subarr_index = 0;
    let mut longest_subarr_length = 0;
    for (i, _f) in arr.iter().enumerate() {
        let mut subarr: Vec<Fireworks> = Vec::new();
        let mut size_sum = 0;
        let mut max_height = 0;
        let mut min_height = i64::MAX;

        for f in arr.iter().skip(i) {
            // Calculate the average size of the fireworks in the subarray
            // after adding the current firework
            size_sum += f.size;
            let new_avg = size_sum / (subarr.len() + 1) as i64;

            // Update max and min heights
            let new_max_height = max_height.max(f.height);
            let new_min_height = min_height.min(f.height);
            let max_height_diff = new_max_height - new_min_height;

            // Check if the new average size, velocity, and height difference conditions are met
            if new_avg >= 5 && f.velocity >= 3 && max_height_diff <= 10 {
                max_height = new_max_height;
                min_height = new_min_height;
                subarr.push(*f);
            } else {
                break;
            }
        }

        // Update the longest subarray if the current one is longer
        if subarr.len() > longest_subarr_length {
            longest_subarr_length = subarr.len();
            longest_subarr_index = i;
        }
    }

    longest_subarr_index
}

fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().collect();
    let inputs = args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    if inputs.len() < 2 {
        eprintln!("Usage: {} [<height>,<size>,<velocity>] ...", inputs[0]);
        return process::ExitCode::from(1);
    }

    let fireworks: Result<Vec<_>, _> = inputs
        .iter()
        .skip(1)
        .map(|s| -> Result<Fireworks, Box<dyn error::Error>> {
            let values: Vec<&str> = s.split(',').collect();
            if values.len() != 3 {
                return Err("Each input must be in the format <height>,<size>,<velocity>".into());
            }
            Ok(Fireworks {
                height: values[0].parse()?,
                size: values[1].parse()?,
                velocity: values[2].parse()?,
            })
        })
        .collect();

    match fireworks {
        Ok(fireworks) => {
            if fireworks.is_empty() {
                eprintln!("No valid fireworks data provided.");
                return process::ExitCode::from(1);
            }
            println!("{:?}", grand_finale_start(&fireworks));
        }
        Err(e) => {
            eprintln!("Error parsing fireworks data: {}", e);
            return process::ExitCode::from(1);
        }
    }

    process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grand_finale_start() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            grand_finale_start(&[
                Fireworks {
                    height: 10,
                    size: 6,
                    velocity: 4
                },
                Fireworks {
                    height: 13,
                    size: 3,
                    velocity: 2
                },
                Fireworks {
                    height: 17,
                    size: 6,
                    velocity: 3
                },
                Fireworks {
                    height: 21,
                    size: 8,
                    velocity: 4
                },
                Fireworks {
                    height: 19,
                    size: 5,
                    velocity: 3
                },
                Fireworks {
                    height: 18,
                    size: 4,
                    velocity: 4
                },
            ]),
            2
        );

        Ok(())
    }
}
