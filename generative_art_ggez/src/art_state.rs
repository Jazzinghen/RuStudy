use ggez::event;
use ggez::graphics;
use ggez::mint;
use ggez::timer;
use ggez::Context;
use ggez::GameResult;
use rand::prelude::*;

pub struct ArtState {
    art_refresh: std::time::Duration,
    shapes: Vec<Shape>,
    delta_t: std::time::Duration,
    rng: ThreadRng,
}

impl ArtState {
    pub fn new(art_refresh: std::time::Duration) -> ArtState {
        ArtState {
            art_refresh,
            delta_t: std::time::Duration::new(0, 0),
            shapes: Vec::new(),
            rng: thread_rng(),
        }
    }

    fn generate_shapes(&mut self, amount: u16) {
        for _ in 1..amount {
            if self.rng.gen() {
                self.shapes.push(Shape::Rectangle(ggez::graphics::Rect::new(
                    self.rng.gen_range(0.0, 800.0),
                    self.rng.gen_range(0.0, 600.0),
                    self.rng.gen_range(0.0, 800.0),
                    self.rng.gen_range(0.0, 600.0),
                )));
            } else {
                self.shapes.push(Shape::Circle(
                    mint::Point2 {
                        x: self.rng.gen_range(0.0, 800.0),
                        y: self.rng.gen_range(0.0, 600.0),
                    },
                    self.rng.gen_range(0.0, 300.0),
                ));
            }
        }
    }
}

pub enum Shape {
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}

impl event::EventHandler for ArtState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.delta_t += ggez::timer::delta(ctx);

        if self.delta_t >= self.art_refresh {
            self.shapes.clear();
            self.generate_shapes(8);

            self.delta_t = std::time::Duration::new(0, 0);
        };
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        for s in &self.shapes {
            let new_shape = match s {
                &Shape::Circle(centre, radius) => graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    centre,
                    radius,
                    0.1,
                    graphics::WHITE,
                )?,
                &Shape::Rectangle(rect) => graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    rect,
                    graphics::Color {
                        r: 1.0,
                        g: 0.0,
                        b: 1.0,
                        a: 1.0,
                    },
                )?,
            };
            graphics::draw(ctx, &new_shape, graphics::DrawParam::default())?;
        }
        graphics::present(ctx)?;

        timer::yield_now();
        Ok(())
    }
}
