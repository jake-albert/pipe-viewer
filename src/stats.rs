use std::io::Result;
use std::sync::{Arc, Mutex};

pub fn stats_loop(silent: bool, quit: Arc<Mutex<bool>>) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        // TODO(jake): Receive bytes from read thread. For now we use empty
        let buffer: Vec<u8> = Vec::new();
        total_bytes += buffer.len();
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        // TODO(jake): Send vector to write loop
        let quit = quit.lock().unwrap();
        if *quit {
            break;
        }
    }
    eprintln!();
    Ok(())
}
