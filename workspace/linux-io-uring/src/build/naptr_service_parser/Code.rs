// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct Code<GPTC: GenerateParseTreeCallback>
{
	writer: BufWriter<File>,
	stack_depth: usize,
	marker: PhantomData<GPTC>,
}

impl<GPTC: GenerateParseTreeCallback> Code<GPTC>
{
	fn new(out_dir: &OsString, function_name: &str) -> io::Result<Self>
	{
		Ok
		(
			Self
			{
				writer: new_buf_writer(out_dir, &format!("{}.naptr_service_parser.rs", function_name))?,
				stack_depth: 0,
				marker: PhantomData,
			}
		)
	}
	
	fn stack_depth(&self) -> usize
	{
		self.stack_depth
	}
	
	fn increment_stack_depth(&mut self)
	{
		self.stack_depth += 1
	}
	
	fn decrement_stack_depth(&mut self)
	{
		self.stack_depth -= 1
	}
	
	fn push_function_start(&mut self) -> io::Result<()>
	{
		GPTC::push_function_start(self)
	}
	
	fn push_function_end(self) -> io::Result<()>
	{
		GPTC::push_function_end(self)
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
		self.push_str(value)
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
