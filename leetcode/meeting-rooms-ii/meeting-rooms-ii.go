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

// Package meetingroomsii implements LeetCode problem 253: Meeting Rooms II.
package meetingroomsii

import (
	"container/heap"
	"sort"
)

// Room represents a meeting room with an end time.
type Room struct {
	end int
}

// RoomQueue is a priority queue of Rooms, ordered by their end time.
//
//nolint:recvcheck // some methods on RoomQueue are read-only
type RoomQueue []*Room

// Len implements heap.Interface.Len.
func (q RoomQueue) Len() int {
	return len(q)
}

// Less implements heap.Interface.Less.
func (q RoomQueue) Less(i, j int) bool {
	// Rooms that free up first are higher priority.
	// i.e. if the ending is later, it has less priority.
	return q[i].end < q[j].end
}

// Swap implements heap.Interface.Swap.
func (q RoomQueue) Swap(i, j int) {
	q[i], q[j] = q[j], q[i]
}

// Push implements heap.Interface.Push.
func (q *RoomQueue) Push(x any) {
	// add x as element Len()
	//nolint:errcheck,forcetypeassert // If not a *Room this should panic.
	r := x.(*Room)
	*q = append(*q, r)
}

// Pop implements heap.Interface.Pop.
func (q *RoomQueue) Pop() any {
	// remove and return element Len() - 1.
	qp := *q
	r := qp[len(qp)-1]
	qp[len(qp)-1] = nil
	*q = qp[:len(qp)-1]

	return r
}

func minMeetingRooms(intervals [][]int) int {
	sort.Slice(intervals, func(i, j int) bool {
		if intervals[i][0] == intervals[j][0] && intervals[i][1] < intervals[j][1] {
			return true
		}

		return intervals[i][0] < intervals[j][0]
	})

	roomQueue := make(RoomQueue, 0)
	heap.Init(&roomQueue)

	for i := range intervals {
		var r *Room

		if roomQueue.Len() > 0 {
			//nolint:errcheck,forcetypeassert // If not a *Room this should panic.
			if c := heap.Pop(&roomQueue).(*Room); c.end <= intervals[i][0] {
				// We can use this room
				r = c
			} else {
				// Push the room we can't reuse back on the heap.
				heap.Push(&roomQueue, c)
			}
		}

		if r == nil {
			// allocate new
			r = &Room{}
		}

		r.end = intervals[i][1]
		heap.Push(&roomQueue, r)
	}

	return roomQueue.Len()
}
