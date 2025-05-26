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

fn type_annotation() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("You guessed: {}", guess);
}

fn floating_point() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x: {}, y: {}", x, y);
}

fn operations() {
    let sum = 5 + 10; // Addition
    let difference = 95.5 - 4.3; // Subtraction
    let product = 4 * 30; // Multiplication
    let quotient = 56.7 / 32.2; // Division
    let remainder = 43 % 5; // Remainder

    println!(
        "Sum: {}, Difference: {}, Product: {}, Quotient: {}, Remainder: {}",
        sum, difference, product, quotient, remainder
    );
}

fn boolean() {
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t: {}, f: {}", t, f);
}

fn character() {
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻'; // Unicode character
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);
}

fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // Destructuring
    println!("x: {}, y: {}, z: {}", x, y, z);
}

fn array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0]; // Accessing an element
    let second = a[1];
    println!("First: {}, Second: {}", first, second);

    let b = [3; 5]; // Array with 5 elements, all initialized to 3
    println!("b: {:?}", b);
}

fn array_access() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an index between 0 and 4: ");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn main() {
    type_annotation();
    floating_point();
    operations();
    boolean();
    character();
    tuple();
    array();
    array_access();
}
