use std::ops;


#[derive(Copy, Clone)]
pub struct Note {
    pub pitch: usize
}

impl Note {

    pub fn middle_c() -> Note {
        Note { pitch: 3 }
    }

    pub fn spell(&self, flat: bool) -> String {
        match self.pitch {
            0 => "A",
            1 => if flat {"Bb"} else {"A#"}, 
            2 => "B",
            3 => "C",
            4 => if flat {"Db"} else {"C#"},
            5 => "D",
            6 => if flat {"Eb"} else {"D#"},
            7 => "E",
            8 => "F",
            9 => if flat {"Gb"} else {"F#"},
            10 => "G",
            11 => if flat {"Ab"} else {"G#"},
            _  => { return format!("Invalid pitch '{}'", self.pitch)},
        }.to_string()
    }

    // Return only the letter of the spelling (omitting sharp/flat). Used to help
    // determine scale flatness. Do I feel good about this...? no....
    pub fn letter(&self, flat:bool) -> String {
        match self.pitch {
            0 => "A",
            1 => if flat {"B"} else {"A"}, 
            2 => "B",
            3 => "C",
            4 => if flat {"D"} else {"C"},
            5 => "D",
            6 => if flat {"E"} else {"D"},
            7 => "E",
            8 => "F",
            9 => if flat {"G"} else {"F"},
            10 => "G",
            11 => if flat {"A"} else {"G"},
            _  => { return format!("Invalid pitch '{}'", self.pitch)},
        }.to_string()
    }
}

impl ops::Add<usize> for Note {
    type Output = Self;

    fn add(self, other: usize) -> Self {
        Self {
            pitch: (self.pitch + other) % 12
        }
    }
}


