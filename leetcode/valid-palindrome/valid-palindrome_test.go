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

package validpalindrome

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestIsPalindrome(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name     string
		input    string
		expected bool
	}{
		{
			name:     "Case 1",
			input:    "A man, a plan, a canal: Panama",
			expected: true,
		},
		{
			name:     "Case 2",
			input:    "race a car",
			expected: false,
		},
		{
			name:     "Case 3",
			input:    " ",
			expected: true,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			t.Parallel()

			result := isPalindrome(test.input)
			if diff := cmp.Diff(test.expected, result); diff != "" {
				t.Errorf("isPalindrome(%q) mismatch (-want +got):\n%s", test.input, diff)
			}
		})
	}
}
