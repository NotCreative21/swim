extern crate x11rb;

pub mod ewmh;
pub mod ipc;
pub mod wm;

use crate::x11rb::connection::Connection;

// TODO 
// thread pool to handle resize and movment events to reduce any unsmooth experiences
fn main() {
    let (conn, scrno) = x11rb::connect(None).unwrap();
    let mut manager = match wm::WindowManager::new(&conn, scrno) {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("\x1b[31merror while connecting: {}", e);
            std::process::exit(1);
        }
    };
    loop {
        let _ = conn.flush();
        let ev = match conn.wait_for_event() {
            Ok(ev) => ev,
            Err(_) => continue,
        };
        // errors do not matter, we can skip them
        let _ = manager.dispatch_event(&ev);
    }
}
