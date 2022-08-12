// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See the Linux kernel function `btf_parse_hdr()`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(crate) struct btf_header
{
	/// Should be `Self::BTF_MAGIC`.
	magic: u16,
	
	/// Should be `Self::BTF_VERSION`.
	version: u8,
	
	/// Should be `0`.
	flags: u8,
	
	/// Should always be `size_of::<btf_header>()`.
	hdr_len: u32,
	
	/// Type (identifier) section (table) offset relative to the end of the `bpf_header` record.
	///
	/// Must be a multiple of `4`; no need to check this ordinarily as all pointers and malloc-ed memory blobs are a multiple of `8` on 64-bit systems.
	///
	/// See the Linux kernel function `btf_parse_type_sec()`.
	type_off: u32,
	
	/// Type (identifier) section (table) length in bytes.
	///
	/// See the Linux kernel function `btf_parse_type_sec()`.
	type_len: u32,
	
	/// String section (table) offset relative to the end of the `bpf_header` record.
	///
	/// Must always be the last section.
	///
	/// See the Linux kernel function `btf_parse_str_sec()`.
	str_off: u32,
	
	/// String section (table) length in bytes.
	///
	/// Must not be `str_len - 1 > BTF_MAX_NAME_OFFSET`.
	///
	/// See the Linux kernel function `btf_parse_str_sec()`.
	str_len: u32,
}

impl btf_header
{
	/// Like a bad car registration plate, `eBPF`.
	///
	/// Intended to distinguish little-endian and big-endian encodings (in case the BTF data has been loaded from a file created by a machine with a different endianness to the current host).
	const BTF_MAGIC: u16 = 0xeB9F;
	
	const BTF_VERSION: u8 = 1;
	
	pub(crate) const Size: usize = size_of::<Self>();
	
	#[inline(always)]
	pub(crate) fn new(type_off: u32, type_len: u32, str_off: u32, str_len: u32) -> Self
	{
		Self
		{
			magic: Self::BTF_MAGIC,
			version: Self::BTF_VERSION,
			flags: 0,
			hdr_len: Self::Size as u32,
			type_off,
			type_len,
			str_off,
			str_len,
		}
	}
}
