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

// Program 006_sum_square_difference the difference between the sum of the
// squares of the first one hundred natural numbers and the square of the sum.
package main

import (
	"fmt"
)

const num = int64(100)

func main() {
	sum := int64(0)
	squareSum := int64(0)

	for i := int64(1); i <= num; i++ {
		sum += i
		squareSum += i * i
	}

	fmt.Println(sum*sum - squareSum)
}
