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

package ransomnote

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestCanConstruct(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name     string
		ransom   string
		magazine string
		expected bool
	}{
		{
			name:     "Case 1",
			ransom:   "a",
			magazine: "b",
			expected: false,
		},
		{
			name:     "Case 2",
			ransom:   "aa",
			magazine: "ab",
			expected: false,
		},
		{
			name:     "Case 3",
			ransom:   "aa",
			magazine: "aab",
			expected: true,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			result := canConstruct(test.ransom, test.magazine)
			if diff := cmp.Diff(test.expected, result); diff != "" {
				t.Errorf("canConstruct(%q, %q) mismatch (-want +got):\n%s", test.ransom, test.magazine, diff)
			}
		})
	}
}
