use std::{env, fs::File, io::Write, path::PathBuf};

use serde_json::json;
use walkdir::WalkDir;

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

    let path = PathBuf::from(&args[1]);
    if !path.exists() {
        println!("Path doesn't exist: {path:?}");
        return;
    }

    if path.is_file() {
        let Ok(bytes) = std::fs::read(path) else {
            println!("Failed to read file.");
            return;
        };
        match midi_msg::MidiFile::from_midi(&bytes) {
            Ok(_) => println!("Success!"),
            Err(e) => println!("Failed:\n{}", e.to_string()),
        }
    } else {
        let mut files_ok = vec![];
        let mut files_err = vec![];
        let mut files_noimp = vec![];
        let mut files_skip = vec![];

        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            let path = entry.path();
            print!("Attempting {path:?} ");
            if path.is_file() && path.extension().is_some_and(|s| s == "mid") {
                let Ok(bytes) = std::fs::read(path) else {
                    println!("Failed to read file.");
                    files_skip.push(path.to_string_lossy().to_string());
                    continue;
                };
                match midi_msg::MidiFile::from_midi(&bytes) {
                    Ok(_) => {
                        println!("Success!");
                        files_ok.push(path.to_string_lossy().to_string());
                    }
                    Err(e) => match e.error {
                        midi_msg::ParseError::NotImplemented(_) => {
                            println!("Not Implemented!");
                            files_noimp.push(path.to_string_lossy().to_string())
                        }
                        _ => {
                            println!("Error!");
                            files_err.push(path.to_string_lossy().to_string());
                        }
                    },
                }
            }
        }
        println!("Files:");
        println!("OK:       {}", files_ok.len());
        println!("ERR:      {}", files_err.len());
        println!("NoImp:    {}", files_noimp.len());
        println!("Skipped:  {}", files_skip.len());

        let data = json!({
            "ok": files_ok,
            "err": files_err,
            "noimp": files_noimp,
            "skip": files_skip,
        });
        let mut file = File::create("midi_msg_log.json").expect("File create fail");
        file.write_all(data.to_string().as_bytes())
            .expect("data write fail");
    }

    println!("goodbye.");
}
