// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use crate::paths::ProcPath;
use libc::pid_t;
use libc_extra::unix::unistd::getpagesize;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::mem::size_of;
#[allow(deprecated)] use std::mem::uninitialized;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::path::Path;
use std::ptr::NonNull;


/// Huge Pages.
pub mod huge_pages;

/// Memory information.
pub mod information;


include!("HasVirtualAddress.rs");
include!("page_size.rs");
include!("PageMap.rs");
include!("PageMapEntry.rs");
include!("PhysicalAddress.rs");
include!("PhysicalPageFrameNumber.rs");
include!("VirtualAddress.rs");
include!("VirtualPageFrameNumber.rs");


const DirectMemoyAccessMemoryAlignment: usize = 128;

/*
-- Allocate DMA-friendly memory.
-- Return virtual memory pointer, physical address, and actual size.
function dma_alloc (bytes,  align)
   align = align or DirectMemoyAccessMemoryAlignment
   assert(bytes <= huge_page_size)
   -- Get current chunk of memory to allocate from
   if #chunks == 0 then allocate_next_chunk() end
   local chunk = chunks[#chunks]
   -- Skip allocation forward pointer to suit alignment
   chunk.used = lib.align(chunk.used, align)
   -- Need a new chunk to service this allocation?
   if chunk.used + bytes > chunk.size then
      allocate_next_chunk()
      chunk = chunks[#chunks]
   end
   -- Slice out the memory we need
   local where = chunk.used
   chunk.used = chunk.used + bytes
   return chunk.pointer + where, chunk.physical + where, bytes
end

*/
