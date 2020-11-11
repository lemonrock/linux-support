// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct Code
{
	writer: BufWriter<File>,
}

impl Code
{
	fn push_function_start(mut self) -> io::Result<()>
	{
		self.push_line("fn naptr_service_field_parse(services_field: &[u8]) -> Result<Either<IgnoreServiceFieldReason>, ServiceFieldParseError>")?;
		self.push_line("{")?;
		self.push_function_line("use self::IgnoreServiceFieldReason::*;")?;
		self.push_function_line("use self::ServiceField::*;")?;
		self.push_function_line("use self::ServiceFieldParseError::*;")?;
		self.push_function_line("")?;
		self.push_function_line("let length = services_field.len();")?;
		self.push_function_line("")?;
		
		Ok(())
	}
	
	fn push_function_end(mut self) -> io::Result<()>
	{
		self.push_line("}")
	}
	
	fn push_function_line(&mut self, value: &str) -> io::Result<()>
	{
		self.push_tab()?;
		self.push_line(value)
	}
	
	fn push_line(&mut self, value: &str) -> io::Result<()>
	{
		self.push_str(value)?;
		self.push_new_line()
	}
	
	fn push_tab_indented_line(&mut self, value: &str) -> io::Result<()>
	{
		self.push_tab_indented(value)?;
		self.push_new_line()
	}
	
	fn push_tab_indented(&mut self, value: &str) -> io::Result<()>
	{
		self.push_tabs()?;
		self.push_str(value);
	}
	
	fn push_new_line(&mut self) -> io::Result<()>
	{
		self.push_char('\n')
	}
	
	fn push_tabs(&mut self)-> io::Result<()>
	{
		self.push_tab()?;
		for _ in 0 .. self.stack_depth
		{
			self.push_tab()?;
		}
		Ok(())
	}
	
	fn push_tab(&mut self) -> io::Result<()>
	{
		self.push_char('\t')
	}
	
	fn push_byte(&mut self, byte: u8) -> io::Result<()>
	{
		self.writer.write_all(&[byte])
	}
	
	#[allow(deprecated)]
	fn push_char(&mut self, value: char) -> io::Result<()>
	{
		let mut buffer: [u8; 4] = unsafe { uninitialized() };
		let slice = value.encode_utf8(&mut buffer);
		self.push_str(slice)
	}
	
	fn push_str(&mut self, value: &str) -> io::Result<()>
	{
		self.writer.write_all(value.as_bytes())
	}
}
