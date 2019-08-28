use std::io::{stdout, Write};
use std::cmp::max;
use std::cell::RefCell;
use std::time::Duration;

use term_size;

use crate::glyphs::GLYPHS;

pub fn render_time(time: Duration) {
    let (minutes, seconds) = {
        // round to prevent floating point accuracy issues
        let time_secs = time.as_secs_f64().round() as u64;

        (time_secs / 60, time_secs % 60)
    };

    let output = format!("{:02}:{:02}", minutes, seconds);

    // get the glyphs we're drawing
    let glyphs = output.chars()
        .map(|ch| GLYPHS[(ch as usize) - ('0' as usize)]);

    // precalculate the size of what we're rendering so we can center it
    let output_bounds = glyphs.clone()
        .map(|glyph| (glyph[0].chars().count(), glyph.len()))
        .fold((0, 0), |acc, (width, height)| (acc.0 + width, max(acc.1, height)));

    // little helper to write/flush stdout
    let stdout = RefCell::new(stdout());

    let (write, flush) = {
        (
            |out: &dyn AsRef<[u8]>| {
                stdout.borrow_mut().write(out.as_ref()).expect("Failed to write to stdout");
            },
            || {
                stdout.borrow_mut().flush().expect("Faild to flush stdout");
            }
        )

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

    // flush stdout
    flush();
}
