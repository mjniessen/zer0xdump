use std::io;
use std::io::Write;

use clap::ArgMatches;

pub fn hexdump(buffer: Vec<u8>, matches: ArgMatches) {
    let mut i = 0;
    let mut line_len = 0;

    let skip = *matches.get_one::<u64>("SKIP").unwrap();
    let width = *matches.get_one::<u16>("WIDTH").unwrap();
    let use_count = matches.contains_id("COUNT");
    let count = *matches.get_one::<u64>("COUNT").unwrap();

    let mut matched_prev = false;
    let mut hex_bytes = String::from(" ");
    let mut hex_text = String::from(" ");
    let mut prev_line = String::from("");
    let mut offset_text = String::from("");

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // generate color codes.
    let mut gy = "\x1b[38;2;70;70;70m";
    let mut r = "\x1b[38;2;255;10;10m";
    let mut gn = "\x1b[38;2;0;100;255m";
    #[cfg(target_os = "linux")]
    let mut b = "\x1b[38;2;100;200;0m";
    #[cfg(target_os = "windows")]
    let mut b = "\x1b[34m";
    let mut n = "\x1b[0m";
    let mut bold = "\x1b[0;1m";

    // Clear color codes if user specified --no-color switch.
    if matches.contains_id("NOCOLOR") & matches.get_flag("NOCOLOR") {
        gy = "";
        r = "";
        gn = "";
        b = "";
        n = "";
        bold = "\x1b[0;1m";
    }

    // loop through each byte in the buffer.
    for x in &buffer {
        // If the user wants to skip bytes, skip till then.
        if i < skip {
            i += 1;
            continue;
        }

        // If the user is using the count parameter, keep an eye on
        // it and break when hit.
        if use_count && count > 0 && (i - skip) >= count {
            break;
        }

        // If the line is clear, initalize the starting text.
        if line_len == 0 {
            hex_bytes = String::from(" ");
            // hex_text = format!("{} ┋{}", bold, n);
            hex_text = format!("{}  {}", bold, n);
            offset_text = format!("{}{:08x}  ", n, i);
        } else if line_len % 4 == 0 {
            //  If the length is at 32 bit offset, inject a bar.
            hex_bytes.push(' ');
            // hex_text = format!("{}{}┊{}", hex_text, b, n);
            hex_text = format!("{}{} {}", hex_text, b, n);
        }

        //  Populate the buffers accordingly by btye.
        if x == &0x00 {
            hex_bytes = format!("{}{}{:02x}{} ", hex_bytes, gy, x, n);
            hex_text = format!("{}{}·{}", hex_text, gy, n);
        } else if x == &0x0a || x == &0x0d {
            hex_bytes = format!("{}{}{:02x}{} ", hex_bytes, r, x, n);
            hex_text = format!("{}{}·{}", hex_text, r, n);
        } else if x == &0xff {
            hex_bytes = format!("{}{}{:02x}{} ", hex_bytes, gn, x, n);
            hex_text = format!("{}{}·{}", hex_text, gn, n);
        } else if x == &0x40 {
            hex_bytes = format!("{}{}{:02x} ", hex_bytes, gn, x);
            hex_text = format!("{}{}{}", hex_text, gn, *x as char);
        } else if !(&0x20..=&0x7e).contains(&x) {
            hex_bytes = format!("{}{}{:02x}{} ", hex_bytes, b, x, n);
            hex_text = format!("{}{}·{}", hex_text, b, n);
        } else {
            hex_bytes = format!("{}{}{:02x} ", hex_bytes, n, x);
            hex_text = format!("{}{}{}", hex_text, n, *x as char);
        }

        //  increment our counters.
        i += 1;
        line_len += 1;

        if line_len == width {
            // hex_text = format!("{}{}┋", hex_text, bold);
            hex_text = format!("{}", hex_text);
            let curr_line = format!("{}{}", hex_bytes, hex_text);
            if curr_line != prev_line {
                writeln!(stdout, "{}{}", offset_text, curr_line);
                matched_prev = false;
            } else if curr_line == prev_line && !matched_prev {
                writeln!(stdout, " {}┋    ┋{}", gy, n);
                matched_prev = true;
            }
            prev_line = curr_line;
            line_len = 0;
        }
    }

    // Handle any remaining bytes in the buffer if it didn't trigger
    // the width size to dump in the loop.
    if line_len != width && line_len != 0 && hex_bytes.len() > 1 {
        write!(stdout, "{}{}", offset_text, hex_bytes);
        while line_len != width {
            if line_len % 4 == 0 {
                write!(stdout, " ");
            }
            write!(stdout, "   ");
            line_len += 1;
        }
        // hex_text = format!("{}{}┋{}", hex_text, bold, n);
        hex_text = format!("{}{} {}", hex_text, bold, n);
        writeln!(stdout, "{}", hex_text);
    }
    writeln!(stdout);
}
