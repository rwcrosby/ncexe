#![allow(dead_code)]
//! Setup colors
//! 
//! Seem to be constrained to 256 colors, rgb doesn't work
//! 
//! https://ethanschoonover.com/solarized/

use anyhow::{anyhow, bail, Context, Result};
use hex;
use pancurses;
use std::collections::HashMap;

// ------------------------------------------------------------------------
/// Color numbers for the various parts of the window
/// 
pub struct ColorSet {

    pub frame: usize,
    pub title: usize,
    pub text: usize,
    pub value: usize,
    
}

// ------------------------------------------------------------------------
/// Overall container for color management
/// Map key is window name, value is the set of colors to use

pub struct Colors<'a> {

    map: HashMap<&'a str, Box<ColorSet>>

}

impl<'a> Colors<'a> {

    pub fn new() -> Result<Box<Colors<'a>>> {

        if !pancurses::has_colors() {
            bail!{"Colors not supported"};
        }

        pancurses::start_color();

        // let num_colors = pancurses::COLORS();
        // let num_pairs = pancurses::COLOR_PAIRS();

        // init_colors();

        // println!("Colors {}, pairs {}", num_colors, num_pairs);

        Ok(
            Box::new(
                Colors{map: 
                    HashMap::from([
                        ("file_list", solarize_color_pairs_1()?),
                        ("header", solarize_color_pairs_2()?),
                    ])}))

    }

    pub fn set(&self, name: &str) -> &ColorSet {
        &self.map[name]
    }

}

// ------------------------------------------------------------------------

fn colour_dist( r1: usize, g1: usize, b1: usize, r2: usize, g2: usize, b2: usize ) -> isize {

    let rd = r1 as isize - r2 as isize;
    let gd = g1 as isize - g2 as isize;
    let bd = b1 as isize - b2 as isize;

    rd * rd + gd * gd + bd * bd 

}

// ------------------------------------------------------------------------

fn colour_to_6cube(v: usize) -> usize {

    if v < 48       { 0 }
    else if v < 114 { 1 }
    else              { (v - 35) / 40 }

}

// ------------------------------------------------------------------------

fn init_color_pair(pair_no: usize, fgr_no: u8, bkgr_no: u8 ) 
    -> Result<usize> {

    match pancurses::init_pair(pair_no as i16, fgr_no as i16, bkgr_no as i16) {
        pancurses::OK => Ok(pair_no),
        _ => Err(anyhow!("init_pair failed"))
    }

}

// ------------------------------------------------------------------------
/// From: https://github.com/tmux/tmux/blob/master/colour.c

fn rgb_to_xterm256(rgb_str: &str) -> Result<u8> {

    const Q2C: [usize; 6] = [0x00, 0x5f, 0x87, 0xaf, 0xd7, 0xff ];


    let hexstr = hex::decode(rgb_str)
        .context(anyhow!("Bad rgb string {}", rgb_str))?;
    let r = hexstr[0] as usize;
    let g = hexstr[1] as usize;
    let b = hexstr[2] as usize;

    let qr = colour_to_6cube(r); let cr = Q2C[qr as usize];
	let qg = colour_to_6cube(g); let cg = Q2C[qg as usize];
	let qb = colour_to_6cube(b); let cb = Q2C[qb as usize];

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
    Ok((if colour_dist(cr, cg, cb, r, g, b) > colour_dist( grey, grey, grey, r, g, b) { 
             232 + grey_idx 
        } else {
            16 + (36 * qr) + (6 * qg) + qb
        } ) as u8
    )

}

// ------------------------------------------------------------------------

fn solarize_color_pairs_1 ()
    -> Result<Box<ColorSet>> {

    let sc = solarize_colors()?;

    let frame = init_color_pair(1, sc["magenta"], sc["base3"])?;
    let title = init_color_pair(2, sc["green"], sc["base3"])?;
    let text = init_color_pair(3, sc["red"], sc["base3"])?;

    Ok(Box::new(ColorSet{frame, title, text, value: 0}))

}

// ------------------------------------------------------------------------

fn solarize_color_pairs_2 ()
    -> Result<Box<ColorSet>> {

    let sc = solarize_colors()?;

    let frame = init_color_pair(4, sc["violet"], sc["base2"])?;
    let title = init_color_pair(5, sc["cyan"], sc["base2"])?;
    let text = init_color_pair(6, sc["blue"], sc["base2"])?;
    let value = init_color_pair(7, sc["green"], sc["base2"])?;

    Ok(Box::new(ColorSet{frame, title, text, value}))

}

// ------------------------------------------------------------------------

fn solarize_colors<'a> () 
    -> Result<Box<HashMap<&'a str, u8>>>
{

    let solarize_map = Box::new(HashMap::from([
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

    Ok(solarize_map)

}

// ------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use pancurses::newwin;

    use super::*;

    use std::env;

    #[test]
    fn color_info() {

        env::set_var("TERM", "screen-256color");

        let mw = pancurses::initscr();
        pancurses::noecho();

        mw.refresh();

        let c = Colors::new().unwrap();


        let w = newwin(10, 30, 5, 5);

        w.bkgd(pancurses::COLOR_PAIR(c.map["file_list"].frame as u32));
        w.attrset(pancurses::COLOR_PAIR(c.map["file_list"].frame as u32));
        w.draw_box(0, 0);

        w.attrset(pancurses::COLOR_PAIR(c.map["file_list"].title as u32));
        w.mvaddstr(0, 0, "A Title");

        w.attrset(pancurses::COLOR_PAIR(c.map["file_list"].text as u32));
        w.mvaddstr(1, 2, "Some Text");

        w.attrset(pancurses::COLOR_PAIR(c.map["file_list"].text as u32) | pancurses::A_REVERSE);
        w.mvaddstr(2, 2, "Some Text Reversed");

        w.getch();

        pancurses::endwin();

    }

    #[test]
    fn rgb_2_xterm() {

        let c1 = rgb_to_xterm256("b58900").unwrap();
        let c2 = rgb_to_xterm256("dc322f").unwrap();
        let c3 = rgb_to_xterm256("d33682").unwrap();
        println!("Yellow {}, Red {}, Magenta {}", c1, c2, c3);

        assert!(c1==136);
        assert!(c2==166);
        assert!(c3==168);

        let c2 = solarize_colors();
        println!{"{:?}", c2};

    }

}
