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

	/// Reads a value from a file which is line-feed terminated and is hexadecimal using a parser.
	fn read_hexadecimal_value_with_prefix<T: Num>(&self, size: usize) -> io::Result<T>;

	/// Reads a file as bytes.
	///
	/// Fails if empty.
	fn read_raw(&self) -> io::Result<Box<[u8]>>;

	/// Reads a file as bytes, expecting a final line feed.
	///
	/// The returned bytes lack a final line feed.
	fn read_raw_without_line_feed(&self) -> io::Result<Box<[u8]>>;

	/// Reads a file as a string.
	///
	/// Fails if empty.
	fn read_raw_string(&self) -> io::Result<String>;

	/// Reads a file as a string, expecting a final line feed.
	///
	/// The returned string lacks a final line feed.
	fn read_string_without_line_feed(&self) -> io::Result<String>;

	/// Reads a value from a file which is line-feed terminated.
	fn read_value<F>(&self) -> io::Result<F> where F: FromStr, <F as FromStr>::Err: 'static + Send + Sync + error::Error;

	/// Reads a 0 or 1 bool from a file which is line-feed terminated.
	fn read_zero_or_one_bool(&self) -> io::Result<bool>;

	/// Writes a value to a file which is line-feed terminated.
	fn write_value<'a>(&self, value: impl IntoLineFeedTerminatedByteString<'a>) -> io::Result<()>;

	/// Opens a file for writing.
	fn open_file_for_writing(&self) -> io::Result<File>;

	/// Reads and parses a linux core or numa list string from a file.
	///
	/// Returns a set with the zero-based indices found in the string.
	/// For example, "2,4-31,32-63" would return a set with all values between 0 to 63 except 0, 1 and 3.
	fn read_hyper_thread_or_numa_node_list<BSA: BitSetAware>(&self) -> Result<BitSet<BSA>, io::Error>;

	/// Reads and parses a HyperThread or NumaNode bit set from a file.
	///
	/// A bit set might look like:-
	///
	/// `ffffffff` for CPUs 0 through 31.
	/// `00000000,00000001` for CPU 0 only.
	/// `ffffffff,ffffffff,ffffffff,ffffffff` for CPUs 0 through 127.
	///
	/// Elements are 32-bit words.
	fn parse_hyper_thread_or_numa_node_bit_set<BSA: BitSetAware>(&self) -> Result<BitSet<BSA>, io::Error>;

	/// HyperThread or NumaNode present in a folder path.
	///
	/// This will return `None` if the path `self` does not exist.
	fn entries_in_folder_path<BSA: BitSetAware>(&self) -> Result<Option<BitSet<BSA>>, io::Error>;

	/// Memory map a file.
	fn memory_map<'a>(&self) -> Result<MemoryMappedFile, io::Error>;
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
	fn read_hexadecimal_value_with_prefix<T: Num>(&self, size: usize) -> io::Result<T>
	{
		use self::ErrorKind::InvalidData;

		let raw_string = self.read_string_without_line_feed()?;

		// '0x' eg '0x1af4'.
		let size_wih_0x_prefix = 2 + size;
		if unlikely!(raw_string.len() != size_wih_0x_prefix)
		{
			return Err(io::Error::new(InvalidData, format!("{} bytes not read", size_wih_0x_prefix)));
		}

		match &raw_string[..2]
		{
			"0x" => (),
			_ => return Err(io::Error::new(InvalidData, "value does not start '0x'")),
		}

		T::from_str_radix(&raw_string[2..], 16).map_err(|_| io::Error::new(InvalidData, "Could not parse hexadecimal value"))
	}

	#[inline(always)]
	fn read_raw(&self) -> io::Result<Box<[u8]>>
	{
		let raw = std::fs::read(self)?.into_boxed_slice();

		if unlikely!(raw.is_empty())
		{
			Err(io::Error::new(ErrorKind::InvalidData, "Empty file"))
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
			return Err(io::Error::new(ErrorKind::InvalidData, "File lacks terminating line feed"));
		}
		Ok(raw.into_boxed_slice())
	}

	#[inline(always)]
	fn read_raw_string(&self) -> io::Result<String>
	{
		let raw_string = read_to_string(self)?;

		if unlikely!(raw_string.is_empty())
		{
			Err(io::Error::new(ErrorKind::InvalidData, "Empty file"))
		}
		else
		{
			Ok(raw_string)
		}
	}

	#[inline(always)]
	fn read_string_without_line_feed(&self) -> io::Result<String>
	{
		let mut raw_string = self.read_raw_string()?;
		let length = raw_string.len();
		let should_be_line_feed = raw_string.remove(length - 1);
		if unlikely!(should_be_line_feed != '\n')
		{
			return Err(io::Error::new(ErrorKind::InvalidData, "File lacks terminating line feed"));
		}
		Ok(raw_string)
	}

	#[inline(always)]
	fn read_value<F>(&self) -> io::Result<F> where F: FromStr, <F as FromStr>::Err: 'static + Send + Sync + error::Error
	{
		let string = self.read_string_without_line_feed()?;

		match string.parse::<F>()
		{
			Err(error) => Err(io::Error::new(ErrorKind::InvalidData, error)),
			Ok(value) => Ok(value),
		}
	}

	#[inline(always)]
	fn read_zero_or_one_bool(&self) -> io::Result<bool>
	{
		let bytes = self.read_raw_without_line_feed()?;

		if unlikely!(bytes.len() != 1)
		{
			return Err(io::Error::new(ErrorKind::InvalidData, "bool is not one byte long"));
		}

		match bytes[0]
		{
			b'0' => Ok(false),
			b'1' => Ok(true),
			_ => Err(io::Error::new(ErrorKind::InvalidData, "bool is not 0 or 1")),
		}
	}

	#[inline(always)]
	fn write_value<'a>(&self, value: impl IntoLineFeedTerminatedByteString<'a>) -> io::Result<()>
	{
		let mut file = self.open_file_for_writing()?;
		file.write_all(&value.into_line_feed_terminated_byte_string())
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

		BitSet::<BSA>::parse_linux_list_string(&without_line_feed).map_err(|error| io::Error::new(ErrorKind::Other, error))
	}

	#[inline(always)]
	fn parse_hyper_thread_or_numa_node_bit_set<BSA: BitSetAware>(&self) -> Result<BitSet<BSA>, io::Error>
	{
		let without_line_feed = self.read_raw_without_line_feed()?;
		Ok(BitSet::parse_hyper_thread_or_numa_node_bit_set(&without_line_feed))
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
	fn memory_map<'a>(&self) -> Result<MemoryMappedFile, io::Error>
	{
		let file = OpenOptions::new().read(true).write(true).open(self)?;

		let size =
		{
			let metadata = file.metadata()?;
			if !metadata.is_file()
			{
				return Err(io::Error::from(ErrorKind::Other))
			}
			metadata.len() as usize
		};

		let result = unsafe { mmap(null_mut(), size, PROT_READ | PROT_WRITE, MAP_SHARED, file.as_raw_fd(), 0) };
		if unlikely!(result == MAP_FAILED)
		{
			return Err(io::Error::last_os_error())
		}

		Ok
		(
			MemoryMappedFile
			{
				pointer: unsafe { NonNull::new_unchecked(result as *mut u8) },
				size,
			}
		)
	}
}
