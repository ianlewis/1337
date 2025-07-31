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

// Package removeduplicatesfromsortedarrayii implements the solution to
// LeetCode problem 80: Remove Duplicates from Sorted Array II.
package removeduplicatesfromsortedarrayii

func removeDuplicates(nums []int) int {
	startIndex := 2
	if len(nums) < startIndex {
		return len(nums)
	}

	i := startIndex // Index into the array
	j := startIndex // Index to insert shifted elements

	for ; i < len(nums); i++ {
		if nums[j-2] != nums[i] {
			nums[j] = nums[i]
			j++
		}
	}

	return j
}
