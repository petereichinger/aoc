use std::time::{Duration, Instant};
use itertools::Itertools;
use log::{error, info};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use utils::{read_input_by_lines, ReadIterator};

const SCREEN_WIDTH: u32 = 10;
const SCREEN_HEIGHT: u32 = 10;

/// Create a window for the game.
///
/// Automatically scales the window to cover about 2/3 of the monitor height.
///
/// # Returns
///
/// Tuple of `(window, surface, width, height, hidpi_factor)`
/// `width` and `height` are in `PhysicalSize` units.
fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = SCREEN_WIDTH as f64;
    let height = SCREEN_HEIGHT as f64;
    let (monitor_width, monitor_height) = {
        if let Some(monitor) = window.current_monitor() {
            let size = monitor.size().to_logical(hidpi_factor);
            (size.width, size.height)
        } else {
            (width, height)
        }
    };
    let scale = (monitor_height / height * 2.0 / 3.0).round().max(1.0);

    // Resize, center, and display the window
    let min_size: winit::dpi::LogicalSize<f64> =
        PhysicalSize::new(width, height).to_logical(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let size = default_size.to_physical::<f64>(hidpi_factor);

    (
        window,
        size.width.round() as u32,
        size.height.round() as u32,
        hidpi_factor,
    )
}

type SimBoard = [[u8; 10]; 10];

struct FlashFishiesSim {
    state: SimBoard,
    swap_state: SimBoard,
    update_timer: f32,
}


impl FlashFishiesSim {
    fn from_input(lines: ReadIterator) -> FlashFishiesSim {
        let mut sim = FlashFishiesSim {
            state: [[0; 10]; 10],
            swap_state: [[0; 10]; 10],
            update_timer: 1.0,
        };

        for (y, line) in lines.enumerate() {
            for (x, char) in line.chars().enumerate() {
                sim.state[x][y] = char.to_digit(10).unwrap() as u8;
            }
        }

        sim
    }

    fn new_increasing()
        -> FlashFishiesSim {
        let mut sim = FlashFishiesSim {
            state: [[0; 10]; 10],
            swap_state: [[0; 10]; 10],
            update_timer: 1.0
        };

        for (val, fish) in (0..100u8).map(|n| n % 10).zip(sim.state.iter_mut().flatten().rev()) {
            *fish = val;
        }

        sim
    }

    fn draw(&self, pixels: &mut [u8]) {
        let fishies = self.state.iter().flatten();
        for (fish, pixel) in fishies.zip(pixels.chunks_exact_mut(4)) {
            let fish: u8 = if *fish == 9 { 0xff } else { 0x0 };
            pixel.copy_from_slice(&[fish, fish, fish, 0xff])
        }
    }

    fn update(&mut self, duration: Duration) {
        self.update_timer -= duration.as_secs_f32();
        if self.update_timer <= 0.0 {
            for col in 0..10 {
                for row in 0..10 {
                    self.swap_state[col][row] = (self.state[col][row] + 1) % 10
                }
            }

            std::mem::swap(&mut self.state, &mut self.swap_state);
            self.update_timer = 1.0
        }
    }
}


fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let mut sim = FlashFishiesSim::from_input(read_input_by_lines());

    let (window, p_width, p_height, mut _hidpi_factor) =
        create_window("Advent Of Code Flashy Fishies", &event_loop);


    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);
    let mut pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?;

    let mut time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            sim.draw(pixels.get_frame());
            if pixels.render().map_err(|e| error!("pixels.render() failed: {}", e)).is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            let now = Instant::now();
            let dt = now.duration_since(time);
            time = now;
            sim.update(dt);
            window.request_redraw();
        }
    });
}