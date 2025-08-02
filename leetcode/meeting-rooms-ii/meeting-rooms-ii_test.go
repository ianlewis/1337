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

package meetingroomsii

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestMinMeetingRooms(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name      string
		intervals [][]int
		expected  int
	}{
		{
			name: "Case 1",
			intervals: [][]int{
				{0, 30},
				{5, 10},
				{15, 20},
			},
			expected: 2,
		},
		{
			name: "Case 2",
			intervals: [][]int{
				{7, 10},
				{2, 4},
			},
			expected: 1,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			got := minMeetingRooms(test.intervals)
			if diff := cmp.Diff(test.expected, got); diff != "" {
				t.Errorf("minMeetingRooms() mismatch (-expected +got):\n%s", diff)
			}
		})
	}
}
