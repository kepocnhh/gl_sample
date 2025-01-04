extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    let w = 640;
    let h = 480;
    let (mut window, events) = glfw.create_window(w, h, "foobar", glfw::WindowMode::Windowed).unwrap();
    window.set_key_polling(true);
    window.make_current();
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
