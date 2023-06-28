use std::{
    io,
    io::{Seek, SeekFrom, Write},
    path::Path,
};

use fs_err as fs;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let paths = std::env::args().skip(1).collect::<Vec<_>>();

    for path in paths {
        let places_ruined = ruin_code_at(&path)?;

        if places_ruined == 0 {
            println!("Nothing to ruin at {path}.");
        } else {
            println!("Did {places_ruined} changes and ruined {path} successfully.");
        }
    }

    Ok(())
}

fn ruin_code_at(path: impl AsRef<Path>) -> io::Result<u64> {
    let contents = fs::read_to_string(path.as_ref())?;

    let mut file = fs::OpenOptions::new().write(true).open(path.as_ref())?;

    let mut ruin_count = 0;

    for (index, ch) in contents.char_indices() {
        if let Some(reversed) = reversed_counterpart(ch) {
            // Safety: `u64` memory is simply too much
            file.seek(SeekFrom::Start(index as u64))?;
            // Safety: `reversed_counterpart` only returns `Some` for ascii chars.
            file.write_all([reversed].as_slice())?;

            ruin_count += 1;
        }
    }

    Ok(ruin_count)
}

fn reversed_counterpart(ch: char) -> Option<u8> {
    match ch {
        '(' => Some(b')'),
        ')' => Some(b'('),
        '[' => Some(b']'),
        ']' => Some(b'['),
        '{' => Some(b'}'),
        '}' => Some(b'{'),
        '<' => Some(b'>'),
        '>' => Some(b'<'),
        _ => None,
    }
}
