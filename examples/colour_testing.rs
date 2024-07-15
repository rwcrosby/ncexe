extern crate ncexe;

use std::collections::HashMap;

use anyhow::{anyhow, bail, Result, Context};

use ncexe::windows::screen::SCREEN;
use ncexe::color;
use pancurses::chtype;

fn main() -> Result<()> {

    let w = &SCREEN.win;
    color::init("dark");

    let _c = color::Colors::new("dark");

    let colors = color_map().unwrap();

    show_colors("Normal", w, &colors, pancurses::A_NORMAL, 2)?;
    show_colors("Bold", w, &colors, pancurses::A_BOLD, 14)?;

    w.refresh();
    w.getch();

    Ok(())

}

fn show_colors(
    title: &str, 
    w: &pancurses::Window,
    colors: &Box<HashMap<&str, u8>>, 
    attr: pancurses::chtype,
    y_start: i32 
) -> Result<()> {

    pancurses::init_pair(255, 0, 7);

    w.mv(y_start, 2);
    w.attrset(pancurses::COLOR_PAIR(255) + pancurses::A_NORMAL);
    w.addstr(title);

    for (y, fgr) in [
        "yellow", 
        "orange",
        "red",
        "magenta",
        "violet",
        "blue",
        "cyan",
        "green",
        ].iter().enumerate() {

        w.mv(y as i32 + y_start + 2, 2);
        
        for (x, bgr) in [
            "base03", 
            "base02",
            "base01",
            "base00",
            "base0",
            "base1",
            "base2",
            "base3",

            ].iter().enumerate() {

            let fc = colors[fgr] as i16;
            let bc = colors[bgr] as i16;

            let pno = (x + (y * 8 ) + 1) as i16;

            pancurses::init_pair(pno, fc, bc );
            if w.attrset(pancurses::COLOR_PAIR(pno as chtype) + attr) != 0
                { bail!("attrset") };

            if w.addstr(format!("{}/{:03.3},{}/{:03.3} ", 
                        &fgr[0..2], fc , &bgr[4..], bc)) != 0 
                { bail!("addstr") };
                
        }

    }

    Ok(())

}

// ------------------------------------------------------------------------
/// Setup colors, see:
/// https://www.ditig.com/publications/256-colors-cheat-sheet

pub fn color_map<'a> () 
    -> Result<Box<HashMap<&'a str, u8>>>
{

    let color_map = Box::new(HashMap::from([
        ("base03", 0xe8), 
        ("base02", 0xec),
        ("base01", 0xef),
        ("base00", 0xf2),
        ("base0", 0xf5),
        ("base1", 0xf8),
        ("base2", 0xfb),
        ("base3", 0xff),
        ("yellow", 226),
        ("orange", 166),
        ("red", 160),
        ("magenta", 127),
        ("violet", 62),
        ("blue", 32),
        ("cyan", 43),
        ("green", 34),
    ]));

/*     let solarize_map = Box::new(HashMap::from([
        ("base03", rgb_to_xterm256("000000")?), 
        ("base02", rgb_to_xterm256("073642")?),
        ("base01", rgb_to_xterm256("586e75")?),
        ("base00", rgb_to_xterm256("657b83")?),
        ("base0", rgb_to_xterm256("839496")?),
        ("base1", rgb_to_xterm256("93a1a1")?),
        ("base2", rgb_to_xterm256("eee8d5")?),
        ("base3", rgb_to_xterm256("fdf6e3")?),
        ("yellow", rgb_to_xterm256("b58900")?),
        ("orange", rgb_to_xterm256("cb4b16")?),
        ("red", rgb_to_xterm256("dc322f")?),
        ("magenta", rgb_to_xterm256("d33682")?),
        ("violet", rgb_to_xterm256("6c71c4")?),
        ("blue", rgb_to_xterm256("268bd2")?),
        ("cyan", rgb_to_xterm256("2aa198")?),
        ("green", rgb_to_xterm256("859900")?),
    ]));
 */    
    Ok(color_map)
    
}

// ------------------------------------------------------------------------
/// From: https://github.com/tmux/tmux/blob/master/colour.c

fn _rgb_to_xterm256(rgb_str: &str) -> Result<u8> {

    const Q2C: [usize; 6] = [0x00, 0x5f, 0x87, 0xaf, 0xd7, 0xff ];


    let hexstr = hex::decode(rgb_str)
        .context(anyhow!("Bad rgb string {}", rgb_str))?;
    let r = hexstr[0] as usize;
    let g = hexstr[1] as usize;
    let b = hexstr[2] as usize;

    let qr = _colour_to_6cube(r); let cr = Q2C[qr as usize];
	let qg = _colour_to_6cube(g); let cg = Q2C[qg as usize];
	let qb = _colour_to_6cube(b); let cb = Q2C[qb as usize];

    // If we have hit the colour exactly, return early.
	if cr == r && cg == g && cb == b {
        return Ok((16 + (36 * qr) + (6 * qg) + qb) as u8);
    }

    // Work out the closest grey (average of RGB).
	let grey_avg = (r + g + b) / 3;

    let grey_idx = 	if grey_avg > 238 {
	    	23
        } else {
		    (grey_avg - 3) / 10
        };
	let grey = 8 + (10 * grey_idx);    

    // Is grey or 6x6x6 colour closest? 
    Ok((if _colour_dist(cr, cg, cb, r, g, b) > _colour_dist( grey, grey, grey, r, g, b) { 
             232 + grey_idx 
        } else {
            16 + (36 * qr) + (6 * qg) + qb
        } ) as u8
    )

}

// ------------------------------------------------------------------------

fn _colour_dist( r1: usize, g1: usize, b1: usize, r2: usize, g2: usize, b2: usize ) -> isize {

    let rd = r1 as isize - r2 as isize;
    let gd = g1 as isize - g2 as isize;
    let bd = b1 as isize - b2 as isize;

    rd * rd + gd * gd + bd * bd 

}

// ------------------------------------------------------------------------

fn _colour_to_6cube(v: usize) -> usize {

    if v < 48       { 0 }
    else if v < 114 { 1 }
    else              { (v - 35) / 40 }

}
