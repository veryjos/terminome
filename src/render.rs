use std::io::{stdout, Write};
use std::cmp::max;

use term_size;

use crate::glyphs::GLYPHS;

pub fn render_time(time: f64) {
    // figure out which glyphs we need to draw
    let minutes = (time / 60.0).floor();
    let seconds = (time % 60.0).floor();

    let output = format!("{:02}:{:02}", minutes, seconds);

    // get the glyphs we're drawing
    let glyphs = output.chars()
        .map(|ch| GLYPHS[(ch as usize) - ('0' as usize)]);

    // precalculate the size of what we're rendering so we can center it
    let output_bounds = glyphs.clone()
        .map(|glyph| (glyph[0].chars().count(), glyph.len()))
        .fold((0, 0), |acc, (width, height)| (acc.0 + width, max(acc.1, height)));

    // little helper to write to the screen
    let mut write = {
        let mut stdout = stdout();

        move |out: &dyn AsRef<[u8]>| {
            stdout.write(out.as_ref()).expect("Failed to write to stdout");
        }
    };

    // clear the screen
    write(&"\x1b[2;J");

    // initialize the cursor at the center position
    let terminal_bounds = term_size::dimensions()
        .expect("Failed to get terminal dimensions");

    let origin = (
        (terminal_bounds.0 / 2) - (output_bounds.0 / 2),
        (terminal_bounds.1 / 2) - (output_bounds.1 / 2),
    );

    // draw each glyph
    let mut cursor = origin.clone();

    for glyph in glyphs {
        cursor.1 = origin.1;

        let mut width = 0;
        for line in glyph {
            // move to the beginning of the line
            write(&format!("\x1b[{};{}H", cursor.1, cursor.0));
            write(line);

            cursor.1 += 1;
            width = max(width, line.chars().count());
        }

        cursor.0 += width;
    }

    // move cursor to bottom left corner so its out of the way
    write(&format!("\x1b[{};1H", terminal_bounds.1));

    write(&"\n");
}
