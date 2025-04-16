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

use std::collections;
use std::env;
use std::process;

fn add(l: i64, r: i64) -> i64 {
    l + r
}

fn sub(l: i64, r: i64) -> i64 {
    l - r
}

fn mul(l: i64, r: i64) -> i64 {
    l * r
}

fn div(l: i64, r: i64) -> i64 {
    l / r
}

fn evaluate_oper(s: &str) -> Result<impl Fn(i64, i64) -> i64, Box<dyn std::error::Error>> {
    if s.len() != 1 {
        return Err("Invalid expression".into());
    }

    let op = s[0..1].chars().next().unwrap();

    match op {
        '+' => Ok(add as fn(i64, i64) -> i64),
        '-' => Ok(sub as fn(i64, i64) -> i64),
        '*' => Ok(mul as fn(i64, i64) -> i64),
        '/' => Ok(div as fn(i64, i64) -> i64),
        _ => Err(format!("Invalid character: {}", op).into()),
    }
}

fn evaluate_postfix(s: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let mut result = 0;

    if s.is_empty() {
        return Ok(result);
    }

    if s.len() == 1 {
        return Ok(s[0..1].parse::<i64>()?);
    }

    let mut stack = collections::VecDeque::new();

    // NOTE: Each input number is a single character.
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_digit() {
            stack.push_front(c.to_string());
        } else {
            // NOTE: We have to pop the operands in reverse order.
            let op = evaluate_oper(&c.to_string())?;
            let r = stack.pop_front().ok_or(format!(
                "Invalid expression: not enough operands for {} at index {}",
                c, i
            ))?;
            let l = stack.pop_front().ok_or(format!(
                "Invalid expression: not enough operands for {} at index {}",
                c, i
            ))?;
            stack.push_front(op(l.parse::<i64>()?, r.parse::<i64>()?).to_string());
        }
    }

    if stack.len() == 1 {
        result = stack.pop_front().unwrap().parse::<i64>()?;
    } else {
        return Err(format!("Invalid expression: dangling values: {:?}", stack).into());
    }

    Ok(result)
}

fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <expr>", args[0]);
        return process::ExitCode::from(1);
    }
    match evaluate_postfix(&args[1]) {
        Ok(result) => println!("{:?}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            return process::ExitCode::from(1);
        }
    }

    process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error;

    #[test]
    fn test_evaluate_postfix() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(3, evaluate_postfix("12+")?);
        assert_eq!(77, evaluate_postfix("56+7*")?);
        assert_eq!(67, evaluate_postfix("56+78*+")?);
        assert_eq!(4, evaluate_postfix("52*2*5/")?);
        assert_eq!(-1, evaluate_postfix("56-")?);

        match evaluate_postfix("+") {
            Ok(_) => panic!("Expected error"),
            Err(e) => assert_eq!(e.to_string(), "invalid digit found in string"),
        }

        match evaluate_postfix("12+a") {
            Ok(_) => panic!("Expected error"),
            Err(e) => assert_eq!(e.to_string(), "Invalid character: a",),
        }

        match evaluate_postfix("32+-") {
            Ok(_) => panic!("Expected error"),
            Err(e) => assert_eq!(
                e.to_string(),
                "Invalid expression: not enough operands for - at index 3"
            ),
        }

        match evaluate_postfix("3+") {
            Ok(_) => panic!("Expected error"),
            Err(e) => assert_eq!(
                e.to_string(),
                "Invalid expression: not enough operands for + at index 1"
            ),
        }

        match evaluate_postfix("123+") {
            Ok(_) => panic!("Expected error"),
            Err(e) => assert_eq!(
                e.to_string(),
                "Invalid expression: dangling values: [\"5\", \"1\"]"
            ),
        }

        match evaluate_postfix("12+3") {
            Ok(_) => panic!("Expected error"),
            Err(e) => assert_eq!(
                e.to_string(),
                "Invalid expression: dangling values: [\"3\", \"3\"]",
            ),
        }
        Ok(())
    }
}
