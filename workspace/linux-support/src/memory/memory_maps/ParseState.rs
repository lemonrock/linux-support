// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug)]
struct ParseState
{
	zero_based_line_number: usize,
	zero_based_field_index: usize,
	heap: bool,
	stack: bool,
	vdso: bool,
	vvar: bool,
	minimum_next_from: VirtualAddress,
}

impl ParseState
{
	#[inline(always)]
	fn new_line(&mut self, zero_based_line_number: usize)
	{
		self.zero_based_line_number = zero_based_line_number;
		self.zero_based_field_index = 0;
	}

	#[inline(always)]
	fn map_line_split_fields(map_line: &[u8]) -> impl Iterator<Item=&[u8]>
	{
		let mut index = 0;
		map_line.splitn(8, move |byte|
		{
			let separator = match index
			{
				0 => b'-',
				4 => b':',
				_ => b' ',
			};
			if *byte == separator
			{
				index += 1;
				true
			}
			else
			{
				false
			}
		})
	}

	#[inline(always)]
	fn next_number_field<'a, P: 'a>(&mut self, fields: &mut impl Iterator<Item=&'a [u8]>, name: &'static str, parser: impl FnOnce(&'a [u8]) -> Result<P, ParseNumberError>) -> Result<P, MemoryMapParseError>
	{
		self.next_field(fields, name, |field_bytes, zero_based_line_number, zero_based_field_index| parser(field_bytes).map_err(|cause| CouldNotParseNumberField { zero_based_line_number, zero_based_field_index, name, cause }))
	}

	#[inline(always)]
	fn next_field<'a, P: 'a>(&mut self, fields: &mut impl Iterator<Item=&'a [u8]>, name: &'static str, parser: impl FnOnce(&'a [u8], usize, usize) -> Result<P, MemoryMapParseError>) -> Result<P, MemoryMapParseError>
	{
		let result = self.field(fields, name, parser);
		self.zero_based_field_index += 1;
		result
	}

	#[inline(always)]
	fn last_field<'a, P: 'a>(&mut self, mut fields: impl Iterator<Item=&'a [u8]>, name: &'static str, parser: impl FnOnce(&'a [u8], usize, usize) -> Result<P, MemoryMapParseError>) -> Result<P, MemoryMapParseError>
	{
		let result = self.field(&mut fields, name, parser);
		debug_assert!(fields.next().is_none());
		result
	}

	#[inline(always)]
	fn field<'a, P: 'a>(&mut self, fields: &mut impl Iterator<Item=&'a [u8]>, name: &'static str, parser: impl FnOnce(&'a [u8], usize, usize) -> Result<P, MemoryMapParseError>) -> Result<P, MemoryMapParseError>
	{
		let field_bytes = fields.next().ok_or(self.missing_required_field(name))?;
		parser(field_bytes, self.zero_based_line_number, self.zero_based_field_index)
	}

	#[inline(always)]
	fn validate_from_and_to(&mut self, from: VirtualAddress, to: VirtualAddress) -> Result<Range<VirtualAddress>, MemoryMapParseError>
	{
		if unlikely!(to <= from)
		{
			return Err(FromAndToAreInvalid { zero_based_line_number: self.zero_based_line_number, from, to })
		}

		if unlikely!(self.minimum_next_from > from)
		{
			return Err(FromTooSmall { zero_based_line_number: self.zero_based_line_number, from, to })
		}

		self.minimum_next_from = to;

		Ok(from .. to)
	}

	#[inline(always)]
	fn parse_kind(&mut self, field_bytes: &[u8], offset: u32, block_device: BlockDevice, inode: Inode, protection: Protection, sharing: Sharing) -> Result<MemoryMapEntryKind, MemoryMapParseError>
	{
		const SpecialFileNameFirstByte: u8 = b'[';
		const FilePathFirstByte: u8 = b'/';

		if field_bytes.is_empty()
		{
			return self.validate_anonymous(offset, block_device, inode)
		}

		let (name, name_first_byte) =
		{
			let name_starts_at = memchr2(SpecialFileNameFirstByte, FilePathFirstByte, field_bytes).ok_or(self.missing_required_field("file_name"))?;
			(&field_bytes[name_starts_at.. ], unsafe { *field_bytes.get_unchecked(name_starts_at) })
		};

		match name_first_byte
		{
			SpecialFileNameFirstByte => self.validate_special_file_name(name, offset, block_device, inode, protection, sharing),

			FilePathFirstByte => self.validate_file(name, offset, block_device, inode),

			_ => unreachable_code(format_args!("")),
		}
	}

	#[inline(always)]
	fn validate_anonymous(&self, offset: u32, block_device: BlockDevice, inode: Inode) -> Result<MemoryMapEntryKind, MemoryMapParseError>
	{
		if unlikely!(offset != 0)
		{
			return Err(OffsetWasNotZeroForAnonymous { zero_based_line_number: self.zero_based_line_number, offset })
		}
		if unlikely!(block_device.is_not_zero_zero())
		{
			return Err(BlockDeviceWasNotZeroZeroForAnonymous { zero_based_line_number: self.zero_based_line_number, block_device })
		}
		if unlikely!(inode != Inode::Zero)
		{
			return Err(InodeWasNotZeroForAnonymous { zero_based_line_number: self.zero_based_line_number, inode })
		}

		Ok(MemoryMapEntryKind::Anonymous { page_counts: None })
	}

	#[inline(always)]
	fn validate_special_file_name(&mut self, special_file_name: &[u8], offset: u32, block_device: BlockDevice, inode: Inode, protection: Protection, sharing: Sharing) -> Result<MemoryMapEntryKind, MemoryMapParseError>
	{
		use self::MemoryMapEntryKindSpecial::*;
		use self::Protection::*;

		let (present, special_file_name, expected_protection) = match special_file_name
		{
			b"[heap]" => (&mut self.heap, Heap { page_counts: None }, ReadWrite),
			b"[stack]" => (&mut self.stack, Stack { page_counts: None }, ReadWrite),
			b"[vdso]" => (&mut self.vdso, vDSO, Read),
			b"[vvar]" => (&mut self.vvar, VVAR, ReadExecutable),

			_ => return Err(UnknownSpecialFileName { zero_based_line_number: self.zero_based_line_number, special_file_name: special_file_name.to_vec() }),
		};

		if unlikely!(*present)
		{
			return Err(RepeatedSpecialFileName { zero_based_line_number: self.zero_based_line_number, special_file_name })
		};

		*present = true;

		if unlikely!(offset != 0)
		{
			return Err(OffsetWasNotZeroForSpecialFileName { zero_based_line_number: self.zero_based_line_number, special_file_name, offset })
		}
		if unlikely!(block_device.is_not_zero_zero())
		{
			return Err(BlockDeviceWasNotZeroZeroForSpecialFileName { zero_based_line_number: self.zero_based_line_number, special_file_name, block_device })
		}
		if unlikely!(inode != Inode::Zero)
		{
			return Err(InodeWasNotZeroForSpecialFileName { zero_based_line_number: self.zero_based_line_number, special_file_name, inode })
		}
		if unlikely!(protection != expected_protection)
		{
			return Err(ProtectionWasNotExpectedForSpecialFileName { zero_based_line_number: self.zero_based_line_number, special_file_name, protection, expected_protection })
		}
		if unlikely!(sharing != Sharing::Private)
		{
			return Err(SharingWasNotPrivateForSpecialFileName { zero_based_line_number: self.zero_based_line_number, special_file_name, sharing })
		}

		Ok(MemoryMapEntryKind::Special(special_file_name))
	}

	#[inline(always)]
	fn validate_file(&self, new_line_escaped_file_path_which_may_be_deleted: &[u8], offset: u32, block_device: BlockDevice, inode: Inode) -> Result<MemoryMapEntryKind, MemoryMapParseError>
	{
		let (new_line_escaped_file_path, deleted) = without_suffix(new_line_escaped_file_path_which_may_be_deleted, b" (deleted)");
		let unescaped_file_path_bytes = Self::unescape_file_path(new_line_escaped_file_path.to_vec());

		Ok
		(
			MemoryMapEntryKind::File
			{
				offset,
				block_device,
				inode,
				file_path: PathBuf::from(OsString::from_vec(unescaped_file_path_bytes)),
				deleted,
				page_counts: None,
			}
		)
	}

	#[inline(always)]
	fn unescape_file_path(path_with_escaped_new_lines: Vec<u8>) -> Vec<u8>
	{
		LinuxStringEscapeSequence::unescape_linux_string(path_with_escaped_new_lines, &[LinuxStringEscapeSequence::LineFeed])
	}

	#[inline(always)]
	const fn missing_required_field(&self, name: &'static str) -> MemoryMapParseError
	{
		MissingRequiredField { zero_based_line_number: self.zero_based_line_number, zero_based_field_index: self.zero_based_field_index, name }
	}

	#[inline(always)]
	fn validate(&self) -> Result<(), MemoryMapParseError>
	{
		if unlikely!(!self.stack)
		{
			return Err(MissingStackMapping)
		}

		if unlikely!(!self.vdso)
		{
			return Err(MissingVdsoMapping)
		}

		if unlikely!(!self.vvar)
		{
			return Err(MissingVvarMapping)
		}

		Ok(())
	}
}
