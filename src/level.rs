use std::{
    fmt::Display, 
    io::{self, BufReader, BufRead, Read}
};

use thiserror::Error;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Cell {
    None,
    Wall,
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub w: usize,
    pub h: usize,
}

#[derive(Debug)]
pub struct Level {
    cells: Vec<Cell>,
    size: Size,
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("Level file is empty")]
    Empty,
    #[error("Io error: {0}")]
    Read(io::Error),
    #[error("Invalid columnt count at line {0}")]
    NonRect(usize),
    #[error("Invalid character at {0}:{1}")]
    InvalidChar(usize, usize),
}

impl Level {
    pub fn load<R: Read>(input: R) -> Result<Self, LoadError> {
        let mut lines = BufReader::new(input).lines();
        let mut cells = Vec::new();

        let first = lines.next()
            .ok_or(LoadError::Empty)?
            .map_err(LoadError::Read)?;

        Self::load_row(&mut cells, &first, 0)?;
        let cols = first.chars().count();
        let mut rows = 1;

        for line in lines {
            let line = line.map_err(LoadError::Read)?;
            if line.chars().count() != cols {
                return Err(LoadError::NonRect(rows + 1));
            }
            Self::load_row(&mut cells, &line, rows)?;
            rows += 1;
        }
        
        Ok(Self {
            cells,
            size: Size { w: cols, h: rows },
        })
    }

    fn load_row(cells: &mut Vec<Cell>, row: &str, row_num: usize) -> Result<(), LoadError> {
        for (col, cell) in row.char_indices() {
            cells.push(match cell {
                '#' => Cell::Wall,
                ' ' => Cell::None,
                _ => return Err(LoadError::InvalidChar(row_num + 1, col + 1)),
            });
        }
        Ok(())
    }

    pub fn cell_at(&self, x: f32, y: f32) -> Option<Cell> {
        if x >= 0.0 && x <= self.size.w as f32 
            && y >= 0.0 && y <= self.size.h as f32 {

            Some(self.cells[(y as usize) * self.size.w + (x as usize)])
        } else {
            None
        }
    }

    #[inline]
    pub fn size(&self) -> Size {
        self.size
    }

    pub fn raytrace(
        &self,
        x: f32, 
        y: f32, 
        angle: f32, 
        max_dist: f32
    ) -> Option<f32> {
        let dir_x = f32::sin(angle);
        let dir_y = -f32::cos(angle);

        let mut dist = 0.0;
        while dist <= max_dist {
            let target_x = x + dir_x * dist;
            let target_y = y + dir_y * dist;

            if let Cell::Wall = self.cell_at(target_x, target_y)? {
                return Some(dist);
            }

            dist += 0.2;
        }
        None
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self.cells.chunks(self.size.w)
            .map(|chunk| {
                chunk.iter().map(|c| match c {
                    Cell::Wall => '#',
                    Cell::None => ' ',
                })
                .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", str)
    }
}


#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_PI_4, PI};

    use super::*;

    #[test]
    fn loaded() {
        let input = "## #\n# ##".as_bytes();
        let level = Level::load(input).unwrap();
        
        assert_matches!(level.size(), Size { w: 4, h: 2 });
        assert_matches!(level.cell_at(3.2, 0.1), Some(Cell::Wall));
        assert_matches!(level.cell_at(1.6, 1.0), Some(Cell::None));
        assert_matches!(level.cell_at(5.6, 14.0), None);
    }

    #[test]
    fn empty() {
        let input = [];
        assert_matches!(Level::load(&input[..]), Err(LoadError::Empty));
    }

    #[test]
    fn non_rect() {
        let input = "####\n# #".as_bytes();
        assert_matches!(Level::load(input), Err(LoadError::NonRect(2)));
    }

    #[test]
    fn invalid() {
        let input = "## a\n".as_bytes();
        assert_matches!(Level::load(input), Err(LoadError::InvalidChar(1, 4)));
    }

    #[test]
    fn display() {
        let input = "## \n###\n # ";
        let level = Level::load(input.as_bytes()).unwrap();

        assert_eq!(format!("{}", level), input);
    }

    #[test]
    fn raytrace() {
        let input = "# ####\n\
                     # ## #\n\
                     #    #\n\
                     ######"
                    .as_bytes();

        let level = Level::load(input).unwrap();
        
        assert_matches!(level.raytrace(1.2, 0.0, 0.0, f32::INFINITY), None);
        assert_matches!(level.raytrace(1.2, 0.0, PI, 1.5), None);
        
        let dist = level.raytrace(1.2, 0.0, PI, f32::INFINITY).unwrap();
        assert!(dist > 2.9 && dist < 4.0);

        let dist = level.raytrace(4.2, 2.5, -FRAC_PI_4, 15.0).unwrap();
        assert!(dist > 0.0 && dist < 1.0);
    }
}
