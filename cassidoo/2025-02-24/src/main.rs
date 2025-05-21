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

use std::env;
use std::process;

fn get_note_names(frequencies: &[f64]) -> Vec<String> {
    // Map frequencies to note names.
    let octaves = vec![
        vec![
            (8.175799, "C"),
            (8.661957, "C♯/D♭"),
            (9.177024, "D"),
            (9.722718, "D♯/E♭"),
            (10.30086, "E"),
            (10.91338, "F"),
            (11.56233, "F♯/G♭"),
            (12.24986, "G"),
            (12.97827, "G♯/A♭"),
            (13.75000, "A"),
            (14.56762, "A♯/B♭"),
            (15.43385, "B"),
        ],
        vec![
            (16.35160, "C"),
            (17.32391, "C♯/D♭"),
            (18.35405, "D"),
            (19.44544, "D♯/E♭"),
            (20.60172, "E"),
            (21.82676, "F"),
            (23.12465, "F♯/G♭"),
            (24.49971, "G"),
            (25.95654, "G♯/A♭"),
            (27.50000, "A"),
            (29.13524, "A♯/B♭"),
            (30.86771, "B"),
        ],
        vec![
            (32.70320, "C"),
            (34.64783, "C♯/D♭"),
            (36.70810, "D"),
            (38.89087, "D♯/E♭"),
            (41.20344, "E"),
            (43.65353, "F"),
            (46.24930, "F♯/G♭"),
            (48.99943, "G"),
            (51.91309, "G♯/A♭"),
            (55.00000, "A"),
            (58.27047, "A♯/B♭"),
            (61.73541, "B"),
        ],
        vec![
            (65.40639, "C"),
            (69.29566, "C♯/D♭"),
            (73.41619, "D"),
            (77.78175, "D♯/E♭"),
            (82.40689, "E"),
            (87.30706, "F"),
            (92.49861, "F♯/G♭"),
            (97.99886, "G"),
            (103.8262, "G♯/A♭"),
            (110.0000, "A"),
            (116.5409, "A♯/B♭"),
            (123.4708, "B"),
        ],
        vec![
            (130.8128, "C"),
            (138.5913, "C♯/D♭"),
            (146.8324, "D"),
            (155.5635, "D♯/E♭"),
            (164.8138, "E"),
            (174.6141, "F"),
            (184.9972, "F♯/G♭"),
            (195.9977, "G"),
            (207.6523, "G♯/A♭"),
            (220.0000, "A"),
            (233.0819, "A♯/B♭"),
            (246.9417, "B"),
        ],
        vec![
            (261.6256, "C"),
            (277.1826, "C♯/D♭"),
            (293.6648, "D"),
            (311.1270, "D♯/E♭"),
            (329.6276, "E"),
            (349.2282, "F"),
            (369.9944, "F♯/G♭"),
            (391.9954, "G"),
            (415.3047, "G♯/A♭"),
            (440.0000, "A"),
            (466.1638, "A♯/B♭"),
            (493.8833, "B"),
        ],
        vec![
            (523.2511, "C"),
            (554.3653, "C♯/D♭"),
            (587.3295, "D"),
            (622.2540, "D♯/E♭"),
            (659.2551, "E"),
            (698.4565, "F"),
            (739.9888, "F♯/G♭"),
            (783.9909, "G"),
            (830.6094, "G♯/A♭"),
            (880.0000, "A"),
            (932.3275, "A♯/B♭"),
            (987.7666, "B"),
        ],
        vec![
            (1046.502, "C"),
            (1108.731, "C♯/D♭"),
            (1174.659, "D"),
            (1244.508, "D♯/E♭"),
            (1318.510, "E"),
            (1396.913, "F"),
            (1479.978, "F♯/G♭"),
            (1567.982, "G"),
            (1661.219, "G♯/A♭"),
            (1760.000, "A"),
            (1864.655, "A♯/B♭"),
            (1975.533, "B"),
        ],
        vec![
            (2093.005, "C"),
            (2217.461, "C♯/D♭"),
            (2349.318, "D"),
            (2489.016, "D♯/E♭"),
            (2637.020, "E"),
            (2793.826, "F"),
            (2959.955, "F♯/G♭"),
            (3135.963, "G"),
            (3322.438, "G♯/A♭"),
            (3520.000, "A"),
            (3729.310, "A♯/B♭"),
            (3951.066, "B"),
        ],
        vec![
            (4186.009, "C"),
            (4434.922, "C♯/D♭"),
            (4698.636, "D"),
            (4978.032, "D♯/E♭"),
            (5274.041, "E"),
            (5587.652, "F"),
            (5919.911, "F♯/G♭"),
            (6271.927, "G"),
            (6644.875, "G♯/A♭"),
            (7040.000, "A"),
            (7458.620, "A♯/B♭"),
            (7902.133, "B"),
        ],
        vec![
            (8372.018, "C"),
            (8869.844, "C♯/D♭"),
            (9397.273, "D"),
            (9956.063, "D♯/E♭"),
            (10548.08, "E"),
            (11175.30, "F"),
            (11839.82, "F♯/G♭"),
            (12543.85, "G"),
            (13289.75, "G♯/A♭"),
            (14080.00, "A"),
            (14917.24, "A♯/B♭"),
            (15804.27, "B"),
        ],
        vec![
            (16744.04, "C"),
            (17739.69, "C♯/D♭"),
            (18794.55, "D"),
            (19912.13, "D♯/E♭"),
            (21096.16, "E"),
            (22350.61, "F"),
            (23679.64, "F♯/G♭"),
            (25087.71, "G"),
            (26579.50, "G♯/A♭"),
            (28160.00, "A"),
            (29834.48, "A♯/B♭"),
            (31608.53, "B"),
        ],
    ];

    let mut note_names = Vec::new();
    for frequency in frequencies {
        let mut found = false;
        let mut prev_freq = (0.0, "");
        for notes in octaves.iter() {
            for (note_freq, name) in notes {
                // Check within range of 0.01. It's not totally clear the precision that we should
                // use here.
                if (frequency - note_freq).abs() < 0.01 {
                    note_names.push(format!("This is a {}", name));
                    found = true;
                    break;
                } else if frequency < note_freq {
                    // The note is between this frequency and the previous one.
                    // Determine which it's closer to.
                    if frequency - prev_freq.0 < note_freq - frequency {
                        // The note is sharp of the previous note.
                        note_names.push(format!("This is a {}, but it's sharp", prev_freq.1));
                        found = true;
                        break;
                    } else {
                        // The note is flat of the current note.
                        note_names.push(format!("This is a {}, but it's flat", name));
                        found = true;
                        break;
                    }
                }
                prev_freq = (*note_freq, name);
            }
            if found {
                break;
            }
        }
        if !found {
            note_names.push(format!("Unknown frequency: {}", frequency));
        }
    }

    note_names
}

fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <frequency>...", args[0]);
        return process::ExitCode::from(1);
    }

    let frequencies = match args[1..]
        .iter()
        .map(|s| s.parse::<f64>())
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(frequencies) => frequencies,
        Err(e) => {
            eprintln!("Error: {}", e);
            return process::ExitCode::from(1);
        }
    };

    let note_names = get_note_names(&frequencies);
    for name in note_names {
        println!("{}", name);
    }

    process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error;

    #[test]
    fn test_get_note_names() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            get_note_names(&[440.0, 490.0, 524.0, 293.66]),
            vec![
                "This is a A",
                "This is a B, but it's flat",
                "This is a C, but it's sharp",
                "This is a D"
            ]
        );

        Ok(())
    }
}
