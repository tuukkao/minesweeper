use rand::prelude::*;

#[derive(Debug)]
pub struct Square {
    pub x: u8,
    pub y: u8,
    pub has_mine: bool,
    pub exposed: bool
}

impl Square {
    fn new(x: u8, y: u8) -> Square {
        Square {
            x,
            y,
            exposed: false,
            has_mine: false
        }
    }
}

#[derive(Debug)]
pub struct World {
    pub width: u8,
    pub height: u8,
    pub map: Vec<Vec<Square>>
}

impl World {
    pub fn new(width: u8, height: u8) -> World {
        let mut map: Vec<Vec<Square>> = Vec::new();

        for y in 0..height {
            let mut row: Vec<Square> = Vec::new();
            for x in 0..width {
                row.push(Square::new(x, y));
            }

            map.push(row);
        }

        World {
            width,
            height,
            map
        }
    }

    fn get_square(&self, x: u8, y: u8) -> &Square {
        // TODO: result checking for out of bound coordinates. Naively assuming that those won't happen for now.
        &self.map[usize::from(y)][usize::from(x)]
    }

    fn get_square_mut(&mut self, x: u8, y: u8) -> &mut Square {
        // TODO: result checking for out of bound coordinates. Naively assuming that those won't happen for now.
        &mut self.map[usize::from(y)][usize::from(x)]
    }
}

#[derive(Debug)]
pub struct Player {
    pub x: u8,
    pub y: u8,
    pub score: u8,
    pub alive: bool
}

impl Player {
    pub fn new() -> Player {
        Player {
            x: 0,
            y: 0,
            score: 0,
            alive: true
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub fn move_player(world: &World, player: &mut Player, direction: &Direction) {
    match direction {
        Direction::Up => if player.y > 0 { player.y -= 1 },
        Direction::Down => if player.y < world.height - 1{ player.y += 1 },
        Direction::Left => if player.x > 0 { player.x -= 1 },
        Direction::Right => if player.x < world.width - 1 { player.x += 1 }
    };
}

pub fn expose_mine(player: &mut Player, world: &mut World) -> bool {
    let square = world.get_square_mut(player.x, player.y);

    if square.exposed {
        return square.has_mine;
    }

    square.exposed = true;

    if square.has_mine {
        player.alive = false;
    } else {
        player.score += 1;
    }

    square.has_mine
}

pub fn generate_random_mines(world: &mut World, mines: u8) {
    let mut rng = rand::thread_rng();

    for _ in 0..mines {
        let (x, y) = random_square_index(&mut rng, world);
        let square = world.get_square_mut(x, y);
        square.has_mine = true;
    }
}

fn random_square_index(rng: &mut rand::rngs::ThreadRng, world: &World) -> (u8, u8) {
    loop {
        let x: u8 = rng.gen_range(0, world.width - 1);
        let y: u8 = rng.gen_range(0, world.height - 1);

        if !is_corner(world, x, y) {
            return (x, y);
        }
    }
}

fn is_corner(world: &World, x: u8, y: u8) -> bool {
    x == 0 || x == world.width - 1
        && y == 0 || y == world.height - 1
}

pub fn is_wall(world: &World, player: &Player, direction: &Direction) -> bool {
    match direction {
        Direction::Up => player.y == 0,
        Direction::Down => player.y == world.height - 1,
        Direction::Left => player.x == 0,
        Direction::Right => player.x == world.width - 1
    }
}

pub fn num_mines(world: &World, player: &Player) -> u8 {
    let mut mines = 0;

    //TODO: There's probably a nicer way to do this
    // left side
    if player.x > 0 && world.get_square(player.x - 1, player.y).has_mine {
        mines += 1;
    }
    // right side
    // - 2 because world.width is 1 greater than the right-most index
    else if player.x < world.width - 2 && world.get_square(player.x + 1, player.y).has_mine {
        mines += 1;
    }
    // above
    else if player.y > 0 && world.get_square(player.x, player.y - 1).has_mine {
        mines += 1;
    }
    // below
    else if player.y < world.height - 2 && world.get_square(player.x, player.y + 1).has_mine {
        mines += 1;
    }

    mines
}
