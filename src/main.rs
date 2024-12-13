use std::env;

fn main() {
    println!("### midi-msg file parser ###\n");

    let args: Vec<String> = env::args().collect();

    println!("args:");
    dbg!(&args);
    println!(" ");

    if args.len() == 1 {
        println!("usage:\n $ midi_msg_test file/path/here.mid");
        return;
    } else if args.len() > 2 {
        println!("too many args");
        return;
    }

    let filepath = &args[1];

    let Ok(bytes) = std::fs::read(filepath) else {
        println!("Failed to read file.");
        return;
    };
    match midi_msg::MidiFile::from_midi(&bytes) {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Failed:\n{}", e.to_string()),
    }
    println!("goodbye.");
}
