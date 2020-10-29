use ggez::*;

struct State {
    delta_time: std::time::Duration,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.delta_time = ggez::timer::delta(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        println!(
            "Hello ggez! Delta time = {}ns",
            self.delta_time.subsec_nanos()
        );
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
    let state = &mut State {
        delta_time: std::time::Duration::new(0, 0),
    };

    let c = ggez::conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("ggez_tutorial", "Jazz")
        .conf(c)
        .build()
        .unwrap();

    match ggez::event::run(ctx, event_loop, state) {
        Err(e) => println!("Something strange happened: {:?}", e),
        Ok(_) => println!("Byebye!"),
    };
}
