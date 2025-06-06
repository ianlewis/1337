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

//nolint:all // TODO(#28): Fix golangci-lint issues.
package main

import (
	"fmt"
	"math"
)

const num = int64(10001)

func isPrime(value int64) bool {
	for i := int64(2); i <= int64(math.Sqrt(float64(value+1))); i++ {
		if value%i == 0 {
			return false
		}
	}
	return true
}

func main() {
	// Current number. Start with 3
	i := int64(3)
	// Number of primes found. We count 2 as prime.
	j := int64(1)

	for {
		if isPrime(i) {
			j++
		}

		if j == num {
			fmt.Println(i)
			return
		}

		// Iterate by two because we know that all primes starting with 3 must
		// be odd.
		i += 2
	}
}
