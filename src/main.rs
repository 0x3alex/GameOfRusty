use std::{thread, time};
use std::time::Duration;
use rand::Rng; // 0.8.5

static DEAD: i8 = 0;
static ALIVE: i8 = 1;

#[derive(Copy, Clone)]
struct Table {
    content: [[i8; 50]; 50]
}

impl Table {
    pub fn new() -> Self {
        Self{
            content: [[1; 50]; 50]
        }
    }

    fn generate(&mut self) {
        for (_,r) in self.content.iter_mut().enumerate() {
            for(_,c) in r.iter_mut().enumerate() {
                *c = rand::thread_rng().gen_range(0..=1)
            }
        }
    }

    fn is_in_bounds(&self, r: usize, c: usize) -> bool {
        return !(r < 0 || r >= self.content.len() || c < 0 || c >= self.content[r].len());
    }
    fn get(&self, r: usize, c: usize) -> Option<i8> {
        if !self.is_in_bounds(r,c) {
            return None;
        }
        return Some(self.content[r][c]);
    }
    fn print(&mut self) {
        for(_,r) in self.content.iter_mut().enumerate() {
            for(_,c) in r.iter_mut().enumerate() {
                match c {
                    1 => print!("🧠"),
                    _ => print!("💀")
                }
            }
            print!("\n");
        }
    }
}


fn get_neighbour_count(table: &Table, row: usize, col: usize) -> i8 {
    let mut total = 0;
    let abs_row = (row as i64) - 1;
    let abs_col = (col as i64) - 1;
    for r in 0..=2 {
        for c in 0..=2 {
            if abs_row+r == row as i64 && abs_col+c == col as i64 {continue;}
            match table.get((abs_row+r) as usize,(abs_col+c) as usize) {
                Some(val) => total += val,
                _ => ()
            }
        }
    }
    return total;
}

fn update_table(table: &Table) -> Table {
    let mut copy_table = table.clone();
    for (ri, r) in copy_table.content.iter_mut().enumerate() {
        for(ci,mut c) in r.iter_mut().enumerate() {
            let n_count = get_neighbour_count(&table,ri,ci);
            if *c == DEAD && n_count == 3 {
                *c = ALIVE;
            }else if *c == ALIVE && (n_count == 2 || n_count == 3) {
                *c = ALIVE;
            }else{
                *c = DEAD;
            }
        }
    }
    return copy_table;
}

pub fn main() -> Result<(), String>  {
    let mut table = Table::new();
    table.generate();
    loop {
        table.print();
        table = update_table(&table);
        println!();
        thread::sleep(time::Duration::from_millis(550));
        print!("{}[2J", 27 as char);
    }
}