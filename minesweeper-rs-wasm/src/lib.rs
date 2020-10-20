use rand::Rng;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Game {
    pub width: u8,
    pub height: u8,
    cells: Vec<Cell>,
    pub bomb_count: u16
}

#[wasm_bindgen]
impl Game {
    pub fn new(width: u8, height: u8, bomb_count: u16) -> Game {
        let cell_count: u16 = (width as u16)*(height as u16);
        assert!(bomb_count < cell_count);

        let mut rng = rand::thread_rng();

        let mut possible_bomb_placements = Vec::new();

        for i in 0..cell_count {
            possible_bomb_placements.push(i);
        }

        let mut bomb_placements = Vec::new();
        for _ in 0..bomb_count {
            let index: usize = rng.gen_range(0, possible_bomb_placements.len());
            bomb_placements.push(possible_bomb_placements[index]);
            possible_bomb_placements.remove(index);
        }

        let mut cells = Vec::new();

        for i in 0..cell_count {
            cells.push(
                Cell::new(if bomb_placements.contains(&i) {CellType::Bomb} else {CellType::Empty})
            );
        }

        Game {
            width,
            height,
            cells,
            bomb_count
        }
    }

    pub fn get_cell(&self, x: u8, y: u8) -> Cell {
        assert!(x < self.width);
        assert!(y < self.height);
        self.cells[y as usize*self.width as usize+x as usize]
    }

    fn get_cell_ref(&self, x: u8, y: u8) -> &Cell {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.cells[y as usize*self.width as usize+x as usize]
    }

    fn get_cell_ref_mut(&mut self, x: u8, y: u8) -> &mut Cell {
        assert!(x < self.width);
        assert!(y < self.height);
        &mut self.cells[y as usize*self.width as usize+x as usize]
    }

    fn get_neightboring_coords(&self, x: u8, y: u8) -> Vec<(u8, u8)> {
        assert!(x < self.width);
        assert!(y < self.height);

        let max_x = self.width-1;
        let max_y = self.height-1;
        
        let mut coords = Vec::new();

        if y > 0 {
            if x > 0 {coords.push((x-1, y-1))}
            coords.push((x, y-1));
            if x < max_x {coords.push((x+1, y-1))}
        }
        if y < max_y {
            if x > 0 {coords.push((x-1, y+1))}
            coords.push((x, y+1));
            if x < max_x {coords.push((x+1, y+1))}
        }
        if x > 0 {coords.push((x-1, y))}
        if x < max_x {coords.push((x+1, y))}

        return coords
    }

    fn get_neighboring_cells(&self, x: u8, y: u8) -> Vec<&Cell> {
        assert!(x < self.width);
        assert!(y < self.height);

        let mut cells = Vec::new();
        let neightboring_coords = self.get_neightboring_coords(x, y);
        for (x, y) in neightboring_coords.iter() {
            cells.push(self.get_cell_ref(*x, *y));
        }

        return cells
    }

    pub fn get_neighboring_bomb_count(&self, x: u8, y: u8) -> u16 {
        assert!(x < self.width);
        assert!(y < self.height);

        let mut bomb_count: u16 = 0;

        let neighboring_cells = self.get_neighboring_cells(x, y);

        for cell in neighboring_cells.iter() {
            if cell.cell_type == CellType::Bomb {
                bomb_count += 1;
            }
        }

        return bomb_count
    }

    pub fn reveal_cell(&mut self, x: u8, y: u8) -> bool {
        assert!(x < self.width);
        assert!(y < self.height);

        let cell = self.get_cell_ref_mut(x, y);
        assert!(cell.cell_status == CellStatus::Hidden);

        let mut killed = false;

        if cell.cell_type == CellType::Empty {
            cell.cell_status = CellStatus::Revealed;
            if self.get_neighboring_bomb_count(x, y) == 0 {
                let neightboring_coords = self.get_neightboring_coords(x, y);
                for (x, y) in neightboring_coords.iter() {
                    if self.get_cell_ref(*x, *y).cell_status == CellStatus::Hidden {
                        self.reveal_cell(*x, *y);
                    }
                }
            }
        } else {
            cell.cell_status = CellStatus::Killed;
            killed = true;
        }

        return killed
    }

    pub fn toggle_cell_flag(&mut self, x: u8, y: u8) {
        assert!(x < self.width);
        assert!(y < self.height);

        let cell = self.get_cell_ref_mut(x, y);
        assert!(cell.cell_status == CellStatus::Hidden || cell.cell_status == CellStatus::Flagged);

        if cell.cell_status == CellStatus::Hidden {
            cell.cell_status = CellStatus::Flagged;
        } else {
            cell.cell_status = CellStatus::Hidden;
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CellType {
    Empty,
    Bomb
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CellStatus {
    Hidden,
    Flagged,
    Revealed,
    Killed
}

#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    pub cell_type: CellType,
    pub cell_status: CellStatus
}

#[wasm_bindgen]
impl Cell {
    pub fn new(cell_type: CellType) -> Cell {
        Cell {
            cell_type: cell_type,
            cell_status: CellStatus::Hidden
        }
    }
}