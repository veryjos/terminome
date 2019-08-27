use std::io::{stdout, Write};
use std::cmp::max;

use crate::glyphs::GLYPHS;

pub fn render_time(time: f64) {
    // figure out which glyphs we need to draw
    let minutes = (time / 60.0).floor();
    let seconds = (time % 60.0).floor();

    let output = format!("{:02}:{:02}", minutes, seconds);

    // get the glyphs we're drawing
    let glyphs = output.chars()
        .map(|ch| GLYPHS[(ch as usize) - ('0' as usize)]);

    // clear the screen
    let mut stdout = stdout();
    stdout.write(b"\x1b[2;J");

    // draw each glyph
    let mut cursor = (1, 1);

    for glyph in glyphs {
        cursor.0 = 1;

        let mut width = 0;
        for line in glyph {
            // move to the beginning of the line
            stdout.write(format!("\x1b[{};{}H", cursor.0, cursor.1).as_bytes());
            stdout.write(line.as_bytes());

            cursor.0 += 1;
            width = max(width, line.chars().count());
        }

        cursor.1 += width;
    }

    stdout.flush();
}
