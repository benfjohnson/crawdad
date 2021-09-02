mod engine;

fn main() {
    let mut game = engine::Game::new();
    // E2 -> E4, I think :)
    let did_move = game.try_move(engine::Color::White, (3, 1), (3, 3));
    println!("Hello, world! Did my move succeed? {}", did_move);
}
