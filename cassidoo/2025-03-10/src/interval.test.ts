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

import { findLargestInterval } from "./interval.js";

describe("findLargestInterval", () => {
  it("should return 7 for ['A0', 'C1', 'G1', 'C2']", () => {
    expect(findLargestInterval(["A0", "C1", "G1", "C2"])).toBe(7);
  });

  it("should return 17 for ['C4', 'G4', 'C5', 'G3']", () => {
    expect(findLargestInterval(["C4", "G4", "C5", "G3"])).toBe(17);
  });

  it("should return 53 for ['E2', 'C3', 'G3', 'C8']", () => {
    expect(findLargestInterval(["E2", "C3", "G3", "C8"])).toBe(53);
  });

  it("should throw an error for out of range values", () => {
    expect(() => {
      findLargestInterval(["A8", "A0"]);
    }).toThrow(new Error("Note out of range: A8"));

    expect(() => {
      findLargestInterval(["C8", "Ab0"]);
    }).toThrow(new Error("Note out of range: Ab0"));
  });

  it("should throw an error for invalid notes", () => {
    expect(() => {
      findLargestInterval(["H8", "A0"]);
    }).toThrow(new Error("Invalid note: H8"));

    expect(() => {
      findLargestInterval(["a9", "Ab0"]);
    }).toThrow(new Error("Invalid note: a9"));
  });
});
