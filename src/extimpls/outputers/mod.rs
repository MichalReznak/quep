mod color;
mod python;
mod serial;
mod text;

pub use color::ColorOutputer;
pub use python::PythonOutputer;
pub use serial::SerialOutputer;
pub use text::TextOutputer;

use cli_table::{Cell, CellStruct, Style};
use termcolor::Color;

pub(self) fn res_to_color(val: i32) -> Color {
    let a = (1024 - (1024 * 2 / 3)) / 4;
    let b = 1024 - a;
    let c = b - a;
    let d = c - a;
    let e = d - a;

    if val >= b {
        Color::Rgb(105, 179, 76)
    }
    else if val >= c {
        Color::Rgb(172, 179, 52)
    }
    else if val >= d {
        Color::Rgb(255, 142, 21)
    }
    else if val >= e {
        Color::Rgb(255, 78, 17)
    }
    else {
        Color::Rgb(255, 13, 13)
    }
}
