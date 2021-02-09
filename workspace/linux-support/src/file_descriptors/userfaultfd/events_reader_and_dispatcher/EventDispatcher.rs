// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct EventDispatcher<UFEH: UserFaultEventHandler>
{
	user_fault_event_handler: UFEH
}

impl<UFEH: UserFaultEventHandler> EventDispatcher<UFEH>
{
	const fn new(user_fault_event_handler: UFEH) -> Self
	{
		Self
		{
			user_fault_event_handler
		}
	}
	
	#[inline(always)]
	fn dispatch_event(&mut self, event: &uffd_msg)
	{
		use self::UserFaultEvent::*;
		
		let arg = event.arg;
		
		match event.event
		{
			PageFault =>
			{
				let (page_fault_event_type, page_aligned_address_of_page_that_caused_page_fault, thread_identifier) = Self::page_fault_arguments(arg);
				
				use self::PageFaultEventType::*;
				
				match page_fault_event_type
				{
					MissingReadFault => self.user_fault_event_handler.missing_read_page_fault(page_aligned_address_of_page_that_caused_page_fault, thread_identifier),
					
					MissingWriteFault => self.user_fault_event_handler.missing_write_page_fault(page_aligned_address_of_page_that_caused_page_fault, thread_identifier),
					
					WriteProtectionFault => self.user_fault_event_handler.write_protection_page_fault(page_aligned_address_of_page_that_caused_page_fault, thread_identifier),
				}
			}
			
			Fork => self.user_fault_event_handler.fork(Self::fork_arguments(arg)),
			
			Remap =>
			{
				let (from_registered_memory_subrange, to) = Self::remap_arguments(arg);
				self.user_fault_event_handler.remap(from_registered_memory_subrange, to)
			}
			
			Remove => self.user_fault_event_handler.remove(Self::remove_or_unmap_arguments(arg)),
			
			Unmap => self.user_fault_event_handler.unmap(Self::remove_or_unmap_arguments(arg)),
		}
	}
	
	#[inline(always)]
	fn page_fault_arguments(arg: uffd_msg_arg) -> (PageFaultEventType, VirtualAddress, Option<ThreadIdentifier>)
	{
		let page_fault = unsafe { arg.pagefault };
		let page_fault_event_type = page_fault.flags;
		let page_aligned_address_of_page_that_caused_page_fault = VirtualAddress::from(page_fault.address);
		let thread_identifier = unsafe { page_fault.feat.ptid };
		(page_fault_event_type, page_aligned_address_of_page_that_caused_page_fault, thread_identifier)
	}
	
	#[inline(always)]
	fn fork_arguments(arg: uffd_msg_arg) -> UserFaultFileDescriptor
	{
		let fork = unsafe { arg.fork };
		UserFaultFileDescriptor(fork.ufd)
	}
	
	#[inline(always)]
	fn remap_arguments(arg: uffd_msg_arg) -> (FastAbsoluteMemoryRange, VirtualAddress)
	{
		let remap = unsafe { arg.remap };
		let from_registered_memory_subrange = FastAbsoluteMemoryRange::new(VirtualAddress::from(remap.from), remap.len as usize);
		let to = VirtualAddress::from(remap.to);
		(from_registered_memory_subrange, to)
	}
	
	#[inline(always)]
	fn remove_or_unmap_arguments(arg: uffd_msg_arg) -> FastAbsoluteMemoryRange
	{
		let remove = unsafe { arg.remove };
		let inclusive_absolute_start = VirtualAddress::from(remove.start);
		let end = VirtualAddress::from(remove.end);
		FastAbsoluteMemoryRange::new(inclusive_absolute_start, end - inclusive_absolute_start)
	}
}
