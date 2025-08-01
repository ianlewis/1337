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

// Package ransomnote implements LeetCode problem 383: Ransom Note.
//
// Given two strings ransomNote and magazine, return true if ransomNote can be
// constructed by using the letters from magazine and false otherwise.
//
// Each letter in magazine can only be used once in ransomNote.
//
// Example 1:
//
//	Input: ransomNote = "a", magazine = "b"
//	Output: false
//
// Example 2:
//
//	Input: ransomNote = "aa", magazine = "ab"
//	Output: false
//
// Example 3:
//
//	Input: ransomNote = "aa", magazine = "aab"
//	Output: true
//
// Constraints:
//
//	1 <= ransomNote.length, magazine.length <= 105
//	ransomNote and magazine consist of lowercase English letters.
package ransomnote

func canConstruct(ransomNote, magazine string) bool {
	// Add up all occurrences of each letter.
	m := make(map[rune]int, len(magazine))
	for _, r := range magazine {
		if c, ok := m[r]; ok {
			m[r] = c + 1
		} else {
			m[r] = 1
		}
	}

	for _, r := range ransomNote {
		if c, ok := m[r]; ok {
			if c == 0 {
				return false
			}

			m[r] = c - 1 // Remove a letter from the magazine
		} else {
			return false
		}
	}

	return true
}
