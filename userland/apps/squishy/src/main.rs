mod font;
mod grid;
mod parser;
mod pty;
mod render;
mod window;

fn main() {
    // let mut p = match pty::new(80, 24) {
    //     Ok(p) => {
    //         eprintln!("pty created, child pid = {}", p.child.id());
    //         p
    //     }
    //     Err(e) => {
    //         eprintln!("pty::new failed: {e}");
    //         return;
    //     }
    // };
    // p.read_from_file();
    // window::run();
}
