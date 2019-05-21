fn main() {

    println!("Hello, world!");

    println!("Notes:");

    for note in get_note_names(false) {
        println!("-> {}", note);
    }

    println!("Flats:");

    for note in get_note_names(true) {
        println!("-> {}", note);
    }

}

fn get_note_names(flats: bool) -> Vec<String> {
    let mut res = vec![];
    for i in 1..13 {
        res.push(get_note_name(i, flats));
    }
    res
}

fn get_note_name(note: i32, flat: bool) -> String {
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
        _  => "NO!",
    }.to_string()
}

