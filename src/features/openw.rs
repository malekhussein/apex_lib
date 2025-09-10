use minifb::{Window, WindowOptions};

pub fn oapex_window() {
    let width = 400;
    let height = 400;
    let buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(
        "My Library Window",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Failed to create window: {}", e);
    });

    
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
