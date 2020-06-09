// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.




#![feature(fmt_internals)]




use std::fmt::ArgumentV1;
use std::fmt::rt::v1::Alignment;
use std::fmt::rt::v1::Argument;
use std::fmt::rt::v1::Count;





pub fn write(output: &mut dyn std::fmt::Write, args: Arguments<'_>) -> fmt::Result {
	let mut formatter = Formatter {
		flags: 0,
		width: None,
		precision: None,
		buf: output,
		align: Alignment::Unknown,
		fill: ' ',
	};
	
	let mut idx = 0;
	
	match args.fmt {
		None => {
			// We can use default formatting parameters for all arguments.
			for (arg, piece) in args.args.iter().zip(args.pieces.iter()) {
				formatter.buf.write_str(*piece)?;
				(arg.formatter)(arg.value, &mut formatter)?;
				idx += 1;
			}
		}
		Some(fmt) => {
			// Every spec has a corresponding argument that is preceded by
			// a string piece.
			for (arg, piece) in fmt.iter().zip(args.pieces.iter()) {
				formatter.buf.write_str(*piece)?;
				run(&mut formatter, arg, &args.args)?;
				idx += 1;
			}
		}
	}
	
	// There can be only one trailing string piece left.
	if let Some(piece) = args.pieces.get(idx) {
		formatter.buf.write_str(*piece)?;
	}
	
	Ok(())
}

fn run(fmt: &mut Formatter<'_>, arg: &Argument, args: &[ArgumentV1<'_>]) -> fmt::Result {
	fmt.fill = arg.format.fill;
	fmt.align = arg.format.align;
	fmt.flags = arg.format.flags;
	fmt.width = getcount(args, &arg.format.width);
	fmt.precision = getcount(args, &arg.format.precision);
	
	// Extract the correct argument
	let value = args[arg.position];
	
	// Then actually do some printing
	(value.formatter)(value.value, fmt)
}

fn getcount(args: &[ArgumentV1<'_>], cnt: &Count) -> Option<usize> {
	match *cnt {
		Count::Is(n) => Some(n),
		Count::Implied => None,
		Count::Param(i) => args[i].as_usize(),
	}
}
