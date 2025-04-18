// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Program 002_even_fibonacci_numbers prints even Fibonacci numbers.
package main

import "fmt"

const (
	maxFibonacci = 4000000
)

func fib(i int64) int64 {
	if i <= 0 {
		return 1
	}

	if i == 1 {
		//nolint:mnd // The second Fibonacci number is not magic number.
		return 2
	}
	//nolint:mnd // These are not magic numbers.
	return fib(i-1) + fib(i-2)
}

func main() {
	// Start with one because we know the zeroth is odd.
	i := int64(1)
	sum := int64(0)

	for {
		f := fib(i)
		if f > maxFibonacci {
			fmt.Println(sum)
			return
		}

		if f%2 == 0 {
			sum += f
		}

		i++
	}
}
