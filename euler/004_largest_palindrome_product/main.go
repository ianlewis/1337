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

// Program 004_largest_palindrome_product prints the largest palindrome made
// from the product of two 3-digit numbers.
package main

import (
	"fmt"
	"strconv"
)

const (
	maxThreeDigit = 999
	maxTwoDigit   = 99
)

func isPalindrome(x int) bool {
	y := strconv.Itoa(x)
	z := ""

	for i := len(y) - 1; i >= 0; i-- {
		z += string(y[i])
	}

	return z == y
}

func main() {
	a := 0
	b := 0
	largest := 0

	for i := maxThreeDigit; i > maxTwoDigit; i-- {
		for j := maxThreeDigit; j > maxTwoDigit; j-- {
			x := i * j
			if isPalindrome(x) && x > largest {
				a = i
				b = j
				largest = x
			}
		}
	}

	fmt.Println(a, b, largest)
}
