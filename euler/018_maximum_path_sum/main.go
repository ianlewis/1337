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

// maxPathSum calculates the maximum path sum in the triangle using a dynamic
// programming technique. The max sum for each point in the triangle is built
// up from bottom to top. The resulting sum at the triangle root gives us the
// maximum sum.
func maxPathSum(t [][]int) int {
	for i := len(t) - 2; i >= 0; i-- {
		for j := 0; j < len(t[i]); j++ {
			t[i][j] += int(math.Max(float64(t[i+1][j]), float64(t[i+1][j+1])))
		}
	}
	return t[0][0]
}

func main() {
	fmt.Println(maxPathSum([][]int{
		{75},
		{95, 64},
		{17, 47, 82},
		{18, 35, 87, 10},
		{20, 0o4, 82, 47, 65},
		{19, 0o1, 23, 75, 0o3, 34},
		{88, 0o2, 77, 73, 0o7, 63, 67},
		{99, 65, 0o4, 28, 0o6, 16, 70, 92},
		{41, 41, 26, 56, 83, 40, 80, 70, 33},
		{41, 48, 72, 33, 47, 32, 37, 16, 94, 29},
		{53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14},
		{70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57},
		{91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48},
		{63, 66, 0o4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31},
		{0o4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 0o4, 23},
	}))
}
