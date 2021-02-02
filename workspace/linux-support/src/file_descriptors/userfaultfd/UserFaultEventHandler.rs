// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle user fault events.
pub trait UserFaultEventHandler
{
	/// A page fault occurred on a registered memory range.
	///
	/// `thread_identifier` is only `Some()` if `Features::ThreadIdentifier` was requested and supported.
	fn page_fault(&mut self, virtual_address: VirtualAddress, page_fault_event_flags: PageFaultEventFlags, thread_identifier: Option<ThreadIdentifier>);
	
	/// The process has forked.
	fn fork(&mut self, child_process_user_fault_file_descriptor: FileDescriptorCopy<UserFaultFileDescriptor>);
	
	/// The process used `mremap()` on a registered memory range.
	fn remap(&mut self, from_mapped_absolute_memory_range: FastAbsoluteMemoryRange, to: VirtualAddress);
	
	/// The process used `madvise(MADV_DONT_NEED)` or `madvise(MADV_REMOVE)` on a registered memory range.
	fn remove(&mut self, from_mapped_absolute_memory_range: FastAbsoluteMemoryRange);
	
	/// The process used `munmap()` on a registered memory range.
	fn unmap(&mut self, from_mapped_absolute_memory_range: FastAbsoluteMemoryRange);
	
	/// An unknown event (not currently possible).
	#[allow(dead_code)]
	fn future_unsupported_event(&mut self, reserved1: u64, reserved2: u64, reserved3: u64)
	{
		panic!("Future unsupported event")
	}
}
