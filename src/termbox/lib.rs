#![desc = "A Rust wrapper for the termbox library"]
#![license = "MIT"]
#![crate_id = "termbox#0.1.0"]
#![crate_type = "lib" ]

#![feature(globs)]
#![feature(phase)]

#[phase(syntax, link)] extern crate log;
extern crate libc;

use std::task;

pub use libc::types::os::arch::c95::{c_int, c_uint};

/*
 *
 * A lightweight curses alternative wrapping the termbox library.
 *
 * # SYNOPSIS
 *
 * A hello world for the terminal:
 *
 *     use std;
 *     use termbox;
 *
 *     import tb = termbox;
 *
 *     fn main() {
 *         tb::init();
 *         tb::print(1, 1, tb::bold, tb::white, tb::black, "Hello, world!");
 *         tb::present();
 *         std::timer::sleep(std::uv_global_loop::get(), 1000);
 *         tb::shutdown();
 *     }
 *
 * # DESCRIPTION
 *
 * Output is double-buffered.
 *
 * TODO
 *
 * # EXAMPLES
 *
 * TODO
 *
 */

// Exported functions
// export init, shutdown
//      , width, height
//      , clear, present
//      , set_cursor
//      , print, print_ch
//      , poll_event, peek_event
//      , event;

// Exported types
// export color, style
//      , event;



/*
 * The event type matches struct tb_event from termbox.h
 */
pub struct raw_event {
	etype: u8,
		       emod: u8,
		       key: u16,
		       ch: u32,
		       w: i32,
		       h: i32,
}



/*
 * Foreign functions from termbox.
 */
mod c {
	use libc::types::os::arch::c95::{ c_int, c_uint};

	#[link(name = "termbox")]
	extern {

		pub fn tb_init() -> c_int;
		pub fn tb_shutdown();

		pub fn tb_width() -> c_uint;
		pub fn tb_height() -> c_uint;

		pub fn tb_clear();
		pub fn tb_present();

		pub fn tb_set_cursor(cx: c_int, cy: c_int);

		pub fn tb_change_cell(x: c_uint, y: c_uint, ch: u32, fg: u16, bg: u16);

		pub fn tb_select_input_mode(mode: c_int) -> c_int;
		pub fn tb_set_clear_attributes(fg: u16, bg: u16);

		pub fn tb_peek_event(ev: *::raw_event, timeout: c_uint) -> c_int;
		pub fn tb_poll_event(ev: *::raw_event) -> c_int;
	}
}

fn test() {

}


pub fn init() -> int {
	unsafe { c::tb_init() as int }
}


pub fn shutdown() {
	unsafe { c::tb_shutdown(); }
}


pub fn width() -> uint {
	unsafe {
		return  c::tb_width() as uint;
	}
}


pub fn height() -> uint {
	unsafe {
		return  c::tb_height() as uint;
	}
}

/**
 * Clear buffer.
 */

pub fn clear() {
	unsafe {
		c::tb_clear();
	}
}

// /**
//  * Write buffer to terminal.
//  */

pub fn present() {
	unsafe {
		c::tb_present();
	}
}


pub fn set_cursor(cx: int, cy: int) {
	unsafe {
		c::tb_set_cursor(cx as c_int, cy as c_int);
	}
}


// low-level wrapper
pub fn change_cell(x: uint, y: uint, ch: u32, fg: u16, bg: u16) {
	unsafe {
		c::tb_change_cell(x as c_uint, y as c_uint, ch, fg, bg);
	}
}

/// Convert from enums to u16
pub fn convert_color(c: color) -> u16 {
	match c {
		black   => 0x00,
			red     => 0x01,
			green   => 0x02,
			yellow  => 0x03,
			blue    => 0x04,
			magenta => 0x05,
			cyan    => 0x06,
			white   => 0x07,
	}
}

pub fn convert_style(sty: style) -> u16 {
	match sty {
		normal         => 0x00,
			       bold           => 0x10,
			       underline      => 0x20,
			       bold_underline => 0x30,
	}
}

pub fn reverse_convert_key(k: u16) -> key {
	match k {
		65535 => f1,
		      65534 => f2,
		      65533 => f3,
		      65532 => f4,
		      65531 => f5,
		      65530 => f6,
		      65529 => f7,
		      65528 => f8,
		      65527 => f9,
		      65526 => f10,
		      65525 => f11,
		      65524 => f12,
		      65523 => insert,
		      65522 => delete,
		      65521 => home,
		      65520 => end,
		      65519 => pgup,
		      65518 => pgdn,
		      65517 => arrow_up,
		      65516 => arrow_down,
		      65515 => arrow_left,
		      65514 => arrow_right,
		      0 => ctrl_tilde,
		      //0 => ctrl_2,
		      1 => ctrl_a,
		      2 => ctrl_b,
		      3 => ctrl_c,
		      4 => ctrl_d,
		      5 => ctrl_e,
		      6 => ctrl_f,
		      7 => ctrl_g,
		      8 => backspace,
		      //8 => ctrl_h,
		      9 => tab,
		      //9 => ctrl_i,
		      10 => ctrl_j,
		      11 => ctrl_k,
		      12 => ctrl_l,
		      13 => enter,
		      //13 => ctrl_m,
		      14 => ctrl_n,
		      15 => ctrl_o,
		      16 => ctrl_p,
		      17 => ctrl_q,
		      18 => ctrl_r,
		      19 => ctrl_s,
		      20 => ctrl_t,
		      21 => ctrl_u,
		      22 => ctrl_v,
		      23 => ctrl_w,
		      24 => ctrl_x,
		      25 => ctrl_y,
		      26 => ctrl_z,
		      27 => esc,
		      //27 => ctrl_lsq_bracket,
		      //27 => ctrl_3,
		      28 => ctrl_4,
		      //28 => ctrl_backslash,
		      29 => ctrl_5,
		      //29 => ctrl_rsq_bracket,
		      30 => ctrl_6,
		      31 => ctrl_7,
		      //31 => ctrl_slash,
		      //31 => ctrl_underscore,
		      32 => space,
		      127 => backspace2,
		      //127 => ctrl_8
		      _ => fail!("invalid key")
	}
}
/**
 * Print a string to the buffer.  Leftmost charater is at (x, y).
 */

pub fn print(x: uint, y: uint, sty: style, fg: color, bg: color, s: &str) {
	let fg: u16 = convert_color(fg) | convert_style(sty);
	let bg: u16 = convert_color(bg);
	for (i, ch) in s.chars().enumerate() {
		unsafe {
			c::tb_change_cell((x + i) as c_uint, y as c_uint, ch as u32, fg, bg);
		}
	}
}

// /**
//  * Print a charater to the buffer.
//  */

pub fn print_ch(x: uint, y: uint, sty: style, fg: color, bg: color, ch: char) {
	unsafe {
		let fg: u16 = convert_color(fg) | convert_style(sty);
		let bg: u16 = convert_color(bg);
		c::tb_change_cell(x as c_uint, y as c_uint, ch as u32, fg, bg);
	}
}

pub enum key {
	f1 = 65535,
	   f2 = 65534,
	   f3 = 65533,
	   f4 = 65532,
	   f5 = 65531,
	   f6 = 65530,
	   f7 = 65529,
	   f8 = 65528,
	   f9 = 65527,
	   f10 = 65526,
	   f11 = 65525,
	   f12 = 65524,
	   insert = 65523,
	   delete = 65522,
	   home = 65521,
	   end = 65520,
	   pgup = 65519,
	   pgdn = 65518,
	   arrow_up = 65517,
	   arrow_down = 65516,
	   arrow_left = 65515,
	   arrow_right = 65514,
	   ctrl_tilde = 0,
	   // ctrl_2 = 0,
	   ctrl_a = 1,
	   ctrl_b = 2,
	   ctrl_c = 3,
	   ctrl_d = 4,
	   ctrl_e = 5,
	   ctrl_f = 6,
	   ctrl_g = 7,
	   backspace = 8,
	   //ctrl_h = 8,
	   tab = 9,
	   //ctrl_i = 9,
	   ctrl_j = 10,
	   ctrl_k = 11,
	   ctrl_l = 12,
	   enter = 13,
	   //ctrl_m = 13,
	   ctrl_n = 14,
	   ctrl_o = 15,
	   ctrl_p = 16,
	   ctrl_q = 17,
	   ctrl_r = 18,
	   ctrl_s = 19,
	   ctrl_t = 20,
	   ctrl_u = 21,
	   ctrl_v = 22,
	   ctrl_w = 23,
	   ctrl_x = 24,
	   ctrl_y = 25,
	   ctrl_z = 26,
	   esc = 27,
	   //ctrl_lsq_bracket = 27,
	   //ctrl_3 = 27,
	   ctrl_4 = 28,
	   //ctrl_backslash = 28,
	   ctrl_5 = 29,
	   //ctrl_rsq_bracket = 29,
	   ctrl_6 = 30,
	   ctrl_7 = 31,
	   //ctrl_slash = 31,
	   //ctrl_underscore = 31,
	   space = 32,
	   backspace2 = 127,
	   //ctrl_8 = 127
}

pub enum color {
	black,
		red,
		green,
		yellow,
		blue,
		magenta,
		cyan,
		white
}

pub enum style {
	normal,
		bold,
		underline,
		bold_underline
}

//Convenience functions
pub fn with_term(f: proc():Send) {
	init();
	let res = task::try(f);
	shutdown();
	match res {
		Err(_) => {
			error!("with_term: An error occured.");
		}
		_ => {}
	}
}

pub fn nil_raw_event() -> raw_event {
	raw_event{etype: 0, emod: 0, key: 0, ch: 0, w: 0, h: 0}
}

pub enum event {
	key_event(u8, key, u32),
		resize_event(i32, i32),
		no_event
}

/**
 * Get an event if within timeout milliseconds, otherwise return urn no_event.
 */


pub fn peek_event(timeout: uint) -> event {
	unsafe {
		let ev = nil_raw_event();
		let rc = c::tb_peek_event(&ev, timeout as c_uint);
		return unpack_event(rc, &ev);
	}
}

// /**
//  * Blocking function to return urn next event.
//  */

pub fn poll_event() -> event {
	unsafe {
		let ev = nil_raw_event();
		let rc = c::tb_poll_event(&ev);
		return unpack_event(rc, &ev);
	}
}

// /* helper pub fn
//  *
//  * ev_type
//  *   0 -> no event
//  *   1 -> key
//  *   2 -> resize
//  *   -1 -> error
//  */
pub fn unpack_event(ev_type: c_int, ev: &raw_event) -> event {
	match ev_type {
		0 => no_event,
		  1 => {
			  return key_event(ev.emod, reverse_convert_key(ev.key), ev.ch);
		  },
		  2 => {
			  return resize_event(ev.w, ev.h);
		  },
		  _ => { fail!("asdf"); }
	}
}
