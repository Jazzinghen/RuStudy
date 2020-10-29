use ggez::event;
use ggez::mint;
use ggez::ContextBuilder;

mod art_state;

fn main() {
    let mut shape_vec = Vec::new();
    shape_vec.push(art_state::Shape::Rectangle(ggez::graphics::Rect::new(
        10.0, 10.0, 50.0, 100.0,
    )));
    shape_vec.push(art_state::Shape::Circle(
        mint::Point2 { x: 400.0, y: 40.0 },
        30.0,
    ));
    // When doing it like this we are storing a constant reference to a mutable object
    // meaning that we _cannot_ change the memory location pointed by the variable, but
    // we can change the contents of the object pointed by it.
    let state = &mut art_state::Art_State { shapes: shape_vec };

    let context_builder = ContextBuilder::new("generative_art", "Jazzinghen");
    // Unwrap the value contained in the returned Enum... If correct?
    let (ref mut ctx, ref mut event_loop) = &mut context_builder.build().unwrap();

    match event::run(ctx, event_loop, state) {
        Ok(_) => println!("Game safely ended"),
        Err(e) => println!("Game returned an error: {:?}", e),
    };
}
