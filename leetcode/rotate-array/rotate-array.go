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

// Package rotatearray implements the LeetCode problem 189: Rotate Array.
package rotatearray

func rotate(nums []int, k int) {
	c := 0
	k %= len(nums)
	start := 0

	for c < len(nums) {
		i := start
		n := nums[i]

		for {
			j := (i + k) % len(nums)
			nums[j], n = n, nums[j]
			i = j
			c++

			if i == start {
				break
			}
		}

		start++
	}
}
