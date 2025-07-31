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

package majorityelement

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestMajorityElement(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name string
		nums []int
		want int
	}{
		{
			name: "Case 1",
			nums: []int{3, 2, 3},
			want: 3,
		},
		{
			name: "Case 2",
			nums: []int{2, 2, 1, 1, 1, 2, 2},
			want: 2,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			got := majorityElement(test.nums)
			if diff := cmp.Diff(test.want, got); diff != "" {
				t.Errorf("majorityElement() mismatch (-want +got):\n%s", diff)
			}
		})
	}
}
