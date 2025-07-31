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

// Package removeduplicatesfromsortedarray implements LeetCode problem 26:
// Remove Duplicates from Sorted Array.
package removeduplicatesfromsortedarray

// removeDuplicates removes duplicates above the first two from a sorted array
// in-place.
func removeDuplicates(nums []int) int {
	if len(nums) == 0 {
		return 0
	}

	j := 1

	for i := 1; i < len(nums); i++ {
		if nums[i] != nums[i-1] {
			nums[j] = nums[i]
			j++
		}
	}

	return j
}
