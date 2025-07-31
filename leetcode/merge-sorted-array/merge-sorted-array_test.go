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

package mergesortedarray

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestMerge(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name  string
		nums1 []int
		nums2 []int

		expected []int
	}{
		{
			name:     "Case 1",
			nums1:    []int{1, 2, 3, 0, 0, 0},
			nums2:    []int{2, 5, 6},
			expected: []int{1, 2, 2, 3, 5, 6},
		},
		{
			name:     "Case 2",
			nums1:    []int{1},
			nums2:    []int{},
			expected: []int{1},
		},
		{
			name:     "Case 3",
			nums1:    []int{0},
			nums2:    []int{1},
			expected: []int{1},
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			merge(test.nums1, len(test.nums1)-len(test.nums2), test.nums2, len(test.nums2))

			if diff := cmp.Diff(test.nums1, test.expected); diff != "" {
				t.Errorf("merge: unexpected output (-want +got):\n%s", diff)
			}
		})
	}
}
