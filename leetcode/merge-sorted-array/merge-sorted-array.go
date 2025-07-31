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

// Package mergesortedarray implements the merge-sorted-array problem.
package mergesortedarray

// merge merges two sorted integer arrays nums1 and nums2 into nums1 as one
// sorted array. It implements a three-pointer technique where one pointer
// (i) iterates through the first array, the second pointer (j) iterates
// through the second array, and the third (k) denotes the location to insert
// into the first array. The merged result is stored in nums1, which has
// enough space to hold the elements of both arrays.
func merge(nums1 []int, _ int, nums2 []int, _ int) {
	// Copy first array
	nums1cp := make([]int, len(nums1)-len(nums2))
	copy(nums1cp, nums1)

	i := 0
	j := 0
	k := 0

	for k < len(nums1) {
		if i < len(nums1cp) && (j >= len(nums2) || nums1cp[i] < nums2[j]) {
			nums1[k] = nums1cp[i]
			i++
		} else if j < len(nums2) {
			nums1[k] = nums2[j]
			j++
		}

		k++
	}
}
