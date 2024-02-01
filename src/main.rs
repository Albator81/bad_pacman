/////////////////////////////////////////////////////////////////////////////////
//                                   PACMAN                                    //
//   Ghosts: Blinky (red), Pinky (pink), Inky (cyan), and Clyde (orange)       //
//   top left is (0, 0)                                                        //
//   the behavior of the ghosts aren't like the original game because I was    //
//   too lazy to make everything identical and perfect.                        //
/////////////////////////////////////////////////////////////////////////////////


//////////////////////////////IMPORTING INTO SCOPE///////////////////////////////
use std::io::stdout;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, SetTitle};
use crossterm::cursor::{MoveTo, Hide};
use crossterm::style::{style, Stylize, Color};
use crossterm::event::{read, poll, Event, KeyCode, KeyEventKind};
use std::time::{Duration, Instant, UNIX_EPOCH, SystemTime};
use std::thread::sleep;

//////////////////////////////IMPORTING INTO SCOPE///////////////////////////////

////////////////////////////////////CONSTANTS////////////////////////////////////

const WIDTH:     usize = 28;
const HEIGHT:    usize = 36;
const MAPWIDTH:  usize = 28;
const MAPHEIGHT: usize = 31;
const EMPTY: i8 = 0; // empty space (no wall)
const WALLH: i8 = 1; // horizontal wall
const WALLV: i8 = 2; // vertical   wall
// const WALLI: i8 = 3; // the inside of the wall
const WALLS: [[i8; MAPWIDTH]; MAPHEIGHT] = [
    [WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH],
    [WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV],
    [WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLH],
    [WALLV, EMPTY, WALLV, WALLV, WALLH, WALLH, EMPTY, WALLV, WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV, WALLV, EMPTY, WALLH, WALLH, WALLV, WALLV, EMPTY, WALLV],
    [WALLV, EMPTY, WALLV, WALLV, EMPTY, EMPTY, EMPTY, WALLV, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, WALLV, EMPTY, EMPTY, EMPTY, WALLV, WALLV, EMPTY, WALLV],
    [WALLV, EMPTY, WALLV, WALLV, EMPTY, WALLH, WALLH, WALLV, WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV, WALLV, WALLH, WALLH, EMPTY, WALLV, WALLV, EMPTY, WALLV],
    [WALLV, EMPTY, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, WALLV, WALLV, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, EMPTY, WALLV],
    [WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV],
    [WALLV, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV, WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, WALLV],
    [WALLV, WALLV, WALLV, WALLV, EMPTY, WALLV, WALLV, EMPTY, WALLV, WALLV, WALLV, WALLV, EMPTY, WALLV, WALLV, EMPTY, WALLV, WALLV, WALLV, WALLV, EMPTY, WALLV, WALLV, EMPTY, WALLV, WALLV, WALLV, WALLV],
    [WALLV, WALLH, WALLH, WALLH, EMPTY, WALLV, WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV, WALLV, EMPTY, WALLH, WALLH, WALLH, WALLV],
    [WALLV, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, WALLV],
    [WALLV, EMPTY, WALLH, WALLH, WALLH, WALLV, WALLV, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, EMPTY, EMPTY, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLV, WALLV, WALLH, WALLH, WALLH, EMPTY, WALLV],
    [WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV],
    [WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV],
    [WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV],
    [WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, WALLH, WALLV, WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV, WALLV, WALLH, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV],
    [WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV],
    [WALLV, WALLH, WALLH, EMPTY, WALLH, WALLH, EMPTY, WALLV, WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV, WALLV, EMPTY, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLV],
    [WALLH, WALLH, WALLH, EMPTY, WALLV, WALLV, EMPTY, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, WALLV, WALLV, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, EMPTY, WALLV, WALLV, EMPTY, WALLH, WALLH, WALLH],
    [EMPTY, EMPTY, EMPTY, EMPTY, WALLV, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, WALLV, EMPTY, EMPTY, EMPTY, EMPTY],
    [WALLH, WALLH, WALLH, EMPTY, WALLV, WALLV, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, EMPTY, WALLV, WALLV, EMPTY, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, WALLV, WALLV, EMPTY, WALLH, WALLH, WALLH],
    [WALLV, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV, WALLV, EMPTY, WALLH, WALLH, EMPTY, WALLV, WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLV],
    [WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV],
    [WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, EMPTY, WALLV, WALLV, WALLH, WALLH, WALLH, WALLH, WALLV, WALLV, EMPTY, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV],
    [WALLV, EMPTY, WALLV, WALLV, WALLV, WALLV, EMPTY, WALLV, WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV, WALLV, EMPTY, WALLV, WALLV, WALLV, WALLV, EMPTY, WALLV],
    [WALLV, EMPTY, WALLV, WALLV, WALLV, WALLV, EMPTY, WALLV, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, WALLV, EMPTY, WALLV, WALLV, WALLV, WALLV, EMPTY, WALLV],
    [WALLV, EMPTY, WALLV, WALLV, WALLV, WALLV, EMPTY, WALLV, WALLV, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, WALLV, WALLV, EMPTY, WALLV, WALLV, WALLV, WALLV, EMPTY, WALLV],
    [WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV, WALLV, EMPTY, WALLH, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLH, WALLH, WALLH, WALLH, EMPTY, WALLV],
    [WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV, WALLV, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALLV],
    [WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH, WALLH]
];
const BLINKY_HOME_TILE: Pos = Pos { x: 25, y: 0 }; // in the wall
const PINKY_HOME_TILE:  Pos = Pos { x: 2 , y: 0 }; // in the wall
const INKY_HOME_TILE:   Pos = Pos { x: 27, y: 37 }; // out of bounds
const CLYDE_HOME_TILE:  Pos = Pos { x: 0 , y: 37 }; // out of bounds

const TARGET_FPS:     u32 = 50;

////////////////////////////////////CONSTANTS////////////////////////////////////

//////////////////////////////////////ENUMS//////////////////////////////////////
#[derive(Clone, Copy)]
enum Mode { Chase, Scatter, Frightened }

// note to self: Not handling NoDirection WILL cause the game to break.
// You can also decide to not do anything about it...
#[derive(Clone, Copy)]
enum Direction { Up, Down, Left, Right, NoDirection }

//////////////////////////////////////ENUMS//////////////////////////////////////

/////////////////////////////////////STRUCTS//////////////////////////////////////
#[derive(Clone, Copy)]
struct Pos { x: usize, y: usize, }

/// contains both the map and the moving things like the player and the ghosts
struct World { pacman: Pacman, blinky: Blinky, pinky: Pinky, inky: Inky, clyde: Clyde, }
// player
struct Pacman { tile: Pos, previous_tile: Pos, rel_pos: i8, direction: Direction, is_energized: bool, }
// ghosts
struct Blinky { tile: Pos, previous_tile: Pos, rel_pos: i8, mode: Mode, target_tile: Pos, direction: Direction, }
struct Pinky  { tile: Pos, previous_tile: Pos, rel_pos: i8, mode: Mode, target_tile: Pos, direction: Direction, }
struct Inky   { tile: Pos, previous_tile: Pos, rel_pos: i8, mode: Mode, target_tile: Pos, direction: Direction, }
struct Clyde  { tile: Pos, previous_tile: Pos, rel_pos: i8, mode: Mode, target_tile: Pos, direction: Direction, }

// obtainables
struct Food      { tile: Pos, is_shown: bool, is_activated: bool, }
struct Energizer { tile: Pos, is_shown: bool, is_activated: bool, }
struct Fruit     { tile: Pos, is_shown: bool, is_activated: bool, }

struct Game { world: World }

/////////////////////////////////////STRUCTS//////////////////////////////////////

////////////////////////////////////FUNCTIONS/////////////////////////////////////

fn print_map() {
    for row in WALLS {
        println!("{}", style(row.iter()
        .map(|&wall| if wall == EMPTY { "  " }  else { "██" })
        .collect::<String>())
        .with(Color::Rgb { r: 10, g: 40, b: 200 })
        .on  (Color::Black)
        );
    }
}

fn collides(tile: Pos) -> bool {
    tile.x >= WIDTH || tile.y >= HEIGHT || WALLS[tile.y][tile.x] != EMPTY
    // works because || ignores the second condition if the first one is true
}

fn rev_direction(direction: Direction) -> Direction {
    return match direction {
        Direction::Up    => Direction::Down,
        Direction::Down  => Direction::Up,
        Direction::Right => Direction::Left,
        Direction::Left  => Direction::Right,
        Direction::NoDirection => Direction::NoDirection,
    }
}
fn adjacent_directions(direction: Direction) -> (Direction, Direction) {
    return match direction {
        Direction::Up    => (Direction::Right, Direction::Left),
        Direction::Down  => (Direction::Left, Direction::Right),
        Direction::Right => (Direction::Down, Direction::Up),
        Direction::Left  => (Direction::Up, Direction::Down),
        Direction::NoDirection => (Direction::NoDirection, Direction::NoDirection),
    }
}
fn random_bool() -> bool {
    if let Ok(clock) = UNIX_EPOCH.elapsed() {
        return clock.as_secs() % 2 == 0;
    } else {
        eprintln!("SystemTimeError");
        return true;
    }
}

fn left_tile       (tile: Pos) -> Pos { return Pos { x: tile.x - 1, y: tile.y     } }
fn right_tile      (tile: Pos) -> Pos { return Pos { x: tile.x + 1, y: tile.y     } }
fn top_tile        (tile: Pos) -> Pos { return Pos { x: tile.x    , y: tile.y - 1 } }
fn bottom_tile     (tile: Pos) -> Pos { return Pos { x: tile.x    , y: tile.y + 1 } }

fn taxicab_distance(tile: Pos, target_tile: Pos) -> usize{
    return tile.x.abs_diff(target_tile.x) + tile.y.abs_diff(target_tile.y)
}
fn tile_nearer_than(tile1: Pos, tile2: Pos, target_tile: Pos) -> bool {
    return taxicab_distance(tile1, target_tile) < taxicab_distance(tile2, target_tile);
}
fn to_next_tile    (tile: Pos, direction: Direction) -> Pos {
    return match direction {
        Direction::Up   if tile.y != 0 => top_tile   (tile),
        Direction::Down                => bottom_tile(tile),
        Direction::Right               => right_tile (tile),
        Direction::Left if tile.x != 0 => left_tile  (tile),
        _ => Pos { x: 0, y: 0 },

    }
}
fn pinky_target_tile(pacman_tile: Pos, pacman_direction: Direction) -> Pos {
    return match pacman_direction {
        Direction::Up    => Pos { x: pacman_tile.x - 4, y: pacman_tile.y - 4 },
        Direction::Down  => Pos { x: pacman_tile.x    , y: pacman_tile.y + 4 },
        Direction::Right => Pos { x: pacman_tile.x + 4, y: pacman_tile.y     },
        Direction::Left  => Pos { x: pacman_tile.x - 4, y: pacman_tile.y     },
        Direction::NoDirection => pacman_tile,
    }
}
fn inky_target_tile(pacman_tile: Pos, pacman_direction: Direction, blinky_tile: Pos) -> Pos {
    let pacman_tile_plus2: Pos = match pacman_direction {
        Direction::Up    => Pos { x: pacman_tile.x    , y: pacman_tile.y - 2 },
        Direction::Down  => Pos { x: pacman_tile.x    , y: pacman_tile.y + 2 },
        Direction::Right => Pos { x: pacman_tile.x + 2, y: pacman_tile.y     },
        Direction::Left  => Pos { x: pacman_tile.x - 2, y: pacman_tile.y     },
        Direction::NoDirection => pacman_tile,
    };
    return Pos { x: pacman_tile_plus2.x.abs_diff(blinky_tile.x) * 2, y: pacman_tile_plus2.x.abs_diff(blinky_tile.x) * 2 };
}
fn clyde_target_tile(clyde_tile: Pos, pacman_tile: Pos) -> Pos {
    // I do know that this is completely wrong IF I wanted to make clyde's original behavior
    if taxicab_distance(clyde_tile, pacman_tile) < 10 {
        return pacman_tile;
    } else {
        return CLYDE_HOME_TILE;
    }
}
fn to_direction(tile: Pos, next_tile: Pos) -> Direction { // worst function of all time
    if tile.x == next_tile.x {
        if tile.y == next_tile.y {
            return Direction::NoDirection;
        }
        else if tile.y < next_tile.y {
            return Direction::Down;
        }
        else {
            return Direction::Up;
        }
    } else if tile.x < next_tile.x {
        return Direction::Right;
    } else {
        return Direction::Left;
    }
}
fn nearer_tile(tile1: Pos, tile2: Pos, target_tile: Pos) -> Pos {
    if tile_nearer_than(tile1, tile2, target_tile) {
        return tile1;
    }
    return tile2;
}

fn next_best_direction(tile: Pos, direction1: Direction, target_tile: Pos) -> Direction {
    let (direction2, direction3) = adjacent_directions(direction1);

    let tile1: Pos = to_next_tile(tile, direction1);
    let tile2: Pos = to_next_tile(tile, direction2);
    let tile3: Pos = to_next_tile(tile, direction3);

    match (
        collides(tile1), 
        collides(tile2), 
        collides(tile3)
    ) {
        (false, true , true ) => direction1,
        (true , false, true ) => direction2,
        (true , true , false) => direction3,
        (false, false, true ) => to_direction(tile, nearer_tile(tile1, tile2, target_tile)),
        (false, true , false) => to_direction(tile, nearer_tile(tile1, tile3, target_tile)),
        (true , false, false) => to_direction(tile, nearer_tile(tile2, tile3, target_tile)),
        (false, false, false) => to_direction(tile, nearer_tile(nearer_tile(tile1, tile2, target_tile), tile3, target_tile)),
        (true , true , true ) => rev_direction(direction1), // shouldn't happen tho
    }
}
fn next_random_direction(tile: Pos, direction1: Direction, target_tile: Pos) -> Direction {
    let (direction2, direction3) = adjacent_directions(direction1);

    fn random_direction3(direction1: Direction, direction2: Direction, direction3: Direction) -> Direction{
        return match (random_bool(), random_bool()) {
            (false, false) => direction1, // Event 1
            (false, true)  => direction2,  // Event 2
            (true, false)  => direction3,  // Event 3
            _ => random_direction3(direction1, direction2, direction3), // Retry if both are true to ensure equal probabilities
        }
    }
    fn random_direction2(direction1: Direction, direction2: Direction) -> Direction{
        return if random_bool() { direction1 } else { direction2 }
    }

    let tile1: Pos = to_next_tile(tile, direction1);
    let tile2: Pos = to_next_tile(tile, direction2);
    let tile3: Pos = to_next_tile(tile, direction3);

    return match (
        collides(tile1), 
        collides(tile2), 
        collides(tile3)
    ) {
        (false, true , true ) => direction1,
        (true , false, true ) => direction2,
        (true , true , false) => direction3,
        (false, false, true ) => random_direction2(direction1, direction2),
        (false, true , false) => random_direction2(direction1, direction3),
        (true , false, false) => random_direction2(direction2, direction3),
        (false, false, false) => random_direction3(direction1, direction2, direction3),
        (true , true , true ) => rev_direction(direction1), // shouldn't happen tho
    }
}

////////////////////////////////////FUNCTIONS/////////////////////////////////////

/////////////////////////////////////TRAITS///////////////////////////////////////

trait Movable {
    fn get_pos (&self) -> Pos;
    fn set_pos (&mut self, new_x: usize, new_y: usize);
}

trait Ghost: Movable {
    fn chase_move      (&mut self);
    fn scatter_move    (&mut self);
    fn frightened_move (&mut self);
    /// should only be called when direction needs to be updated, when the ghost is in the middle of the tile (relatively?)
    // fn update_direction(&mut self, tile_used: Pos, direction_used: Direction);
    fn update_pos      (&mut self);
    /// the position in the terminal
    fn update_rel_pos  (&mut self);
    // fn update          (&mut self, tile_used: Pos, direction_used: Direction);
}

trait Obtainable: Movable {
    fn activate   (&mut self);
    fn deactivate (&mut self);
    fn show       (&mut self);
    fn hide       (&mut self);
    fn do_effect  (&mut self);
}

/////////////////////////////////////TRAITS///////////////////////////////////////

////////////////////////////////IMPLEMENTATIONS///////////////////////////////////

impl Movable for Pacman {
    fn get_pos (&self) -> Pos { return self.tile }
    fn set_pos (&mut self, new_x: usize, new_y: usize) {
        self.tile.x = new_x;
        self.tile.y = new_y;
    }
}
impl Pacman {
    fn new( tile: Pos) -> Self {
        Self { tile, previous_tile: tile, rel_pos: 0, direction: Direction::NoDirection, is_energized: false }
    }
    fn update_direction(&mut self, c: char) {
        match c {
            'z' => self.direction = Direction::Up,
            's' => self.direction = Direction::Down,
            'd' => self.direction = Direction::Right,
            'q' => self.direction = Direction::Left,
            _   => (),
        }
    }
    fn update_pos(&mut self) {
        self.previous_tile = self.tile;
        let new_tile: Pos = to_next_tile(self.tile, self.direction);
        if collides(new_tile) { return; }
        self.tile = new_tile;
    }
    fn draw(&self) {
        execute!(stdout(), MoveTo(self.previous_tile.x as u16 * 2, self.previous_tile.y as u16));
        print!(" ");

        execute!(stdout(), MoveTo(self.tile.x as u16 * 2, self.tile.y as u16));
        print!("{}", style("O").with(Color::Yellow));
    }
    fn update(&mut self, c: char) {
        self.update_direction(c);
        self.update_pos();
    }
}

impl Movable for Blinky {
    fn get_pos    (&self) -> Pos { return self.tile }
    fn set_pos (&mut self, new_x: usize, new_y: usize) {
        self.tile.x = new_x;
        self.tile.y = new_y;
    }
}
impl Movable for Pinky {
    fn get_pos    (&self) -> Pos { return self.tile }
    fn set_pos (&mut self, new_x: usize, new_y: usize) {
        self.tile.x = new_x;
        self.tile.y = new_y;
    }
}
impl Movable for Inky {
    fn get_pos    (&self) -> Pos { return self.tile }
    fn set_pos (&mut self, new_x: usize, new_y: usize) {
        self.tile.x = new_x;
        self.tile.y = new_y;
    }
}
impl Movable for Clyde {
    fn get_pos    (&self) -> Pos { return self.tile }
    fn set_pos (&mut self, new_x: usize, new_y: usize) {
        self.tile.x = new_x;
        self.tile.y = new_y;
    }
}

impl Ghost for Blinky {
    fn chase_move     (&mut self) {
        self.direction = next_best_direction(self.tile, self.direction, self.target_tile);
    }
    fn frightened_move(&mut self) {
        self.direction = next_random_direction(self.tile, self.direction, self.target_tile);
    }
    fn scatter_move   (&mut self) {
        self.direction = next_best_direction(self.tile, self.direction, self.target_tile)
    }

    fn update_pos     (&mut self) {
        self.previous_tile = self.tile;
        match self.direction {
            Direction::Up    => { self.tile.y -= 1; },
            Direction::Down  => { self.tile.y += 1; },
            Direction::Right => { self.tile.x += 1; },
            Direction::Left  => { self.tile.x -= 1; },
            Direction::NoDirection => (),
        }
    }
    fn update_rel_pos (&mut self) {}
}
impl Ghost for Pinky {
    fn chase_move     (&mut self) {
        self.direction = next_best_direction(self.tile, self.direction, self.target_tile);
    }
    fn frightened_move(&mut self) {
        self.direction = next_random_direction(self.tile, self.direction, self.target_tile);
    }
    fn scatter_move   (&mut self) {
        self.direction = next_best_direction(self.tile, self.direction, self.target_tile);
    }

    fn update_pos     (&mut self) {
        self.previous_tile = self.tile;
        match self.direction {
            Direction::Up    => { self.tile.y -= 1; },
            Direction::Down  => { self.tile.y += 1; },
            Direction::Right => { self.tile.x += 1; },
            Direction::Left  => { self.tile.x -= 1; },
            Direction::NoDirection => (),
        }
    }
    fn update_rel_pos (&mut self) {}
}
impl Ghost for Inky {
    fn chase_move     (&mut self) {
        self.direction = next_best_direction(self.tile, self.direction, self.target_tile);
    }
    fn frightened_move(&mut self) {
        self.direction = next_random_direction(self.tile, self.direction, self.target_tile);
    }
    fn scatter_move   (&mut self) {
        self.direction = next_best_direction(self.tile, self.direction, self.target_tile);
    }

    fn update_pos     (&mut self) {
        self.previous_tile = self.tile;
        match self.direction {
            Direction::Up    => { self.tile.y -= 1; },
            Direction::Down  => { self.tile.y += 1; },
            Direction::Right => { self.tile.x += 1; },
            Direction::Left  => { self.tile.x -= 1; },
            Direction::NoDirection => (),
        }
    }
    fn update_rel_pos (&mut self) {}
}
impl Ghost for Clyde {
    fn chase_move     (&mut self) {
        self.direction = next_best_direction(self.tile, self.direction, self.target_tile);
    }
    fn frightened_move(&mut self) {
        self.direction = next_random_direction(self.tile, self.direction, self.target_tile);
    }
    fn scatter_move   (&mut self) {
        self.direction = next_best_direction(self.tile, self.direction, self.target_tile);
    }

    fn update_pos     (&mut self) {
        self.previous_tile = self.tile;
        match self.direction {
            Direction::Up    => { self.tile.y -= 1; },
            Direction::Down  => { self.tile.y += 1; },
            Direction::Right => { self.tile.x += 1; },
            Direction::Left  => { self.tile.x -= 1; },
            Direction::NoDirection => (),
        }
    }
    fn update_rel_pos (&mut self) {}
}

impl Blinky {
    fn new(tile: Pos) -> Self {
        Self {
            tile, 
            previous_tile: tile, 
            rel_pos: 0, 
            mode: Mode::Scatter, 
            target_tile: Pos{x: 0, y: 0}, 
            direction: Direction::Right
        }
    }
    fn update_direction(&mut self, pacman_tile: Pos) {
        match self.mode {
            Mode::Chase      => { self.target_tile = pacman_tile;      self.chase_move(); },
            Mode::Scatter    => { self.target_tile = BLINKY_HOME_TILE; self.scatter_move(); },
            Mode::Frightened => self.frightened_move(),
        }
    }
    fn draw(&self) {
        execute!(stdout(), MoveTo(self.previous_tile.x as u16 * 2, self.previous_tile.y as u16));
        print!(" ");

        execute!(stdout(), MoveTo(self.tile.x as u16 * 2, self.tile.y as u16 ));
        print!("{}", style("X").with(Color::Rgb { r: 200, g: 0, b: 20 }));
    }
    fn update          (&mut self, pacman_tile: Pos) {
        self.update_direction(pacman_tile);
        self.update_pos      ();
        self.update_rel_pos  ();
    }
}
impl Pinky {
    fn new(tile: Pos) -> Self {
        Self {
            tile, 
            previous_tile: tile, 
            rel_pos: 0, 
            mode: Mode::Scatter, 
            target_tile: Pos{x: 0, y: 0}, 
            direction: Direction::Right
        }
    }
    fn update_direction(&mut self, pacman_tile: Pos, pacman_direction: Direction) {
        match self.mode {
            Mode::Chase      => { self.target_tile = pinky_target_tile(pacman_tile, pacman_direction); self.chase_move(); },
            Mode::Scatter    => { self.target_tile = PINKY_HOME_TILE; self.scatter_move(); },
            Mode::Frightened => self.frightened_move(),
        }
    }
    fn draw(&self) {
        execute!(stdout(), MoveTo(self.previous_tile.x as u16 * 2, self.previous_tile.y as u16));
        print!(" ");

        execute!(stdout(), MoveTo(self.tile.x as u16 * 2, self.tile.y as u16 ));
        print!("{}", style("X").with(Color::Rgb { r: 252, g: 73, b: 217 }));
    }
    fn update          (&mut self, pacman_tile: Pos, pacman_direction: Direction) {
        self.update_direction(pacman_tile, pacman_direction);
        self.update_pos      ();
        self.update_rel_pos  ();
    }
}
impl Inky {
    fn new(tile: Pos) -> Self {
        Self {
            tile,  
            previous_tile: tile, 
            rel_pos: 0, 
            mode: Mode::Scatter, 
            target_tile: Pos{x: 0, y: 0}, 
            direction: Direction::Right
        }
    }
    fn update_direction(&mut self, pacman_tile: Pos, pacman_direction: Direction, blinky_tile: Pos) {
        match self.mode {
            Mode::Chase      => { self.target_tile = inky_target_tile(pacman_tile, pacman_direction, blinky_tile); self.chase_move(); },
            Mode::Scatter    => { self.target_tile = INKY_HOME_TILE; self.scatter_move(); },
            Mode::Frightened => self.frightened_move(),
        }
    }
    fn draw(&self) {
        execute!(stdout(), MoveTo(self.previous_tile.x as u16 * 2, self.previous_tile.y as u16));
        print!(" ");

        execute!(stdout(), MoveTo(self.tile.x as u16 * 2, self.tile.y as u16 ));
        print!("{}", style("X").with(Color::Rgb { r: 3, g: 234, b: 255 }));
    }
    fn update          (&mut self, pacman_tile: Pos, pacman_direction: Direction, blinky_tile: Pos) {
        self.update_direction(pacman_tile, pacman_direction, blinky_tile);
        self.update_pos      ();
        self.update_rel_pos  ();
    }
}
impl Clyde {
    fn new(tile: Pos) -> Self {
        Self {
            tile, 
            previous_tile: tile, 
            rel_pos: 0, 
            mode: Mode::Scatter, 
            target_tile: Pos{x: 0, y: 0}, 
            direction: Direction::Right
        }
    }
    fn update_direction(&mut self, pacman_tile: Pos) {
        match self.mode {
            Mode::Chase      => { self.target_tile = clyde_target_tile(self.tile, pacman_tile); self.chase_move(); },
            Mode::Scatter    => { self.target_tile = CLYDE_HOME_TILE; self.scatter_move(); },
            Mode::Frightened => self.frightened_move(),
        }
    }
    fn draw(&self) {
        execute!(stdout(), MoveTo(self.previous_tile.x as u16 * 2, self.previous_tile.y as u16));
        print!(" ");

        execute!(stdout(), MoveTo(self.tile.x as u16 * 2, self.tile.y as u16 ));
        print!("{}", style("X").with(Color::Rgb { r: 255, g: 120, b: 3 }));
    }
    fn update          (&mut self, pacman_tile: Pos) {
        self.update_direction(pacman_tile);
        self.update_pos      ();
        self.update_rel_pos  ();
    }
}


impl World {
    fn new(pacman: Pacman, blinky: Blinky, pinky: Pinky, inky: Inky, clyde: Clyde) -> Self {
        Self { pacman, blinky, pinky, inky, clyde }
    }
    fn draw(&self) {
        self.pacman.draw();
        self.blinky.draw();
        self.pinky .draw();
        self.inky  .draw();
        self.clyde .draw();
    }
    fn update_world(&mut self, c: char) {
        self.pacman.update(c);
        self.blinky.update(self.pacman.tile);
        self.pinky .update(self.pacman.tile, self.pacman.direction);
        self.inky  .update(self.pacman.tile, self.pacman.direction, self.blinky.tile);
        self.clyde .update(self.pacman.tile);
    }
}

impl Game {
    fn new() -> Self {
        execute!(stdout(), 
            SetTitle("Pacman terminal ?"),
            Hide
        );
        Self {
            world: World::new(
                Pacman::new(Pos { x: 1, y: 1 }),
                Blinky::new(Pos { x: 2, y: 1 }),
                Pinky ::new(Pos { x: 2, y: 1 }),
                Inky  ::new(Pos { x: 2, y: 1 }),
                Clyde ::new(Pos { x: 2, y: 1 }),
            )
        }
    }
    fn run(&mut self) {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0));
        print_map();

        let mut c: char; // ! is default value

        let frame_interval = Duration::new(0, 1000000000u32/TARGET_FPS);
        let mut frame_start: Instant;
        let mut frame_delta: Duration;

        loop {
            frame_start = Instant::now();

            c = '!';
            if let Ok(true) = poll(Duration::ZERO) {
                if let Ok(Event::Key(key_event)) = read(){ // `read` won't block because an event is available
                    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Esc => break, // Exit the game loop
                            KeyCode::Char(character) => c = character,
                            _ => (),
                        }
                    }
                }
            }

            self.world.update_world(c);
            self.world.draw();

            frame_delta = frame_start.elapsed();
            if frame_delta < frame_interval { sleep(frame_interval - frame_delta) };
        }
    }
}

////////////////////////////////IMPLEMENTATIONS///////////////////////////////////

fn main() {
    let mut game : Game = Game::new();
    game.run();
}
