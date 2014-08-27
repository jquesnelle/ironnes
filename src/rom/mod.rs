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

use std::io::{File};
use emulator::Emulator;
use std::kinds::marker::NoCopy;

pub struct Rom {
  pub header: [u8, ..16],
  pub trainer: [u8, ..512],
  pub prg: Vec<u8>,
  pub chr: Vec<u8>,
  pub pc_inst: [u8, ..8192],
  pub pc_prom: [u8, ..32],
  pub name: String,
  _marker: NoCopy
}

impl Rom {
  pub fn load(path: &Path, emulator: &Emulator) -> Result<Box<Rom>, String> {
    let bad_format = String::from_str("Invald ROM format");
    let mut file = match File::open(path) {
      Ok(e) => e,
      Err(_) => return Err(format!("Unable to open {}", path.display()))
    };
    let mut rom = box Rom {
      header: [0, ..16],
      trainer: [0, ..512],
      prg: Vec::new(),
      chr: Vec::new(),
      pc_inst: [0, ..8192],
      pc_prom: [0, ..32],
      name: "".to_string(),
      _marker: NoCopy
    };

    if file.read(rom.header) != Ok(16) {
        return Err(bad_format);
    }

    match rom.header.slice(0, 4) {
      [0x4e, 0x45, 0x53, 0x1a] => (),
      _ => {
        emulator.logger.log_debug("ROM does not have magic bytes in header; not a ROM?");
        return Err(bad_format);
      }
    };

    if (rom.header[6] & 4) == 4 {
      if file.read(rom.trainer) != Ok(512) {
        return Err(bad_format);
      }
    }

    let prg_size = (rom.header[4] as uint) * 1024 * 16;
    rom.prg.grow(prg_size, &(0i as u8));
    if file.read(rom.prg.as_mut_slice()) != Ok(prg_size) {
      return Err(bad_format);
    }

    if rom.header[5] != 0 {
      let chr_size = (rom.header[5] as uint) * 1024 * 8;
      rom.chr.grow(chr_size, &(0i as u8));
      if file.read(rom.chr.as_mut_slice()) != Ok(chr_size) {
        return Err(bad_format);
      }
    }

    if (rom.header[7] & 2) == 2 {
      if file.read(rom.pc_inst) != Ok(8192) {
        return Err(bad_format);
      }

      // according to pc_inst: http://wiki.nesdev.com/w/index.php/PC10_ROM-Images
      // these aren't always here... will we eat the name at the end if they aren't? */

      file.read(rom.pc_prom).unwrap();

    }

    let mut name_buf: Vec<u8> = Vec::new();
    name_buf.grow(128, &(0i as u8));
    match file.read(name_buf.as_mut_slice()) {
      Ok(_) => {
        rom.name = match String::from_utf8(name_buf) {
          Ok(s) => s,
          Err(_) => "".to_string()
        }
      },
      Err(_) => ()
    }

    if rom.name.is_empty() {
      emulator.logger.log_debug(format!("Finished loading ROM {}",path.display()).as_slice());
    }
    else {
      emulator.logger.log_debug(format!("Finished loading ROM {} at {}", rom.name, path.display()).as_slice());
    }

    return Ok(rom);
  }
}
