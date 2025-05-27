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

use chrono::format::ParseError;
use chrono::NaiveDate;

fn calculate_price(
    closing_date: &str,
    visit_date: &str,
    original_price: f64,
) -> Result<f64, ParseError> {
    let closing_date = NaiveDate::parse_from_str(closing_date, "%Y-%m-%d")?;
    let visit_date = NaiveDate::parse_from_str(visit_date, "%Y-%m-%d")?;

    if visit_date > closing_date {
        return Ok(original_price);
    }

    let date_diff = closing_date.signed_duration_since(visit_date);
    let mut price = original_price;
    for _ in 0..date_diff.num_weeks() {
        price *= 0.9; // Apply 10% discount for each week
    }

    Ok(price)
}

fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!(
            "Usage: {} [closing_date] [visit_date] [original_price]",
            args[0]
        );
        return process::ExitCode::from(1);
    }

    let price = match calculate_price(
        &args[1],
        &args[2],
        args[3].parse::<f64>().unwrap_or_else(|_| {
            eprintln!("Error: Original price must be a valid integer.");
            process::exit(1);
        }),
    ) {
        Ok(price) => price,
        Err(e) => {
            eprintln!("Error parsing date: {}", e);
            return process::ExitCode::from(1);
        }
    };

    println!("{}", price);

    process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::assert_approx_eq;
    use std::error;

    #[test]
    fn test_calculate_price() -> Result<(), Box<dyn error::Error>> {
        assert_approx_eq!(
            f64,
            calculate_price("2025-04-01", "2025-03-03", 100.0)?,
            65.61
        );
        assert_approx_eq!(
            f64,
            calculate_price("2025-04-01", "2025-03-15", 50.0)?,
            40.5
        );
        assert_approx_eq!(
            f64,
            calculate_price("2025-04-01", "2025-04-15", 75.0)?,
            75.0
        );

        Ok(())
    }
}
