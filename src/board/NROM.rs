/*
  (c) 2014 by Jeffrey Quesnelle

  This program is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use rom::Rom;
use std::rc::Rc;
use std::cell::RefCell;
use super::Board;

pub struct Nrom {
  rom: Rc<RefCell<Rom>>
}

impl Board for Nrom {

  fn new(_dummy: Option<Nrom>, rom: &Rc<RefCell<Rom>>) -> Nrom {
    return Nrom {
      rom: rom.clone()
    }
  }

  fn get_name(&self) -> &'static str {
    let rom = self.rom.borrow();
    match rom.header[4] {
      1 => "NROM-128",
      2 => "NROM-256",
      _ => "NROM (non-standard)"
    }
  }
}
