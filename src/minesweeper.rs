use rand::{thread_rng, Rng};
use std::{collections::HashSet, fmt::Display};

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_field: HashSet<Position>,
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();
                let mut rng = thread_rng();
                while mines.len() < mine_count {
                    mines.insert((
                        rng.gen_range(0..width) as usize,
                        rng.gen_range(0..height) as usize,
                    ));
                }
                mines
            },
            flagged_field: HashSet::new(),
        }
    }

    pub fn new_with_mines(width: usize, height: usize, mines: Vec<Position>) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: HashSet::from_iter(mines),
            flagged_field: HashSet::new(),
        }
    }

    fn neightbors(&self, (x, y): Position) -> Vec<Position> {
        (x.max(1) - 1..=(x + 1).min(self.width - 1))
            .flat_map(|i| (y.max(1) - 1..=(y + 1).min(self.height - 1)).map(move |j| (i, j)))
            .filter(|(i, j)| (i, j) != (&x, &y))
            .collect()
    }

    pub fn open(&mut self, position: Position) {
        if self.is_lost() || self.flagged_field.contains(&position) {
            return;
        }
        let neighboring_mines = self.neightboring_mines(position);
        if self.open_fields.contains(&position) {
            let neightbors = self.neightbors(position);
            let neighboring_flags = neightbors
                .iter()
                .filter(|n| self.flagged_field.contains(n))
                .count();
            log(format!("neightbors {:?}", neightbors).as_str());
            log(format!("neighboring_flags {:?}", neighboring_flags).as_str());
            log(format!("neighboring_mines {:?}", neighboring_mines).as_str());
            if neighboring_mines == neighboring_flags {
                neightbors.iter().for_each(|n| {
                    if !self.open_fields.contains(&n) {
                        self.open(*n);
                    }
                });
            }
        } else {
            if self.mines.contains(&position) {
                for mine in &self.mines {
                    self.open_fields.insert(mine.clone());
                }
            } else {
                self.open_fields.insert(position);
                if neighboring_mines == 0 {
                    for n in self.neightbors(position) {
                        if !self.open_fields.contains(&n) {
                            self.open(n);
                        }
                    }
                }
            }
        }
    }

    pub fn is_lost(&self) -> bool {
        for m in &self.mines {
            if self.open_fields.contains(&m) {
                return true;
            }
        }
        false
    }

    pub fn is_won(&self) -> bool {
        !self.is_lost() && self.open_fields.len() + self.mines.len() == self.width * self.height
    }

    pub fn neightboring_mines(&self, p: Position) -> usize {
        self.neightbors(p)
            .iter()
            .filter(|p| self.mines.contains(p))
            .count()
    }

    pub fn toggle_flag(&mut self, p: Position) {
        if self.is_lost() {
            return;
        }
        if !self.open_fields.contains(&p) {
            if !self.flagged_field.remove(&p) {
                self.flagged_field.insert(p);
            }
        }
    }
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = &(x, y);
                if !self.open_fields.contains(p) {
                    if self.flagged_field.contains(p) {
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("⬜ ")?;
                    }
                } else if self.mines.contains(p) {
                    f.write_str("💣 ")?;
                } else {
                    f.write_fmt(format_args!(
                        " {} ",
                        // match self.neightboring_mines(*p) {
                        //     0 => "0️⃣",
                        //     1 => "1️⃣",
                        //     2 => "2️⃣",
                        //     3 => "3️⃣",
                        //     4 => "4️⃣",
                        //     5 => "5️⃣",
                        //     6 => "6️⃣",
                        //     7 => "7️⃣",
                        //     8 => "8️⃣",
                        //     _ => "⁉",
                        // }
                        self.neightboring_mines(*p)
                    ))?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Minesweeper;

    #[test]
    fn create_new() {
        let ms = Minesweeper::new(10, 10, 10);
        println!("{:?}", ms);
    }

    use std::{collections::HashSet, hash::Hash};

    fn my_eq<T>(a: &[T], b: &[T]) -> bool
    where
        T: Eq + Hash,
    {
        let a: HashSet<_> = a.iter().collect();
        let b: HashSet<_> = b.iter().collect();

        a == b
    }

    #[test]
    fn neightbors() {
        let ms = Minesweeper::new(10, 10, 10);
        let n = ms.neightbors((1, 1));
        println!("{:?}", n);
        assert!(my_eq(
            &n,
            &[
                (0, 0),
                (1, 0),
                (2, 0),
                (0, 1),
                (2, 1),
                (0, 2),
                (1, 2),
                (2, 2)
            ]
        ));
        let n = ms.neightbors((0, 0));
        println!("{:?}", n);
        assert!(my_eq(&n, &[(1, 0), (1, 1), (0, 1)]));

        let n = ms.neightbors((9, 0));
        println!("{:?}", n);
        assert!(my_eq(&n, &[(8, 0), (8, 1), (9, 1)]));

        let n = ms.neightbors((0, 9));
        println!("{:?}", n);
        assert!(my_eq(&n, &[(0, 8), (1, 8), (1, 9)]));

        let n = ms.neightbors((9, 9));
        println!("{:?}", n);
        assert!(my_eq(&n, &[(8, 9), (8, 8), (9, 8)]));
    }

    // #[test]
    // fn open() {
    //     let mut ms = Minesweeper::new_with_mines(10, 10, vec![(0, 0), (0, 1)]);
    //     let o = ms.open((0, 0));
    //     println!("{:?}", o);
    //     assert_eq!(o, OpenResult::Mine);
    //     let o = ms.open((1, 1));
    //     println!("{:?}", o);
    //     assert_eq!(o, OpenResult::MineCount(2));
    //     let o = ms.open((0, 2));
    //     println!("{:?}", o);
    //     assert_eq!(o, OpenResult::MineCount(1));
    //     let o = ms.open((0, 3));
    //     println!("{:?}", o);
    //     assert_eq!(o, OpenResult::MineCount(0));
    // }

    #[test]
    fn flag() {
        let mut ms = Minesweeper::new_with_mines(10, 10, vec![(0, 0), (0, 1)]);
        ms.toggle_flag((0, 0));
        assert!(ms.flagged_field.contains(&(0, 0)));
        ms.toggle_flag((0, 0));
        assert!(!ms.flagged_field.contains(&(0, 0)));
        ms.open((0, 0));
        ms.toggle_flag((0, 0));
        assert!(!ms.flagged_field.contains(&(0, 0)));
    }

    #[test]
    fn print() {
        let mut ms = Minesweeper::new_with_mines(10, 10, vec![(0, 0), (0, 1)]);
        println!("{}", ms);
        ms.open((0, 0));
        ms.open((0, 1));
        ms.open((0, 2));
        ms.open((0, 3));
        ms.open((1, 0));
        ms.open((1, 1));
        ms.open((1, 2));
        ms.open((1, 3));
        ms.toggle_flag((2, 3));
        println!("{}", ms);
    }

    #[test]
    fn print_str() {
        let ms = Minesweeper::new(10, 10, 10);
        println!("{}", ms.to_string());
    }

    #[test]
    fn open_flag() {
        let mut ms = Minesweeper::new_with_mines(10, 10, vec![(0, 0)]);
        ms.toggle_flag((0, 0));
        ms.open((0, 0));
        println!("{}", ms);
    }
}
