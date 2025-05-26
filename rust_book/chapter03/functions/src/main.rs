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

fn another_function() {
    println!("Another function.");
}

fn with_paramater(x: i32) {
    println!("The value of x is: {}", x);
}

fn with_multiple_parameters(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn scope_block() {
    let y = {
        let x = 3;
        x + 1 // This expression is returned
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, world!");

    another_function();

    with_paramater(5);

    with_multiple_parameters(5, 'h');

    scope_block();

    println!("The value returned by five() is: {}", five());
    println!(
        "The value returned by plus_one(five()) is: {}",
        plus_one(five())
    );
}
