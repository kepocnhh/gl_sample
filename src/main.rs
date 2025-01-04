use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    let w = 640;
    let h = 480;
    let (mut window, events) = glfw.create_window(w, h, "foobar", glfw::WindowMode::Windowed).unwrap();
    window.set_key_polling(true);
    window.make_current();
    gl::load_with(|ptr| window.get_proc_address(ptr));
    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }
    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
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
