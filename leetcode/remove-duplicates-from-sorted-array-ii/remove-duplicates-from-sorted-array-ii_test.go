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

package removeduplicatesfromsortedarrayii

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestRemoveDuplicates(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name     string
		nums     []int
		expected []int
	}{}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			k := removeDuplicates(test.nums)

			if diff := cmp.Diff(test.expected, test.nums[:k]); diff != "" {
				t.Errorf("removeDuplicates: (-want +got):\n%s", diff)
			}
		})
	}
}
