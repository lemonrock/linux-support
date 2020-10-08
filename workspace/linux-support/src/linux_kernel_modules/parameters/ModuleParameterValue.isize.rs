// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


impl ModuleParameterValue for isize
{
	#[inline(always)]
	fn parse_bytes(bytes: Box<[u8]>) -> io::Result<Self>
	{
		isize::from_bytes(&bytes[..]).map_err(io_error_invalid_data)
	}
	
	#[inline(always)]
	fn write_value(&self, extant_parameter_file_path: PathBuf) -> io::Result<()>
	{
		use self::InverseBooleanModuleParameterValue::*;
		
		extant_parameter_file_path.write_value(UnpaddedDecimalInteger(*self))
	}
}
