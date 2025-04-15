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

fn find_anagrams(s: &str, p: &str) -> Vec<usize> {
    let mut result = Vec::new();
    if s.len() < p.len() || p.is_empty() {
        return result;
    }

    let mut p_chars: Vec<_> = p.chars().collect();
    p_chars.sort();

    for i in 0..s.len() - p.len() + 1 {
        let mut s_chars: Vec<_> = s[i..i + p_chars.len()].chars().collect();
        s_chars.sort();

        if s_chars == p_chars {
            result.push(i);
        }
    }

    result
}

fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <str> <substr>", args[0]);
        return process::ExitCode::from(1);
    }
    println!("{:?}", find_anagrams(&args[1], &args[2]));

    process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error;

    #[test]
    fn test_find_anagrams() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(vec![0, 6], find_anagrams("cbaebabacd", "abc"));
        assert_eq!(Vec::<usize>::new(), find_anagrams("fish", "cake"));
        assert_eq!(vec![0, 1, 2], find_anagrams("abab", "ab"));
        assert_eq!(Vec::<usize>::new(), find_anagrams("", ""));
        assert_eq!(vec![2], find_anagrams("abc", "c"));
        Ok(())
    }
}
