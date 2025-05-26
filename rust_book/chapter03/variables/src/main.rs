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

fn mutability() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // This line will cause a compile-time error because `x` is immutable
    println!("The value of x is: {}", x);
}

fn shadowing() {
    let x = 5;

    let x = x + 1; // This creates a new immutable variable `x` that shadows the previous one

    {
        let x = 5 * 2; // This creates another new immutable variable `x` that shadows the previous one
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

fn type_shadowing() {
    let spaces = "   ";
    let spaces = spaces.len(); // This shadows the previous `spaces` variable
    println!("The number of spaces is: {spaces}");
}

fn main() {
    mutability();
    shadowing();
    type_shadowing();
}
