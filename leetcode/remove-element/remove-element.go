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

// Package removeelement implements the remove-element problem.
package removeelement

// removeElement removes all instances of val from nums in-place and returns the
// new length. It implements a two-pointer technique where one pointer
// (j) keeps track of the position to write the next non-val element, and the
// other pointer (i) iterates through the array.
func removeElement(nums []int, val int) int {
	j := 0

	for i := range nums {
		if nums[i] != val {
			nums[j] = nums[i]
			j++
		}
	}

	return j
}
