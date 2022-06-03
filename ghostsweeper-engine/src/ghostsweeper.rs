use crate::random::range;
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Display, Write};

type Position = (usize, usize);

#[derive(Debug)]
pub enum OpenResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Ghostsweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    mines: HashSet<Position>,
    pub is_over: bool,
    pub is_winner: bool,
}

impl Display for Ghostsweeper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.width {
            if i == 0 {
                write!(f, "   \t")?;
            }

            write!(f, "{}  ", i + 1)?;
        }

        f.write_char('\n')?;
        f.write_char('\n')?;
        f.write_char('\n')?;

        for y in 0..self.height {
            write!(f, "{} \t", y + 1)?;

            for x in 0..self.width {
                let position = (x, y);
                if !self.open_fields.contains(&position) {
                    if self.flagged_fields.contains(&position) {
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("🟦 ")?;
                    }
                } else if self.mines.contains(&position) {
                    f.write_str("👻 ")?;
                } else {
                    write!(f, "{}  ", self.get_neighbor_mines(position).len())?;
                }
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Ghostsweeper {
    pub fn new(width: usize, height: usize, num_of_mines: usize) -> Ghostsweeper {
        let mut mines = HashSet::new();
        while mines.len() < num_of_mines {
            mines.insert((range(0, width), range(0, height)));
        }

        Ghostsweeper {
            width,
            height,
            mines,
            open_fields: HashSet::new(),
            flagged_fields: HashSet::new(),
            is_over: false,
            is_winner: false,
        }
    }

    pub fn open(&mut self, position: Position) -> Option<OpenResult> {
        if self.is_over {
            return None;
        }

        if self.flagged_fields.contains(&position) {
            return None;
        }

        if self.open_fields.contains(&position) {
            return None;
        }

        self.open_fields.insert(position);

        if self.mines.contains(&position) {
            self.is_over = true;
            return Some(OpenResult::Mine);
        }

        let count = self.get_neighbor_mines(position).len() as u8;
        if count == 0 {
            let neighbors = self.get_neighbors(position);
            for n in neighbors {
                self.open(n);
            }
        }

        Some(OpenResult::NoMine(count))
    }

    pub fn toggle_flag(&mut self, position: Position) {
        if self.is_over {
            return;
        }
        if self.open_fields.contains(&position) {
            return;
        }

        if self.flagged_fields.contains(&position) {
            self.flagged_fields.remove(&position);
        } else {
            self.flagged_fields.insert(position);
        }
    }

    pub fn check_state(&mut self) {
        let width = self.width as i32;
        let height = self.height as i32;
        let mine_count = self.mines.len() as i32;
        let open_count = self.open_fields.len() as i32;

        let max_open_count = (width * height) - mine_count;
        // println!("max: {}", max_open_count);
        // println!("open: {}", open_count);

        if open_count == max_open_count {
            self.is_over = true;
            self.is_winner = true;
        }
    }

    fn get_neighbor_mines(&self, position: Position) -> Vec<Position> {
        self.get_neighbors(position)
            .into_iter()
            .filter(|n| self.mines.contains(n))
            .collect()
    }

    fn get_neighbors(&self, position: Position) -> Vec<Position> {
        let width = self.width as isize;
        let height = self.height as isize;
        let (x, y) = position;

        let deltas: Vec<(isize, isize)> = vec![
            (-1, -1), // Top Left
            (0, -1),  // Top
            (1, -1),  // Top Right
            (-1, 0),  // Left
            (1, 0),   // Right
            (-1, 1),  // Bottom Left
            (0, 1),   // Bottom
            (1, 1),   // Bottom Right
        ];

        let mut neighbors = Vec::new();

        for (dx, dy) in deltas {
            let xt = (x as isize) + dx;
            let yt = (y as isize) + dy;

            if xt >= 0 && xt < width && yt >= 0 && yt < height {
                neighbors.push((xt as usize, yt as usize));
            }
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::Ghostsweeper;

    #[test]
    fn new_game() {
        let game = Ghostsweeper::new(10, 10, 8);
        assert_eq!(game.mines.len(), 8);
    }

    #[test]
    fn open() {
        let mut game = Ghostsweeper::new(10, 10, 8);
        game.open((5, 5));
        assert_eq!(game.open_fields.len(), 1);
    }

    #[test]
    fn toggle_flag() {
        let mut game = Ghostsweeper::new(10, 10, 8);
        game.open((5, 5));
        game.toggle_flag((5, 5));
        game.toggle_flag((1, 1));
        game.toggle_flag((5, 3));
        game.toggle_flag((6, 3));

        println!("{}", game);

        assert_eq!(game.flagged_fields.len(), 3);

        game.toggle_flag((6, 3));
        assert_eq!(game.flagged_fields.len(), 2);
    }
}
