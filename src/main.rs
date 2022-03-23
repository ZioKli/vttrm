use std::env;
use std::fs;
use std::io::Write;

fn run(filename: &str) -> Result<(), String> {
    if !filename.ends_with(".vtt") {
        return Err("argument must end with .vtt".into());
    }

    let f = fs::read_to_string(filename).or(Err(format!(
        "File {} cannot be opened or does not exist",
        filename
    )))?;

    let phrase = f
        .split("<v")
        .map(|w| w.trim_start_matches(" >").trim_end_matches("</v>"))
        .collect::<Vec<&str>>();

    let dirty_words = phrase[1..]
        .into_iter()
        .map(|p| {
            *p.split("</v>")
                .collect::<Vec<&str>>()
                .first()
                .unwrap_or(&"")
        })
        .map(|p| p.trim().trim_end_matches("</v>"))
        .collect::<Vec<&str>>();

    let words = dirty_words
        .into_iter()
        .map(|w| w.replace(">", ": "))
        .collect::<Vec<String>>();

    let outfile = format!("{}{}", filename.trim_end_matches(".vtt"), ".txt");
    match fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&outfile)
    {
        Ok(mut out) => {
            for w in words {
                out.write(&w.as_bytes()).or(Err(format!(
                    "Could not write \"{}\" to file {}",
                    w, outfile
                )))?;
                out.write(&[0x0A, 0x0A])
                    .or(Err(format!("Could not write new line to file {}", outfile)))?;
            }
        }
        Err(_) => return Err(format!("Could not open \"{}\"", outfile)),
    }

    Ok(())
}
fn main() {
    let args = env::args().collect::<Vec<String>>();
    match args.get(1) {
        Some(a) => match run(a) {
            Ok(_) => std::process::exit(0),
            Err(e) => {
                println!("{}", e);
                std::process::exit(1)
            }
        },
        None => {
            println!("USAGE: vttrm <Filename>.vtt");
            std::process::exit(1)
        }
    }
}
