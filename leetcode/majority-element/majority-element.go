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

// Package majorityelement implements the LeetCode problem 169: Majority
// Element.
package majorityelement

func majorityElement(nums []int) int {
	// The problem states that there is always a majority element, so there will
	// be at maximum len(nums)/2 + 1 unique elements in the map.
	m := make(map[int]int)
	for _, v := range nums {
		if n, ok := m[v]; ok {
			m[v] = n + 1
		} else {
			m[v] = 1
		}

		if m[v] > len(nums)/2 {
			return v
		}
	}

	panic("this should not happen")
}
