use winit::window::Window;

pub use platform::render;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod platform {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::mem;
    use std::mem::ManuallyDrop;
    use std::num::NonZeroU32;

    use softbuffer::{Context, Surface};
    use winit::window::{self, Window, WindowId};

    thread_local! {
        // NOTE: You should never do things like that, create context and drop it before
        // you drop the event loop. We do this for brevity to not blow up examples. We use
        // ManuallyDrop to prevent destructors from running.
        //
        // A static, thread-local map of graphics contexts to open windows.
        static GC: ManuallyDrop<RefCell<Option<GraphicsContext>>> = const { ManuallyDrop::new(RefCell::new(None)) };
    }

    /// The graphics context used to draw to a window.
    struct GraphicsContext {
        /// The global softbuffer context.
        context: RefCell<Context<&'static Window>>,

        /// The hash map of window IDs to surfaces.
        surfaces: HashMap<WindowId, Surface<&'static Window, &'static Window>>,
    }

    impl GraphicsContext {
        fn new(w: &Window) -> Self {
            Self {
                context: RefCell::new(
                    Context::new(unsafe {
                        mem::transmute::<&'_ Window, &'static Window>(w)
                    })
                    .expect("Failed to create a softbuffer context"),
                ),
                surfaces: HashMap::new(),
            }
        }

        fn create_surface(
            &mut self,
            window: &Window,
        ) -> &mut Surface<&'static Window, &'static Window> {
            self.surfaces.entry(window.id()).or_insert_with(|| {
                Surface::new(&self.context.borrow(), unsafe {
                    mem::transmute::<&'_ Window, &'static Window>(window)
                })
                .expect("Failed to create a softbuffer surface")
            })
        }

        fn destroy_surface(&mut self, window: &Window) {
            self.surfaces.remove(&window.id());
        }
    }

    pub struct Renderer {
        width: usize,
        height: usize,
        buffer: Vec<u8>,
        window: Window
    }

    pub fn render(window: &Window) {
        GC.with(|gc| {
            let size = window.inner_size();
            let (Some(width), Some(height)) =
                (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
            else {
                return;
            };

            // Either get the last context used or create a new one.
            let mut gc = gc.borrow_mut();
            let surface = gc
                .get_or_insert_with(|| GraphicsContext::new(&window))
                .create_surface(&window);

            // Fill the buffer with a solid color (example: red)
            let color = 0xFF0000; // Red
            surface.resize(width, height).expect("Failed to resize the softbuffer surface");

            let mut buffer = surface.buffer_mut().expect("Failed to get the softbuffer buffer");
            buffer.fill(color);
            buffer.present().expect("Failed to present the softbuffer buffer");
        });
    }
}


pub struct Camera {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,

    pub transform: glam::Mat4,
    pub projection: glam::Mat4,

    pub width: f32,
    pub height: f32,
}

impl Camera {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32 , near: f32, far: f32) -> Self {
        Self {
            left,
            right,
            bottom,
            top,
            near,
            far,

            transform: glam::Mat4::IDENTITY,
            projection: glam::Mat4::IDENTITY,

            width: right - left,
            height: top - bottom,
        }
    }

    pub fn ratio(&self) -> f32 {
        self.width / self.height
    }

    pub fn update(&mut self) {
        self.projection = glam::Mat4::orthographic_rh_gl(self.left, self.right, self.bottom, self.top, self.near, self.far);
    }
}