use crate::game;
use crate::game::{Direction, Player, Square, World};
use std::io;

#[derive(Debug)]
enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Expose,
    Score,
    Quit,
}

fn draw_world(world: &World, player: &Player) {
    for row in world.map.iter() {
        for square in row.iter() {
            draw_square(square, player);
        }
        println!("");
    }
}

fn draw_square(square: &Square, player: &Player) {
    print!("[");

    if is_player_on_square(player, square) {
        print!("o");
    } else {
        print!(" ");
    }

    if square.exposed {
        print!("x");
    } else {
        print!(" ");
    }

    print!("] ");
}

fn is_player_on_square(player: &Player, square: &Square) -> bool {
    player.x == square.x && player.y == square.y
}

fn print_score(player: &Player) {
    println!("Score: {}", player.score);
}

fn prompt() -> Action {
    // TODO: Move commands outside the match and this prompt
    loop {
        let mut input = String::new();
        println!("Enter command (u/d/l/r/e/s/q): ");
        io::stdin().read_line(&mut input).unwrap();
        let input: char = match input.trim().parse() {
            Ok(char) => char,
            // TODO: duplicated logic
            Err(_) => {
                println!("Invalid command.");
                continue;
            }
        };

        // TODO: Replace this with something more elegant, e.g. taking the key-action mapping out of this function
        let action = match input {
            'u' => Some(Action::MoveUp),
            'd' => Some(Action::MoveDown),
            'l' => Some(Action::MoveLeft),
            'r' => Some(Action::MoveRight),
            'e' => Some(Action::Expose),
            's' => Some(Action::Score),
            'q' => Some(Action::Quit),
            _ => None
        };

        match action {
            Some(action) => return action,
            _ => {
                println!("Invalid command.");
            }
        };
    }
}

fn handle_action(action: &Action, world: &mut World, player: &mut Player) {
    match action {
        Action::MoveUp => move_player(player, world, Direction::Up),
        Action::MoveDown => move_player(player, world, Direction::Down),
        Action::MoveLeft => move_player(player, world, Direction::Left),
        Action::MoveRight => move_player(player, world, Direction::Right),
        Action::Expose => expose(player, world),
        Action::Score => print_score(player),
        Action::Quit => quit(player)
    }
}

fn move_player(player: &mut Player, world: &mut World, direction: Direction) {
    if game::is_wall(world, player, &direction) {
        println!("There's a wall.");
    }

    game::move_player(world, player, &direction);
}

fn expose(player: &mut Player, world: &mut World) {
    let has_mine = game::expose_mine(player, world);

    if has_mine {
        println!("Boom!");
    } else {
        println!("Nice! There are {} surrounding mines.", game::num_mines(world, player));
    }

    print_score(&player);
}

fn quit(player: &mut Player) {
    // A bit of a lazy hack really
    println!("Goodbye.");
    player.alive = false
}

pub fn run_game() {
    let mut world = World::new(4, 4);
    let mut player = Player::new();
    game::generate_random_mines(&mut world, 8);

    while player.alive {
        draw_world(&world, &player);
        let action = prompt();
        handle_action(&action, &mut world, &mut player);
    }
}
