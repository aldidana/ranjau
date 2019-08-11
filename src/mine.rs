use rand::Rng;

#[derive(Debug, Clone)]
struct Mine {
    position: (u32, u32),
    active: bool
}

pub struct MineField {
    size: (u32, u32),
    mines: Vec<Mine>
}

impl MineField {
    pub fn new(x: u32, y: u32) -> MineField {
        MineField {
            size: (x, y),
            mines: Vec::new()
        }
    }

    pub fn find_by_coord(&mut self, x: u32, y: u32) -> bool {
        for mine in self.mines.iter() {
            if mine.position.0 == x && mine.position.1 == y && mine.active == false {
                println!("Yay!");
                return false
            }
        }

        true
    }

    fn generate_empty_fields(&mut self) {
        let mut x = self.size.0;
        let mut y = self.size.1;

        while x > 0 {
            while y > 0 {
                let mine = Mine{
                    position: (x-1, y-1),
                    active: false
                };

                &self.mines.push(mine);

                y = y - 1;
            }

            y = self.size.1;
            x = x - 1;
        }
    }

    pub fn generate_mines(&mut self) {
        &self.generate_empty_fields();

        for mine in self.mines.iter_mut() {
            if 1 == rand::thread_rng().gen_range(0, 3) {
                (*mine).active = true
            }
        }

        &self.mines.reverse();
    }

    pub fn print_fields(&self) {
        let y = self.size.1;

        for mine in self.mines.iter() {
            print!("x"); 

            if (y-1) == mine.position.1 {
                println!("");
            }
        }

        println!("");
    }

    pub fn print_fields_solved(&self) {
        let y = self.size.1;

        for mine in self.mines.iter() {
            if mine.active {
                print!("o");
            } else {
                print!("x");
            }

            if (y-1) == mine.position.1 {
                println!("");
            }
        }

        println!("");
    }
}