use crate::blocks::block_kind;
use crate::game::*;

pub fn eval(game: &Game) -> Game {
    // elite block (Game, score)
    let mut elite = (game.clone(), 0f64);

    // all rotate
    for rotate_count in 0..=3 {
        let mut game = game.clone();
        for _ in 0..=rotate_count {
            // rotate process
            rotate_right(&mut game);
        }

        // all move sideway
        for dx in -4..=5 {
            let mut game = game.clone();
            // move process
            let new_pos = Position {
                x: match game.pos.x as isize + dx {
                    (..=0) => 0,
                    x => x as usize,
                },
                y: game.pos.y,
            };
            move_block(&mut game, new_pos);
            hard_drop(&mut game);
            fix_block(&mut game);

            // get input
            let line = erase_line_count(&game.field);
            let height_max = field_hight_max(&game.field);
            let height_diff = diff_in_height(&game.field);
            let dead_space = dead_space_count(&game.field);

            // normalization
            let mut line = normalization(line as f64, 0.0, 4.0);
            let mut height_max = 1.0 - normalization(height_max as f64, 0.0, 20.0);
            let mut height_diff = 1.0 - normalization(height_diff as f64, 0.0, 200.0);
            let mut dead_space = 1.0 - normalization(dead_space as f64, 0.0, 200.0);

            // weight
            line *= 100.0;
            height_max *= 1.0;
            height_diff *= 10.0;
            dead_space *= 100.0;

            // eval input
            let score = line + height_max + height_diff + dead_space;
            if elite.1 < score {
                // save best
                elite.0 = game;
                elite.1 = score;
            }
        }
    }
    elite.0
}

// get count of can erase line
#[allow(clippy::needless_range_loop)]
fn erase_line_count(field: &Field) -> usize {
    let mut count = 0;
    for y in 1..FIELD_HEIGHT - 2 {
        let mut can_erase = true;
        for x in 2..FIELD_WIDTH - 2 {
            if field[y][x] == block_kind::NONE {
                can_erase = false;
                break;
            }
        }
        if can_erase {
            count += 1;
        }
    }
    count
}

// get hight of the highest block in the field
#[allow(clippy::needless_range_loop)]
fn field_hight_max(field: &Field) -> usize {
    for y in 1..FIELD_HEIGHT - 2 {
        for x in 2..FIELD_WIDTH - 2 {
            if field[y][x] != block_kind::NONE {
                return FIELD_HEIGHT - y - 1;
            }
        }
    }
    unreachable!();
}

// normalization
fn normalization(value: f64, min: f64, max: f64) -> f64 {
    (value - min) / (max - min)
}

// get difference in field elevation
#[allow(clippy::needless_range_loop)]
pub fn diff_in_height(field: &Field) -> usize {
    let mut diff = 0;
    let mut top = [0; FIELD_WIDTH - 4];
    // find the height at the top of each column
    for x in 2..FIELD_WIDTH - 2 {
        for y in 1..FIELD_HEIGHT - 2 {
            if field[y][x] != block_kind::NONE {
                top[x - 2] = FIELD_HEIGHT - y - 1;
                break;
            }
        }
    }
    // sum side difference
    for i in 0..FIELD_WIDTH - 4 - 1 {
        diff += top[i].abs_diff(top[i + 1]);
    }
    diff
}

// get number of dead spaces
pub fn dead_space_count(field: &Field) -> usize {
    let mut count = 0;
    for y in (1..FIELD_HEIGHT - 2).rev() {
        for x in 2..FIELD_WIDTH - 2 {
            if field[y][x] == block_kind::NONE {
                for y2 in (2..y).rev() {
                    if field[y2][x] != block_kind::NONE {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }
    count
}
