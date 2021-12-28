// Answer is TODO
use std::fs;
use std::collections::HashMap;

const FILE_NAME: &str = "day11.txt"; 
const NUM_OF_STEPS: u8 = 4;
const MAX_OCTOPUS_ENERGY: u8 = 9;
const RADIX: u32 = 10;

fn main() {
    let mut file_contents = fs::read_to_string(FILE_NAME)
        .expect("Something went wrong reading the file");

    file_contents.pop();

    let octopus_col_count = file_contents.find("\n").unwrap() as u8;

    let mut octopi: HashMap<Position, Octopus> = HashMap::new();
    for (x, line) in file_contents.lines().enumerate() {
        for (y, energy) in line.chars().enumerate() {
            let position = Position { 
                x: x as u8,
                y: y as u8,
            };

            let octopus = Octopus {
                energy: energy.to_digit(RADIX).unwrap() as u8,
                needs_to_flash: false,
                flashed: false,
            };

            octopi.insert(position, octopus);
        }
    }

    let mut total_octopus_flashes = 0; 
    for q in 0..NUM_OF_STEPS { // TODO q -> _

        println!("step: {}", q);
        show_octopi(&file_contents, &octopi);

        for mut octopus in octopi.values_mut() {
            octopus.reset();
        }
 
        let mut an_octopus_needs_to_flash = false;
        
        for mut octopus in &mut octopi.values_mut() {
            octopus.increment_energy();
            
            if octopus.needs_to_flash() {
                octopus.set_needs_to_flash(true);
                an_octopus_needs_to_flash = true;
            }
        }

        show_octopi(&file_contents, &octopi);

        while an_octopus_needs_to_flash {
            let mut an_octopus_needs_to_flash = false;

            let mut flash_positions: Vec::<Position> = Vec::new();
            for (position, octopus) in &octopi {
                if octopus.needs_to_flash {
                    flash_positions.push(*position);
                }
            }

            for p in &flash_positions {
                print!("{}", p.x);
                print!("{}", p.y);
                println!("");
            }

            for octopus in octopi.values_mut() {
                if octopus.needs_to_flash {
                    octopus.flash();
                    total_octopus_flashes += 1;
                }
            }

            increment_adjacent_octopi_energy(flash_positions,
                                             octopus_col_count,
                                             &mut octopi);

            for octopus in octopi.values() {
                if octopus.needs_to_flash == true {
                    an_octopus_needs_to_flash = true;
                    break;
                }
            }

            if !an_octopus_needs_to_flash {
                break; 
            }
        }
    }

    println!("total");
    println!("{}", total_octopus_flashes);

}

fn show_octopi(file_contents: &String, octopi: &HashMap<Position, Octopus>) {
    for (x, line) in file_contents.lines().enumerate() {
        for (y, energy) in line.chars().enumerate() {
            let position = Position { 
                x: x as u8,
                y: y as u8,
            };

            print!("{}", octopi.get(&position).unwrap().energy);
        }
        println!("");
    }
    println!("");
}

struct Octopus {
    energy: u8,
    needs_to_flash: bool,
    flashed: bool,
}

impl Octopus {
    fn increment_energy(&mut self) {
        self.energy += 1;
    }

    fn needs_to_flash(&self) -> bool {
        return self.energy > MAX_OCTOPUS_ENERGY;
    }

    fn set_needs_to_flash(&mut self, val: bool) {
        self.needs_to_flash = val
    }

    fn reset(&mut self) {
        self.needs_to_flash = false;
        self.flashed = false;
    }

    fn flash(&mut self) {
        self.flashed = true;
        self.needs_to_flash = false;
        self.energy = 0;
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn has_left_side(&self) -> bool {
        return !self.x == 0;
    }

    fn get_left_side(&self) -> Position {
        return Position { 
            x: self.x - 1,
            y: self.y,
        };
    }

    fn has_right_side(&self, octopus_col_count: u8) -> bool {
        return !self.x == octopus_col_count - 1;
    }

    fn get_right_side(&self) -> Position {
        return Position {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn has_top_side(&self) -> bool {
        return !self.y == 0;
    }

    fn get_top_side(&self) -> Position {
        return Position {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn has_bottom_side(&self, octopus_row_count: u8) -> bool {
        return !self.y == octopus_row_count - 1;
    }

    fn get_bottom_side(&self) -> Position {
        return Position {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn has_top_left_side(&self) -> bool {
        return self.has_top_side()
            && self.has_left_side();
    }

    fn get_top_left_side(&self) -> Position {
        return Position {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    fn has_top_right_side(&self, octopus_col_count: u8) -> bool {
        return self.has_top_side()
            && self.has_right_side(octopus_col_count);
    }

    fn get_top_right_side(&self) -> Position {
        return Position {
            x: self.x + 1,
            y: self.y - 1,
        }
    }

    fn has_bottom_left_side(&self, octopus_row_count: u8) -> bool {
        return self.has_bottom_side(octopus_row_count)
            && self.has_left_side();
    }

    fn get_bottom_left_side(&self) -> Position {
        return Position {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn has_bottom_right_side(&self,
                        octopus_row_count: u8,
                        octopus_col_count: u8) -> bool {

        return self.has_bottom_side(octopus_row_count)
            && self.has_right_side(octopus_col_count);
    }

    fn get_bottom_right_side(&self) -> Position {
        return Position {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

fn increment_adjacent_octopi_energy(flash_positions: Vec<Position>,
                                    octopus_col_count: u8,
                                    octopi: &mut HashMap<Position, Octopus>) {

    let octopus_row_count = octopi.len() as u8 / octopus_col_count;

    for flash_position in flash_positions {
        if flash_position.has_left_side() {
            octopi.get_mut(&flash_position.get_left_side()).unwrap().increment_energy();
        }
        
        if flash_position.has_right_side(octopus_col_count) {
            octopi.get_mut(&flash_position.get_right_side()).unwrap().increment_energy();
        }

        if flash_position.has_top_side() {
            octopi.get_mut(&flash_position.get_top_side()).unwrap().increment_energy();
        }
     
        if flash_position.has_bottom_side(octopus_row_count) {
            octopi.get_mut(&flash_position.get_bottom_side()).unwrap().increment_energy();
        }

        if flash_position.has_top_left_side() {
            octopi.get_mut(&flash_position.get_top_left_side()).unwrap().increment_energy();
        }
        
        if flash_position.has_top_right_side(octopus_col_count) {
            octopi.get_mut(&flash_position.get_top_right_side()).unwrap().increment_energy();
        }

        if flash_position.has_bottom_left_side(octopus_row_count) {
            octopi.get_mut(&flash_position.get_bottom_left_side()).unwrap().increment_energy();
        }
     
        if flash_position.has_bottom_right_side(octopus_row_count, octopus_col_count) {
            octopi.get_mut(&flash_position.get_bottom_right_side()).unwrap().increment_energy();
        }
    }
}
