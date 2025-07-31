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

package removeelement

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestRemoveElement(t *testing.T) {
	t.Parallel()

	tests := []struct {
		nums     []int
		val      int
		expected []int
	}{
		{
			nums:     []int{3, 2, 2, 3},
			val:      3,
			expected: []int{2, 2},
		},
		{
			nums:     []int{0, 1, 2, 2, 3, 0, 4, 2},
			val:      2,
			expected: []int{0, 1, 3, 0, 4},
		},
	}

	for _, test := range tests {
		t.Run("", func(t *testing.T) {
			t.Parallel()

			result := removeElement(test.nums, test.val)
			if diff := cmp.Diff(test.expected, test.nums[:result]); diff != "" {
				t.Errorf("removeElement: (-want +got):\n%s", diff)
			}
		})
	}
}
