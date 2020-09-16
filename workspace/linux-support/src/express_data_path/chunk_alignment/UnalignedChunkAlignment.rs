// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Unaligned chunks.
///
/// Useful for very large (eg Jumbo) frames.
///
/// Requires huge pages; even then, if a chunk lies over two non-contiguous huge pages, this will cause an error inside the Linux kernel.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnalignedChunkAlignment;

impl ChunkAlignment for UnalignedChunkAlignment
{
	const IsUnaligned: bool = true;
	
	#[inline(always)]
	fn user_memory_area_relative_address(_chunk_size: ChunkSize, descriptor: &xdp_desc) -> UserMemoryAreaRelativeAddress
	{
		descriptor.extract_address_if_unaligned() + descriptor.extract_offset_if_unaligned()
	}
}
