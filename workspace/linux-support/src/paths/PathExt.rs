// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An extension trait to make it easier to work with sys and proc files and folders.
pub trait PathExt
{
	/// Converts a `Path` to a `CString`.
	fn to_c_string(&self) -> CString;

	/// Makes a file read-write to all.
	fn make_file_read_write_all(&self) -> io::Result<()>;

	/// Makes a folder searchable to all (ie gives it read and execute permissions).
	fn make_folder_searchable_to_all(&self) -> io::Result<()>;

	/// Reads a file as bytes.
	///
	/// Fails if empty.
	fn read_raw(&self) -> io::Result<Box<[u8]>>;

	/// Reads a file as bytes, expecting a final line feed.
	///
	/// The returned bytes lack a final line feed.
	fn read_raw_without_line_feed(&self) -> io::Result<Box<[u8]>>;
	
	/// Reads a value from a file which is line-feed terminated if the file exists.
	fn read_value_if_exists<F>(&self) -> io::Result<Option<F>> where F: FromBytes, <F as FromBytes>::Error: 'static + Send + Sync + error::Error;

	/// Reads a value from a file which is line-feed terminated.
	fn read_value<F>(&self) -> io::Result<F> where F: FromBytes, <F as FromBytes>::Error: 'static + Send + Sync + error::Error;

	/// Reads a 0 or 1 bool from a file which is line-feed terminated.
	fn read_zero_or_one_bool(&self) -> io::Result<bool>;

	/// Writes a value to a file which is line-feed terminated.
	fn write_value<'a>(&self, value: impl IntoLineFeedTerminatedByteString<'a>) -> io::Result<()>;
	
	/// Writes a value to a file which is then line-feed terminated.
	fn write_value_then_line_feed(&self, value: &[u8]) -> io::Result<()>;
	
	/// Opens a file for writing.
	fn open_file_for_writing(&self) -> io::Result<File>;

	/// Reads and parses a linux core or numa list string from a file.
	///
	/// Handles empty files.
	///
	/// Returns a set with the zero-based indices found in the string.
	/// For example, "2,4-31,32-63" would return a set with all values between 0 to 63 except 0, 1 and 3.
	fn read_hyper_thread_or_numa_node_list<BSA: BitSetAware>(&self) -> Result<BitSet<BSA>, io::Error>;
	
	/// Reads and parses a linux core or numa list string from a file if it exists.
	///
	/// Handles empty files.
	///
	/// Returns a set with the zero-based indices found in the string.
	/// For example, "2,4-31,32-63" would return a set with all values between 0 to 63 except 0, 1 and 3.
	fn read_hyper_thread_or_numa_node_list_if_exists<BSA: BitSetAware>(&self) -> Result<Option<BitSet<BSA>>, io::Error>;

	/// Reads and parses a HyperThread or NumaNode bit set from a file.
	///
	/// A bit set might look like:-
	///
	/// `ffffffff` for CPUs 0 through 31.
	/// `00000000,00000001` for CPU 0 only.
	/// `ffffffff,ffffffff,ffffffff,ffffffff` for CPUs 0 through 127.
	///
	/// Elements are 32-bit words.
	fn parse_comma_separated_bit_set<BSA: BitSetAware>(&self) -> Result<BitSet<BSA>, io::Error>;

	/// HyperThread or NumaNode present in a folder path.
	///
	/// This will return `None` if the path `self` does not exist.
	fn entries_in_folder_path<BSA: BitSetAware>(&self) -> Result<Option<BitSet<BSA>>, io::Error>;

	/// Memory map a file read-write.
	fn memory_map_read_write<'a>(&self, offset: u64, address_hint: AddressHint, sharing: Sharing, huge_memory_page_size: Option<Option<HugePageSize>>, prefault: bool, reserve_swap_space: bool, defaults: &DefaultPageSizeAndHugePageSizes) -> io::Result<MappedMemory>;
}

impl PathExt for Path
{
	#[inline(always)]
	fn to_c_string(&self) -> CString
	{
		CString::new(self.as_os_str().as_bytes()).expect("Paths should not contain interior ASCII NULs")
	}

	#[inline(always)]
	fn make_file_read_write_all(&self) -> io::Result<()>
	{
		#[inline(always)]
		fn add_read_write_permissions(permissions: Permissions) -> Permissions
		{
			Permissions::from_mode(permissions.mode() | 0o666)
		}
		let metadata = metadata(self)?;
		set_permissions(self, add_read_write_permissions(metadata.permissions()))
	}

	#[inline(always)]
	fn make_folder_searchable_to_all(&self) -> io::Result<()>
	{
		#[inline(always)]
		fn add_read_and_execute_permissions(permissions: Permissions) -> Permissions
		{
			Permissions::from_mode(permissions.mode() | 0o555)
		}
		let metadata = metadata(self)?;
		set_permissions(self, add_read_and_execute_permissions(metadata.permissions()))
	}

	#[inline(always)]
	fn read_raw(&self) -> io::Result<Box<[u8]>>
	{
		let raw = std::fs::read(self)?.into_boxed_slice();

		if unlikely!(raw.is_empty())
		{
			Err(io_error_invalid_data("Empty file"))
		}
		else
		{
			Ok(raw)
		}
	}

	#[inline(always)]
	fn read_raw_without_line_feed(&self) -> io::Result<Box<[u8]>>
	{
		let mut raw = self.read_raw()?.to_vec();
		let length = raw.len();
		let should_be_line_feed = raw.remove(length - 1);
		if unlikely!(should_be_line_feed != b'\n')
		{
			return Err(io_error_invalid_data("File lacks terminating line feed"));
		}
		Ok(raw.into_boxed_slice())
	}

	#[inline(always)]
	fn read_value<F>(&self) -> io::Result<F> where F: FromBytes, <F as FromBytes>::Error: 'static + Send + Sync + error::Error
	{
		let bytes = self.read_raw_without_line_feed()?;

		match F::from_bytes(&bytes)
		{
			Err(error) => Err(io_error_invalid_data(error)),
			Ok(value) => Ok(value),
		}
	}

	#[inline(always)]
	fn read_value_if_exists<F>(&self) -> io::Result<Option<F>> where F: FromBytes, <F as FromBytes>::Error: 'static + Send + Sync + error::Error
	{
		let bytes = match self.read_raw_without_line_feed()
		{
			Ok(bytes) => bytes,
			Err(error) => return if error.kind() == ErrorKind::NotFound
			{
				Ok(None)
			}
			else
			{
				Err(error)
			}
		};

		match F::from_bytes(&bytes)
		{
			Err(error) => Err(io_error_invalid_data(error)),
			Ok(value) => Ok(Some(value)),
		}
	}

	#[inline(always)]
	fn read_zero_or_one_bool(&self) -> io::Result<bool>
	{
		let bytes = self.read_raw_without_line_feed()?;

		if unlikely!(bytes.len() != 1)
		{
			return Err(io_error_invalid_data("bool is not one byte long"));
		}

		match bytes[0]
		{
			b'0' => Ok(false),
			b'1' => Ok(true),
			_ => Err(io_error_invalid_data("bool is not 0 or 1")),
		}
	}

	#[inline(always)]
	fn write_value<'a>(&self, value: impl IntoLineFeedTerminatedByteString<'a>) -> io::Result<()>
	{
		let mut file = self.open_file_for_writing()?;
		file.write_all(&value.into_line_feed_terminated_byte_string())
	}

	#[inline(always)]
	fn write_value_then_line_feed(&self, value: &[u8]) -> io::Result<()>
	{
		let mut file = self.open_file_for_writing()?;
		file.write_all(value)?;
		file.write_all(&[b'\n'])
	}

	#[inline(always)]
	fn open_file_for_writing(&self) -> io::Result<File>
	{
		OpenOptions::new().write(true).open(self)
	}

	#[inline(always)]
	fn read_hyper_thread_or_numa_node_list<BSA: BitSetAware>(&self) -> Result<BitSet<BSA>, io::Error>
	{
		let without_line_feed = self.read_raw_without_line_feed()?;

		BitSet::<BSA>::parse_linux_list_string(&without_line_feed).map_err(io_error_invalid_data)
	}
	
	#[inline(always)]
	fn read_hyper_thread_or_numa_node_list_if_exists<BSA: BitSetAware>(&self) -> Result<Option<BitSet<BSA>>, io::Error>
	{
		let without_line_feed = match self.read_raw_without_line_feed()
		{
			Ok(without_line_feed) => without_line_feed,
			
			Err(error) => return if error.kind() == ErrorKind::NotFound
			{
				Ok(None)
			}
			else
			{
				Err(error)
			}
		};
		
		match BitSet::<BSA>::parse_linux_list_string(&without_line_feed)
		{
			Err(error) => Err(io_error_invalid_data(error)),
			Ok(bit_set) => Ok(Some(bit_set)),
		}
	}

	#[inline(always)]
	fn parse_comma_separated_bit_set<BSA: BitSetAware>(&self) -> Result<BitSet<BSA>, io::Error>
	{
		let without_line_feed = self.read_raw_without_line_feed()?;
		Ok(BitSet::parse_comma_separated_bit_set(&without_line_feed))
	}

	fn entries_in_folder_path<BSA: BitSetAware>(&self) -> Result<Option<BitSet<BSA>>, io::Error>
	{
		if !self.exists()
		{
			return Ok(None)
		}

		let mut bit_set = BitSet::<BSA>::new();
		for entry in self.read_dir()?
		{
			let entry = entry?;
			let file_type = entry.file_type()?;
			if file_type.is_dir() || file_type.is_symlink()
			{
				if let Some(bsa) = BSA::parse_file_name(&entry.file_name())?
				{
					bit_set.add(bsa)
				}
			}
		}

		Ok(Some(bit_set))
	}

	#[inline(always)]
	fn memory_map_read_write<'a>(&self, offset: u64, address_hint: AddressHint, sharing: Sharing, huge_memory_page_size: Option<Option<HugePageSize>>, prefault: bool, reserve_swap_space: bool, defaults: &DefaultPageSizeAndHugePageSizes) -> io::Result<MappedMemory>
	{
		const protection: Protection = Protection::ReadWrite;
		let file = protection.adjust_open_options_to_match(&mut OpenOptions::new()).open(self)?;

		let length =
		{
			let metadata = file.metadata()?;
			if !metadata.is_file()
			{
				return Err(io_error_other("Not a file"))
			}
			metadata.len()
		};
		if length == 0
		{
			return Err(io_error_other("Empty files can not be memory-mapped"))
		}

		MappedMemory::from_file(&file, offset, new_non_zero_u64(length), address_hint, protection, sharing, huge_memory_page_size, prefault, reserve_swap_space, defaults).map_err(io_error_other)
	}
}
