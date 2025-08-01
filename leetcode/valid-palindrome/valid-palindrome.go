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

// Package validpalindrome implements LeetCode problem 125: Valid Palindrome.
//
// A phrase is a palindrome if, after converting all uppercase letters into
// lowercase letters and removing all non-alphanumeric characters, it reads the
// same forward and backward. Alphanumeric characters include letters and
// numbers.
//
// Given a string s, return true if it is a palindrome, or false otherwise.
//
// Example 1:
//
//	Input: s = "A man, a plan, a canal: Panama"
//	Output: true
//	Explanation: "amanaplanacanalpanama" is a palindrome.
//
// Example 2:
//
//	Input: s = "race a car"
//	Output: false
//	Explanation: "raceacar" is not a palindrome.
//
// Example 3:
//
//	Input: s = " "
//	Output: true
//	Explanation: s is an empty string "" after removing non-alphanumeric
//	characters.
//	Since an empty string reads the same forward and backward, it is a
//	palindrome.
package validpalindrome

import "strings"

func isPalindrome(s string) bool {
	s = strings.ToLower(s)

	i := 0
	j := len(s) - 1
	for i < j {
		left := s[i]
		if (left < '0' || left > '9') && (left < 'a' || left > 'z') {
			i++
			continue
		}

		right := s[j]
		if (right < '0' || right > '9') && (right < 'a' || right > 'z') {
			j--
			continue
		}

		if s[i] != s[j] {
			return false
		}
		i++
		j--
	}

	return true
}
