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
extern crate ironnes;
extern crate getopts;

use ironnes::log;
use ironnes::emulator::Emulator;
use getopts::{optopt, optflag, getopts, usage};
use std::{os, io};

fn main() {

  let mut emulator = Emulator::new();

  let args: Vec<String> = os::args();
  //let program = args[0].clone();

  let opts = [
    optopt("l", "logLevel", "set the log level", "LEVEL"),
    optflag("h", "help", "prints this help menu")
  ];

  let matches = match getopts(args.tail(), opts) {
    Ok(m) => m,
    Err(f) => fail!(f.to_string())
  };

  if matches.opt_present("h") {
    io::println(usage("ironness", opts).as_slice());
    return;
  }

  let log_level_str = matches.opt_str("l");
  emulator.logger.log_level = match log_level_str {
    Some(s) => match s.as_slice() {
      "debug" => log::Debug,
      "error" => log::Error,
      _ => log::Normal
    },
    None => log::Normal
  };

  if !matches.free.is_empty() {
    let path = Path::new(matches.free[0].clone());
    match emulator.load_rom(&path) {
      Ok(_) => emulator.logger.log_normal(format!("Loaded command line ROM {}", path.display()).as_slice()),
      Err(s) => emulator.logger.log_error(format!("Unable to load command line ROM {}: {}", path.display(), s).as_slice())
    }
  }

}
