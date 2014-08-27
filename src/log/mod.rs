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

extern crate term;

pub enum Level {
  Error,
  Normal,
  Debug
}

pub struct Log {
  pub log_level: Level,
}

impl Log {

  pub fn log_error(&self, message: &str) {
    self.log(message, Error);
  }

  pub fn log_normal(&self, message: &str) {
    self.log(message, Normal);
  }

  pub fn log_debug(&self, message: &str) {
    self.log(message, Debug);
  }

  pub fn log(&self, message: &str, level: Level) {
    if level as int <= self.log_level as int {
      let mut t = term::stdout().unwrap();

      // need these unwraps to avoid warning: unused result which must be used

      (t.reset()).unwrap();
      (write!(t, "[")).unwrap();

      match level {
        Error => { (t.fg(term::color::RED)).unwrap(); (write!(t, "ERROR")).unwrap(); },
        Normal => { (t.fg(term::color::WHITE)).unwrap(); (write!(t, "     ")).unwrap(); },
        Debug => { (t.fg(term::color::YELLOW)).unwrap(); (write!(t, "DEBUG")).unwrap(); }
      }

      (t.reset()).unwrap();
      (write!(t, "] ")).unwrap();

      (t.write_line(message)).unwrap();
    }
  }

}
