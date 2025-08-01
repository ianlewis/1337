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

// Package meetingrooms implements LeetCode problem 252: Meeting Rooms.
//
// Given an array of meeting time intervals where intervals[i] = [starti, endi],
// determine if a person could attend all meetings.
//
// Example 1:
//
//	Input: intervals = [[0,30],[5,10],[15,20]]
//	Output: false
//
// Example 2:
//
//	Input: intervals = [[7,10],[2,4]]
//	Output: true
//
// Constraints:
//
//	0 <= intervals.length <= 104
//	intervals[i].length == 2
//	0 <= starti < endi <= 106
package meetingrooms

import "sort"

func canAttendMeetings(intervals [][]int) bool {
	sort.Slice(intervals, func(i, j int) bool {
		if intervals[i][0] == intervals[j][0] && intervals[i][1] < intervals[j][1] {
			return true
		}

		return intervals[i][0] < intervals[j][0]
	})

	for i := range len(intervals) - 1 {
		if intervals[i][1] > intervals[i+1][0] {
			// Next meeting starts before this one ends.
			return false
		}
	}

	return true
}
