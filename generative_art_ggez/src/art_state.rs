use ggez::event;
use ggez::graphics;
use ggez::mint;
use ggez::Context;
use ggez::GameResult;

pub struct Art_State {
    pub shapes: Vec<Shape>,
}

pub enum Shape {
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}

impl event::EventHandler for Art_State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
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
        Ok(())
    }
}
