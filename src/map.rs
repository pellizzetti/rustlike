use bracket_terminal::prelude::*;

lazy_static! {
  pub static ref FLOOR_TILE: Tile =
    Tile::new(true, Graphic::new(' ', (255, 255, 255), (50, 50, 150)),);
  pub static ref WALL_TILE: Tile =
    Tile::new(false, Graphic::new('#', (255, 255, 255), (0, 0, 100)),);
}

#[derive(Copy, Clone)]
pub struct Graphic {
  ch: char,
  fg: (u8, u8, u8),
  bg: (u8, u8, u8),
}

impl Graphic {
  pub fn new(ch: char, fg: (u8, u8, u8), bg: (u8, u8, u8)) -> Graphic {
    Graphic {
      ch: ch,
      fg: fg,
      bg: bg,
    }
  }
}

#[derive(Copy, Clone)]
pub struct Tile {
  pub walkable: bool,
  pub graphic: Graphic,
}

impl Tile {
  pub fn new(walkable: bool, graphic: Graphic) -> Tile {
    Tile {
      walkable: walkable,
      graphic: graphic,
    }
  }
}

pub struct Map {
  width: u32,
  height: u32,
  pub tiles: Box<[Tile]>,
}

impl Map {
  pub fn new(width: u32, height: u32) -> Map {
    let map_size = (width * height) as usize;
    let tiles = vec![*WALL_TILE; map_size].into_boxed_slice();

    Map {
      width: width,
      height: height,
      tiles: tiles,
    }
  }
  pub fn map_idx(&self, x: u32, y: u32) -> usize {
    ((y * self.width) + x) as usize
  }
  pub fn render(&self, ctx: &mut BTerm) {
    ctx.set_active_console(0);
    for y in 0..self.height {
      for x in 0..self.width {
        let index = self.map_idx(x, y);
        ctx.set(
          x,
          y,
          self.tiles[index].graphic.fg,
          self.tiles[index].graphic.bg,
          to_cp437(self.tiles[index].graphic.ch),
        );
      }
    }
  }
}
