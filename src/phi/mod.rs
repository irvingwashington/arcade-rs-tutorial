#[macro_use]
mod events;
pub mod data;
pub mod gfx;

use sdl2::render::Renderer;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right,
        key_space: Space
    },
    else: {
        quit: Quit { .. }
    }
}


/// Bundles the Phi abstractions in a single structure which
/// can be passed easily between functions.
pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
}

impl<'window> Phi<'window> {
    pub fn new(events: Events, renderer: Renderer<'window>) -> Phi<'window> {
        Phi {
            events: events,
            renderer: renderer,
        }
    }

    pub fn output_size(&self) -> (f64, f64) {
        let (w, h) = self.renderer.output_size().unwrap();
        (w as f64, h as f64)
    }
}

/// A `ViewAction` is a way for the currently executed view to
/// communicate with the game loop. It specifies which action
/// should be executed before the next rendering.
pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}


pub trait View {
    /// Called on every frame to take care of both the logic and
    /// the rendering of the current view.
    ///
    /// `elapsed` is expressed in seconds.
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

pub fn spawn<F>(title: &str, init: F) where F: Fn(&mut Phi) -> Box<View> {
    // Initialize SDL2
    let sdl_context = ::sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let _image_context = ::sdl2::image::init(::sdl2::image::INIT_PNG).unwrap();

    // Create the window
    let window = video.window(title, 800, 600)
        .position_centered().opengl().resizable()
        .build().unwrap();

    // Create the context
    let mut context = Phi::new(
        Events::new(sdl_context.event_pump().unwrap()),
        window.renderer()
        .accelerated()
        .build().unwrap()
    );

    // Create the default view
    let mut current_view = init(&mut context);


    // Frame timing

    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    loop {
        // Frame timing (bis)

        let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;

        // If the time elapsed since the last frame is too small, wait out the
        // difference and try again.
        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }


        // Logic & rendering

        context.events.pump(&mut context.renderer);

        match current_view.render(&mut context, elapsed) {
            ViewAction::None =>
                context.renderer.present(),

            ViewAction::Quit =>
                break,

            ViewAction::ChangeView(new_view) =>
                current_view = new_view,
        }
    }
}