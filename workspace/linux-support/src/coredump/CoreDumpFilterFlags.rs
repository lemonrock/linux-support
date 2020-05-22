// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// What to dump when core dumping.
	///
	/// Some things, like vDSO pages, are dumped regardless of the settings here.
	#[derive(Deserialize, Serialize)]
	pub struct CoreDumpFilterFlags: u32
	{
		#[allow(missing_docs)]
		const DumpAnonymousPrivateMappings = 1 << 0;
		
		#[allow(missing_docs)]
		const DumpAnonymousSharedMappings = 1 << 1;
		
		#[allow(missing_docs)]
		const DumpFileBackedPrivateMappings = 1 << 2;
		
		#[allow(missing_docs)]
		const DumpFileBackedSharedMappings = 1 << 3;
		
		/// ELF headers are only dumped if the kernel was built with `CONFIG_CORE_DUMP_DEFAULT_ELF_HEADERS`.
		///
		/// Since Linux 2.6.24.
		const DumpElfHeaders = 1 << 4;
		
		/// Since Linux 2.6.28.
		const DumpPrivateHugePages = 1 << 5;
		
		/// Since Linux 2.6.28.
		const DumpSharedHugePages = 1 << 6;
		
		/// `DAX`: persistent memory mappings.
		///
		/// Since Linux 4.4.
		const DumpPrivateDirectlyAccessiblePages = 1 << 7;
		
		/// `DAX`: persistent memory mappings.
		///
		/// Since Linux 4.4.
		const DumpSharedDirectlyAccessiblePages = 1 << 8;
	}
}

impl Default for CoreDumpFilterFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		CoreDumpFilterFlags::DumpAnonymousPrivateMappings | CoreDumpFilterFlags::DumpAnonymousSharedMappings | CoreDumpFilterFlags::DumpElfHeaders | CoreDumpFilterFlags::DumpPrivateHugePages
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for CoreDumpFilterFlags
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		ZeroPaddedLowerCaseHexadecimalInteger(self.bits).into_line_feed_terminated_byte_string()
	}
}

impl CoreDumpFilterFlags
{
	/// Change core dump filter settings.
	#[inline(always)]
	pub fn change_core_dump_filter(self, proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> io::Result<()>
	{
		let file_path = proc_path.process_file_path(process_identifier, "coredump_filter");
		if likely!(file_path.exists())
		{
			file_path.write_value(self)
		}
		else
		{
			Ok(())
		}
	}
}
