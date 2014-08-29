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

use board::NROM::Nrom;
use emulator::Emulator;
use rom::Rom;
use std::rc::Rc;
use std::cell::RefCell;

mod NROM;

pub trait Board {
  //see my SO question: http://stackoverflow.com/questions/25563667/implementing-rust-traits-cause-struct-to-not-be-found
  fn new(_dummy: Option<Self>, rom: &Rc<RefCell<Rom>>) -> Self;
  fn get_name(&self) -> &'static str;
}

pub fn load_board(emulator: &Emulator, rom_ref: &Rc<RefCell<Rom>>) -> Result<Box<NROM>, String> {
  let mut mapper_no: u8;
  {
    let rom = &rom_ref.borrow();
    mapper_no = (rom.header[6] >> 4) | (rom.header[7] & 0xf0);
  }
  let board = match mapper_no {
    0 => box Nrom::new(None::<Nrom>, rom_ref),
    _ => return Err("Unimplemented board".to_string())
  };
  log_debug!(emulator.logger "Using board {}", board.get_name());
  return Ok(board);
}
