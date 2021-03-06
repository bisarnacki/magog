use crate::cache;
use calx::split_line;
use euclid::default::{Point2D, Rect};
use std::io;
use std::io::prelude::*;
use std::mem;
use std::str;
use std::sync::Arc;
use vitral::{color, Align, Canvas, FontData, Rgba};

struct Message {
    expire_time_s: f64,
    text: String,
}

impl Message {
    fn new(text: String, time_start_s: f64) -> Message {
        const TIME_TO_READ_CHAR_S: f64 = 0.1;
        let expire_time_s = time_start_s + text.len() as f64 * TIME_TO_READ_CHAR_S;
        Message {
            expire_time_s,
            text,
        }
    }
}

/// Output text container.
pub struct Console {
    font: Arc<FontData>,
    lines: Vec<Message>,
    input_buffer: String,
    output_buffer: String,
    done_reading_s: f64,
}

impl Default for Console {
    fn default() -> Self {
        Console {
            font: cache::font(),
            lines: Vec::new(),
            input_buffer: String::new(),
            output_buffer: String::new(),
            done_reading_s: 0.0,
        }
    }
}

impl Console {
    /// Draw the console as a regular message display.
    pub fn draw_small(&mut self, canvas: &mut Canvas, screen_area: &Rect<i32>) {
        let t = calx::precise_time_s();
        let mut lines = Vec::new();
        // The log can be very long, and we're always most interested in the latest ones, so
        // do a backwards iteration with an early exist once we hit a sufficiently old item.
        for msg in self.lines.iter().rev().take_while(|m| m.expire_time_s > t) {
            // The split_line iterator can't be reversed, need to do a bit of caching here.
            lines.extend(
                split_line(
                    &msg.text,
                    |c| self.font.char_width(c).unwrap_or(0),
                    screen_area.size.width,
                )
                .map(|x| x.to_string()),
            );
        }

        // Draw the lines
        let mut pos = screen_area.origin;
        for line in lines.iter().rev() {
            pos = canvas.draw_text(&*self.font, pos, Align::Left, color::WHITE.alpha(0.4), line);
        }
    }

    /// Draw the console as a big drop-down with a command prompt.
    pub fn draw_large(&mut self, canvas: &mut Canvas, screen_area: &Rect<i32>) {
        // TODO: Store color in draw context.
        let color = Rgba::from([0.6, 0.6, 0.6]);
        let background = Rgba::from([0.0, 0.0, 0.6, 0.8]);

        canvas.fill_rect(screen_area, background);

        let h = self.font.height;
        let mut lines_left = (screen_area.size.height + h - 1) / h;
        let mut y = screen_area.max_y() - h;

        // TODO: Handle enter with text input.
        // TODO: Command history.
        // TODO
        /*
        canvas
            .bound(0, y as u32, screen_area.size.width as u32, h as u32)
            .text_input(color, &mut self.input_buffer);
        */
        y -= h;
        lines_left -= 1;

        for msg in self.lines.iter().rev() {
            // XXX: Duplicated from draw_small.
            let fragments = split_line(
                &msg.text,
                |c| self.font.char_width(c).unwrap_or(0),
                screen_area.size.width,
            )
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
            for line in fragments.iter().rev() {
                canvas.draw_text(&*self.font, Point2D::new(0, y), Align::Left, color, line);
                y -= h;
                lines_left -= 1;
            }

            if lines_left <= 0 {
                break;
            }
        }
    }

    fn end_message(&mut self) {
        let mut message_text = String::new();
        mem::swap(&mut message_text, &mut self.output_buffer);

        let now = calx::precise_time_s();
        if now > self.done_reading_s {
            self.done_reading_s = now;
        }

        let message = Message::new(message_text, now);
        self.done_reading_s = message.expire_time_s;
        self.lines.push(message);
    }

    pub fn get_input(&mut self) -> String {
        let mut ret = String::new();
        mem::swap(&mut ret, &mut self.input_buffer);
        ret
    }
}

impl Write for Console {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = str::from_utf8(buf).expect("Wrote non-UTF-8 to Console");
        let mut lines = s.split('\n');
        if let Some(line) = lines.next() {
            self.output_buffer.push_str(line);
        }

        for line in lines {
            self.end_message();
            self.output_buffer.push_str(line);
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}
