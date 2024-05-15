use miniquad::EventHandler;
use quad_gl::{color::*, math::*, sprite_batcher::SpriteBatcher, Context3};
use std::sync::{Arc, Mutex};

struct Stage {
    ctx: Arc<Mutex<Box<miniquad::Context>>>,
    canvas1: SpriteBatcher,
    canvas2: SpriteBatcher,
    canvas3: SpriteBatcher,
}

impl Stage {
    pub fn new() -> Stage {
        let ctx = miniquad::window::new_rendering_backend();
        let ctx = Arc::new(Mutex::new(ctx));

        let graphics = Context3::new(ctx.clone());
        let mut canvas1 = graphics.new_canvas();
        let canvas2 = graphics.new_canvas();
        let canvas3 = graphics.new_canvas();

        // canvas1 is a static background canvas.
        // It will be never updated.
        canvas1.draw_rectangle(0.0, 0.0, 100.0, 100.0, RED);
        canvas1.draw_text("HELLO WORLD", 300.0, 300.0, 30.0, BLACK);

        Stage {
            ctx: ctx.clone(),
            canvas1,
            canvas2,
            canvas3,
        }
    }
}
impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        // some fake animations
        let t = (miniquad::date::now() - 1715800000.0) as f32;
        let p1 = vec2(
            (t * 0.1).sin() * 400.0 + 400.0,
            (t * 0.1).cos() * 200.0 + 200.0,
        );
        let p2 = vec2((t * 3.0).sin() * 400.0 + 800.0, t.cos() * 200.0 + 400.0);

        // normal miniquad clearing code
        self.ctx
            .lock()
            .unwrap()
            .clear(Some((1., 1., 1., 1.)), None, None);

        // canvas2 is an "additive" canvas. It holds its previous state
        // and will get an additiona red circle each frame.
        self.canvas2.draw_circle(p1.x, p1.y, 10.0, RED);

        // canvas3 is a "dynamic" canvas. starts from scratch each frame.
        // Useful for animated content.
        self.canvas3.clear();
        self.canvas3.draw_circle(p2.x, p2.y, 10.0, BLUE);

        // .draws order defines "Z" order, here canvas1 content will be
        // on the background and canvas3's blue circle will be on top of everything
        self.canvas1.draw();
        self.canvas2.draw();
        self.canvas3.draw();
    }
}

fn main() {
    miniquad::start(Default::default(), || Box::new(Stage::new()));
}
