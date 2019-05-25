mod note;
use note::Note;

fn main() {


    let mut a = Note { pitch: 0 };

    for root in 0..12 {
        let scale = major_scale( Note { pitch:root });
        let flat = is_flat_scale(&scale);
        for n in scale {
          print!("{} ", n.spell(flat));
        }
        println!();
    }


    /*
    for root in 2..3 {
        println!("{} major", spell(root, false));
        for note in major_scale(root) {
            print!("{} ", spell(note, false));
        }
        println!("{}", if validate_scale(major_scale(root), false) {"good"} else {"bad"});
    }
    */
}

fn major_scale(r: Note) -> Vec<Note> {
    // T T S T T T S
    vec![r, r+2, r+4, r+5, r+7, r+9, r+11, r+12]
}

fn is_flat_scale(scale: &Vec<Note>) -> bool {
    // We care if the scale would contain consecutive letter names if written in sharps
    // e.g. G and G#, instead it should be G and Ab. 

    for index in 0..scale.len() - 1 {
        let note = scale[index];
        let next = scale[index + 1];

        if (note.letter(false) == next.letter(false)) {
            return true;
        }
    }

    return false;
    
}
