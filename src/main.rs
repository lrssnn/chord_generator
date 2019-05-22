fn main() {

    for root in 2..3 {
        println!("{} major", spell(root, false));
        for note in major_scale(root) {
            print!("{} ", spell(note, false));
        }
        println!("{}", if validate_scale(major_scale(root), false) {"good"} else {"bad"});
    }
}

fn get_note_names(flats: bool) -> Vec<String> {
    let mut res = vec![];
    for i in 1..13 {
        res.push(spell(i, flats));
    }
    res
}

fn spell(note: i32, flat: bool) -> String {
    if note > 12 {
        return spell(note - 12, flat);
    }

    match note {
        1 => "A",
        2 => if flat {"Bb"} else {"A#"}, 
        3 => "B",
        4 => "C",
        5 => if flat {"Db"} else {"C#"},
        6 => "D",
        7 => if flat {"Eb"} else {"D#"},
        8 => "E",
        9 => "F",
        10 => if flat {"Gb"} else {"F#"},
        11 => "G",
        12 => if flat {"Ab"} else {"G#"},
        _  => "Invalid",
    }.to_string()
    
}

fn major_scale(r: i32) -> Vec<i32> {
    // T T S T T T S
    vec![r, r+2, r+4, r+5, r+7, r+9, r+11, r+12]
}

fn validate_scale(scale: Vec<i32>, flat: bool) -> bool {
    // We care about whether the scale as written contains consecutive letters e.g. G and G#
    // This means the scale requires a double sharps/flats and we don't want it
    
    for index in 0..7 {
        let note = scale[index];
        let next = scale[index + 1];

        print!("({} - {}) ", note, next);

        match note {
            1 => {if next == 2 {return false;}},
            _ => (),
        };

    }

    return true;
}
