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

package besttimetobuyandsellstockii

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestMaxProfit(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name     string
		prices   []int
		expected int
	}{
		{
			name:     "Case 1",
			prices:   []int{7, 1, 5, 3, 6, 4},
			expected: 7,
		},
		{
			name:     "Case 2",
			prices:   []int{1, 2, 3, 4, 5},
			expected: 4,
		},
		{
			name:     "Case 3",
			prices:   []int{7, 6, 4, 3, 1},
			expected: 0,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			result := maxProfit(test.prices)
			if diff := cmp.Diff(test.expected, result); diff != "" {
				t.Errorf("maxProfit() mismatch (-want +got):\n%s", diff)
			}
		})
	}
}
