// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


pub(crate) trait ExtendFromSlice: AsRef<[u8]>
{
	#[inline(always)]
	fn parse_header_line<R: BufRead + ?Sized, Value, Error>(&mut self, buf_read: &mut R, field_name_with_colon_and_tab: &[u8], parser: impl FnOnce(&str) -> Result<Value, Error>) -> io::Result<Value>
	{
		self.read_until_line_feed(buf_read)?;

		let token: &[u8] = field_name_with_colon_and_tab;
		if unlikely!(self.length() < token.len())
		{
			return Err(invalid_data())
		}

		if unlikely!(&(self.as_ref())[0 .. token.len()] != token)
		{
			return Err(invalid_data())
		}

		let value =
		{
			let raw_value = &(self.as_ref())[token.len()..];
			let raw_value_utf8 = from_utf8(raw_value).map_err(|_utf8_error| invalid_data())?;
			parser(raw_value_utf8).map_err(|_parse_error| invalid_data())?
		};

		self.empty();

		Ok(value)
	}

	#[inline(always)]
	fn read_until_line_feed<R: BufRead + ?Sized>(&mut self, buf_read: &mut R) -> io::Result<usize>
	{
		self.read_until_delimiter_populating_buffer_with_bytes_read_excluding_delimiter(buf_read, b'\n')
	}

	fn read_until_delimiter_populating_buffer_with_bytes_read_excluding_delimiter<R: BufRead + ?Sized>(&mut self, buf_read: &mut R, delimiter: u8) -> io::Result<usize>;

	fn extend_from_slice(&mut self, slice: &[u8]) -> io::Result<()>;

	fn length(&self) -> usize;

	fn empty(&mut self);
}

impl<const CAP: usize> ExtendFromSlice for ArrayVec<u8, CAP>
{
	#[inline(always)]
	fn read_until_delimiter_populating_buffer_with_bytes_read_excluding_delimiter<R: BufRead + ?Sized>(&mut self, buf_read: &mut R, delimiter: u8) -> io::Result<usize>
	{
		let mut total_bytes_read = 0;
		loop
		{
			let (used, is_delimited) =
			{
				let available_slice = match buf_read.fill_buf()
				{
					Ok(available_slice) => available_slice,

					Err(error) => if error.kind() == ErrorKind::Interrupted
					{
						continue
					}
					else
					{
						return Err(error)
					},
				};

				let delimiter_index = available_slice.memchr(delimiter);

				let index = delimiter_index.unwrap_or(available_slice.len());
				self.extend_from_slice(&available_slice[0 .. index])?;

				(index, delimiter_index.is_some())
			};

			if used != 0
			{
				buf_read.consume(used);
				total_bytes_read += used;
			}

			if is_delimited || used == 0
			{
				return Ok(total_bytes_read)
			}
		}
	}

	#[inline(always)]
	fn extend_from_slice(&mut self, slice: &[u8]) -> io::Result<()>
	{
		let original_length = self.len();
		let slice_length = slice.len();
		let new_length = original_length + slice_length;

		if new_length > self.capacity()
		{
			return Err(invalid_data())
		}

		let pointer = self.as_mut_slice().as_mut_ptr();
		unsafe
		{
			pointer.add(original_length).copy_from_nonoverlapping(slice.as_ptr(), slice_length);
			self.set_len(new_length)
		}

		Ok(())
	}

	#[inline(always)]
	fn length(&self) -> usize
	{
		self.len()
	}

	#[inline(always)]
	fn empty(&mut self)
	{
		self.clear()
	}
}
