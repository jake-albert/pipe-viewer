use std::{
    fs::File,
    io::{self, BufWriter, ErrorKind, Result, Write},
    sync::{Arc, Mutex},
};

pub fn write_loop(outfile: &str, quit: Arc<Mutex<bool>>) -> Result<()> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    loop {
        // TODO(jake): Receive bytes from stats thread. For now we use empty
        let buffer: Vec<u8> = Vec::new();

        // Use a narrower scope so we quickly release the lock on quit
        {
            let quit = quit.lock().unwrap();
            if *quit {
                break;
            }
        }

        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                // Stop the program cleanly
                return Ok(());
            }
            return Err(e);
        }
    }

    Ok(())
}
