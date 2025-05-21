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
use std::process;

// find_shield_break returns the index into the damage array where where the shield breaks.
fn find_shield_break(damage: &[i64], shield: i64) -> i64 {
    let mut current_shield = shield;
    let mut damage = damage; // Make a mutable copy of the damage array.

    if damage.is_empty() {
        // Empty damage is equivalent to single zero damage value.
        damage = &[0];
    }

    for (i, &d) in damage.iter().enumerate() {
        current_shield -= d;
        if current_shield < 0 {
            // Shield breaks
            return i as i64;
        }
    }

    -1
}

fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <damage value>... [shield capacity]", args[0]);
        return process::ExitCode::from(1);
    }

    // Read the damage values.
    let damage = match args[1..args.len() - 1]
        .iter()
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(damage) => damage,
        Err(e) => {
            eprintln!("Error: {}", e);
            return process::ExitCode::from(1);
        }
    };

    // Read the shield capacity.
    let shield_capacity = match args[args.len() - 1].parse::<i64>() {
        Ok(shield_capacity) => shield_capacity,
        Err(e) => {
            eprintln!("Error: {}", e);
            return process::ExitCode::from(1);
        }
    };

    // Find the index where the shield breaks and print it.
    println!("{:?}", find_shield_break(&damage, shield_capacity));

    process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error;

    #[test]
    fn test_find_shield_break() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(-1, find_shield_break(&[1, 2, 3], 6)); // Shield is zero.
        assert_eq!(-1, find_shield_break(&[], 6)); // No damage.
        assert_eq!(0, find_shield_break(&[], -1)); // Shield already broken.
        assert_eq!(0, find_shield_break(&[], -1)); // Shield already broken.

        assert_eq!(2, find_shield_break(&[10, 20, 30, 40], 50));
        assert_eq!(-1, find_shield_break(&[1, 2, 3, 4], 20));
        assert_eq!(0, find_shield_break(&[50], 30));

        Ok(())
    }
}
