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

const notes = {
  // NOTE: C starts the octave
  // Each note is represented by its semitone value relative to C in an octave.
  C: 0,
  D: 2,
  E: 4,
  F: 5,
  G: 7,
  A: 9,
  B: 11,
};
const minVal = 9 + 0 * 12; // A0
const maxVal = 3 + 8 * 12; // C8

// findLargestInterval returns the largest interval between consecutive
// semitones in the array. Members of the array are expected to be in the format
// "NoteOctave" where Note is a string representation of the note Octave is a
// number. For example, "C4", "D#5", "Bb3" are valid inputs. An Error is thrown
// if the input is not in the expected format or if a note is out of range of
// notes on a piano (A0 - C8).
export function findLargestInterval(arr: string[]): number {
  // Each note is converted to a semitone value from 9 (A0) to 99 (C8).
  // This is calulated by taking the note value and adding it to the octave
  // multiplied by 12.
  // The interval (difference) between adjacent notes is calculated, and the
  // largest interval is returned.

  let maxInterval = 0;
  let prevValue = -1;
  for (let i = 0; i < arr.length; i++) {
    const noteStr = arr[i];

    // Validate the note format and get the note portion of the semitone value.
    const note = noteStr[0];
    let noteValue = notes[note as keyof typeof notes];
    if (noteValue === undefined) {
      throw new Error(`Invalid note: ${noteStr}`);
    }

    // If sharp or flat is specified, adjust the note index accordingly.
    let nextIndex = 1;
    if (noteStr[1] === "#" || noteStr[1] === "♯") {
      // sharp
      noteValue += 1;
      nextIndex = 2;
    } else if (noteStr[1] === "b" || noteStr[1] === "♭") {
      // flat
      noteValue -= 1;
      nextIndex = 2;
    }

    // Use the note value and octave value to calculate the semitone value.
    const semitoneValue = noteValue + parseInt(noteStr[nextIndex], 10) * 12;

    // Ensure the semitone value is within the valid range.
    if (semitoneValue < minVal || semitoneValue > maxVal) {
      throw new Error(`Note out of range: ${noteStr}`);
    }

    if (prevValue !== -1) {
      const interval = Math.abs(semitoneValue - prevValue);
      if (interval > maxInterval) {
        maxInterval = interval;
      }
    }

    prevValue = semitoneValue;
  }

  return maxInterval;
}
