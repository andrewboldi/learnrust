use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Simple Pixel Drawing - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Example: Draw a simple gradient
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let r = (x as u32) & 0xFF;
                let g = (y as u32) & 0xFF;
                let b = ((x + y) as u32) & 0xFF;
                buffer[y * WIDTH + x] = (r << 16) | (g << 8) | b;
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

