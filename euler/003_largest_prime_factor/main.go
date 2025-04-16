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

// Program 003_largest_prime_factor prints
package main

import (
	"fmt"
	"math"
)

const (
	numToFactor = 600851475143
)

func main() {
	currentNum := int64(numToFactor)
	largest := int64(0)
	// Divide out all 2s
	for currentNum%2 == 0 {
		currentNum /= 2
		largest = 2

		//nolint:mnd // 2 is a not a magic number.
		fmt.Println(2)
	}

	// Divide out all other primes. Prime factors must be odd numbers less than
	// the sqrt of N.
	sqrtN := int64(math.Sqrt(float64(currentNum)))
	//nolint:mnd // 3 is a not a magic number.
	for i := int64(3); i < sqrtN; i += 2 {
		for currentNum%i == 0 {
			currentNum /= i
			largest = i

			fmt.Println(i)
		}
	}

	// Catch the case that the remaining factor is greater than cases so far.
	if currentNum > largest {
		largest = currentNum
	}

	fmt.Println(largest)
}
