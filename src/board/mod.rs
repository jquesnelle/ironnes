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

use emulator::Emulator;
use rom::Rom;

pub trait Board {
  pub fn new(rom: &Rom) -> Self;
  pub fn get_name(&self) -> &'static str;
}

struct Nrom<'a> {
  rom: &'a Rom
}

impl Board for Nrom {

  pub fn new(rom: &Rom) -> Nrom {
    return Nrom {
      rom: rom
    }
  }

  pub fn get_name(&self) -> &'static str {
    match self.rom.header[4] {
      1 => "NROM-128",
      2 => "NROM-256",
      _ => "NROM (non-standard)"
    }
  }
}

pub fn load_board(emulator: &Emulator, rom: &Rom) -> Result<Box<Board>, String> {
  let mapper_no = (rom.header[6] >> 4) | (rom.header[7] & 0xf0);
  match mapper_no {
    0 => Nrom::new(rom),
    _ => return Err("Unimplemented board".to_string())
  }
}
