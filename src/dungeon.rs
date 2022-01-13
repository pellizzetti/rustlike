use std::iter::Chain;
use std::ops::Range;

use bracket_geometry::prelude::*;
use bracket_random::prelude::*;

use crate::map::{Map, FLOOR_TILE};

struct RectangularRoom {
  x1: u32,
  y1: u32,
  x2: u32,
  y2: u32,
}

impl RectangularRoom {
  fn new(x: u32, y: u32, width: u32, heigth: u32) -> RectangularRoom {
    RectangularRoom {
      x1: x,
      y1: y,
      x2: x + width,
      y2: y + heigth,
    }
  }
  fn center(&self) -> Point {
    let center_x = (self.x1 + self.x2) / 2;
    let center_y = (self.y1 + self.y2) / 2;

    Point::new(center_x, center_y)
  }
  fn inner(&self) -> (Range<u32>, Range<u32>) {
    (((self.x1 + 1)..self.x2), ((self.y1 + 1)..self.y2))
  }
  fn intersects(&self, other_room: &RectangularRoom) -> bool {
    self.x1 <= other_room.x2
      && self.x2 >= other_room.x1
      && self.y1 <= other_room.y2
      && self.y2 >= other_room.y1
  }
}

fn tunnel_between<'a>(start: Point, end: Point) -> Chain<Bresenham, Bresenham> {
  let corner_x: u32;
  let corner_y: u32;

  let mut rng = RandomNumberGenerator::new();
  if rng.range(0.0, 1.0) < 0.5 {
    corner_x = end.x as u32;
    corner_y = start.y as u32;
  } else {
    corner_x = start.x as u32;
    corner_y = end.y as u32;
  }

  let start_tunnel = Bresenham::new(start, Point::new(corner_x, corner_y));
  let end_tunnel = Bresenham::new(Point::new(corner_x, corner_y), end);

  start_tunnel.chain(end_tunnel)
}

pub fn generate_dungeon(
  map_width: u32,
  map_height: u32,
  map_max_rooms: u32,
  room_max_size: u32,
  room_min_size: u32,
) -> (Map, Point) {
  let mut dungeon = Map::new(map_width, map_height);

  let mut rooms: Vec<RectangularRoom> = vec![];

  for _ in 0..map_max_rooms {
    let mut rng = RandomNumberGenerator::new();

    let room_width = rng.range(room_min_size, room_max_size);
    let room_height = rng.range(room_min_size, room_max_size);

    let x = rng.range(0, map_width - room_width - 1);
    let y = rng.range(0, map_height - room_height - 1);

    let new_room = RectangularRoom::new(x, y, room_width, room_height);

    let intersects = rooms
      .iter()
      .any(|other_room| new_room.intersects(other_room));

    if intersects {
      continue;
    }

    let new_room_inner = new_room.inner();

    for x in new_room_inner.0 {
      for y in new_room_inner.1.clone() {
        let index = dungeon.map_idx(x, y);
        dungeon.tiles[index] = *FLOOR_TILE;
      }
    }

    let new_room_center = new_room.center();

    if rooms.len() > 0 {
      let last_room_center = rooms.last().unwrap().center();
      for point in tunnel_between(last_room_center, new_room_center) {
        let index = dungeon.map_idx(point.x as u32, point.y as u32);
        dungeon.tiles[index] = *FLOOR_TILE;
      }
    }

    rooms.push(new_room);
  }

  let first_room_center = rooms.first().unwrap().center();

  (dungeon, first_room_center)
}
