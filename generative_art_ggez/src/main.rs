use ggez::*;

struct Art_State {}

impl ggez::event::EventHandler for Art_State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    // When doing it like this we are storing a constant reference to a mutable object
    // meaning that we _cannot_ change the memory location pointed by the variable, but
    // we can change the contents of the object pointed by it.
    let state = &mut Art_State {};

    let context_builder = ggez::ContextBuilder::new("generative_art", "Jazzinghen");
    // Unwrap the value contained in the returned Enum... If correct?
    let (ref mut ctx, ref mut event_loop) = &mut context_builder.build().unwrap();

    match ggez::event::run(ctx, event_loop, state) {
        Ok(_) => println!("Game safely ended"),
        Err(e) => println!("Game returned an error: {:?}", e),
    };
}
