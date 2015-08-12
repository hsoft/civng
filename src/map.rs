/* Copyright 2015 Virgil Dupras
 *
 * This software is licensed under the "GPLv3" License as described in the "LICENSE" file,
 * which should be included with this package. The terms are also available at
 * http://www.gnu.org/licenses/gpl-3.0.html
 */

use hexpos::{Pos, Direction};
use unit::Unit;
use terrain::TerrainMap;

pub struct LiveMap {
    terrain: TerrainMap,
    units: Vec<Unit>,
}

impl LiveMap {
    pub fn new(terrain: TerrainMap) -> LiveMap {
        LiveMap {
            terrain: terrain,
            units: Vec::new(),
        }
    }

    pub fn terrain(&self) -> &TerrainMap {
        &self.terrain
    }

    pub fn units(&self) -> &Vec<Unit> {
        &self.units
    }

    pub fn is_pos_passable(&self, pos: Pos) -> bool {
        if !self.terrain.get_terrain(pos).is_passable() {
            false
        }
        else {
            self.units.iter().all(|u| u.pos() != pos)
        }
    }

    pub fn first_passable(&self) -> Pos {
        for (pos, terrain) in self.terrain.tiles() {
            if terrain.is_passable() {
                if self.units.iter().all(|u| u.pos() != pos) {
                    return pos
                }
            }
        }
        panic!("No tile is passable!");
    }

    pub fn create_unit(&mut self, name: &str, pos: Pos) -> &mut Unit {
        let unit = Unit::new(name, pos);
        self.units.push(unit);
        let newlen = self.units.len();
        &mut self.units[newlen-1]
    }

    pub fn moveunit(&mut self, unit_id: usize, direction: Direction) -> bool {
        let newpos = {
            let unit = &self.units[unit_id];
            let newpos = unit.pos().neighbor(direction);
            if !self.is_pos_passable(newpos) {
                return false
            }
            newpos
        };
        let unit = &mut self.units[unit_id];
        let terrain = self.terrain.get_terrain(newpos);
        unit.move_(direction, terrain)
    }

    pub fn refresh(&mut self) {
        for unit in self.units.iter_mut() {
            unit.refresh();
        }
    }
}

