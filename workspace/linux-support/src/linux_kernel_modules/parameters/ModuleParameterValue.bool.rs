// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


impl ModuleParameterValue for bool
{
	#[inline(always)]
	fn parse_bytes(bytes: Box<[u8]>) -> io::Result<Self>
	{
		match &bytes[..]
		{
			b"Y" => Ok(true),
			
			b"N" => Ok(false),
			
			_ => Err(io_error_invalid_data(ParseNumberError::OutOfRange))
		}
	}
	
	#[inline(always)]
	fn write_value(&self, extant_parameter_file_path: PathBuf) -> io::Result<()>
	{
		let value = if *self
		{
			b"Y\n"
		}
		else
		{
			b"N\n"
		};
		extant_parameter_file_path.write_value(value)
	}
}
