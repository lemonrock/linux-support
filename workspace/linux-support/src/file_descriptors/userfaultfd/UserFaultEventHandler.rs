// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Handle user fault events.
pub trait UserFaultEventHandler
{
	/// A missing read page fault occurred on a registered memory range.
	///
	/// Called if the memory in `page_aligned_address_of_page_that_caused_page_fault` was within a range registered with `PageFaultEventNotificationSetting::IfMissing` or `PageFaultEventNotificationSetting::BothIfMissingAndIfWriteProtectedPageAccess`.
	///
	/// The thread causing this page fault is suspended and will be resumed ('woken up') when we call either `UserFaultFileDescriptor.copy_registered_memory_subrange(wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange = true)`, `UserFaultFileDescriptor.zero_registered_memory_subrange(wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange = true)` or `UserFaultFileDescriptor.wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange()`.
	///  then the `UserFaultEventHandler` should ensure `copy_memory_range()` or `zero_memory_range()` (the latter is particularly important if the `virtual_address` is for a range notified with the `remove()` event) is called and the thread causing the page fault event is then woken up.
	/// `page_aligned_address_of_page_that_caused_page_fault` is not necessarily page aligned, it can be anywhere from the inclusive start of the page to the inclusive end of the page (eg, for a 4Kb page, anywhere from offset 0 inclusive to offset 4091 inclusive).
	/// `thread_identifier` is `Some` if the `Feature::ThreadIdentifier` is requested (and supported by Linux) for a user fault file descriptor in `UserFaultFileDescriptor::new()`.
	///
	/// __NOTE__: This event handler method will NOT BE CALLED if the feature `Feature::DoNotRaisePageFaultEventsButRaiseSIGBUSSignalInstead` is requested (and supported by Linux) for a user fault file descriptor in `UserFaultFileDescriptor::new()`.
	fn missing_read_page_fault(&mut self, page_aligned_address_of_page_that_caused_page_fault: VirtualAddress, thread_identifier: Option<ThreadIdentifier>);
	
	/// A missing write page fault occurred on a registered memory range.
	///
	/// Called if the memory in `page_aligned_address_of_page_that_caused_page_fault` was within a range registered with `PageFaultEventNotificationSetting::IfMissing` or `PageFaultEventNotificationSetting::BothIfMissingAndIfWriteProtectedPageAccess`.
	///
	/// The thread causing this page fault is suspended and will be resumed ('woken up') when we call either `UserFaultFileDescriptor.copy_registered_memory_subrange(wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange = true)`, `UserFaultFileDescriptor.zero_registered_memory_subrange(wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange = true)` or `UserFaultFileDescriptor.wake_up_suspended_thread_that_page_faulted_in_registered_memory_subrange()`.
	///  then the `UserFaultEventHandler` should ensure `copy_memory_range()` or `zero_memory_range()` (the latter is particularly important if the `virtual_address` is for a range notified with the `remove()` event) is called and the thread causing the page fault event is then woken up.
	/// `page_aligned_address_of_page_that_caused_page_fault` is not necessarily page aligned, it can be anywhere from the inclusive start of the page to the inclusive end of the page (eg, for a 4Kb page, anywhere from offset 0 inclusive to offset 4091 inclusive).
	/// `thread_identifier` is `Some` if the `Feature::ThreadIdentifier` is requested (and supported by Linux) for a user fault file descriptor in `UserFaultFileDescriptor::new()`.
	///
	/// __NOTE__: This event handler method will NOT BE CALLED if the feature `Feature::DoNotRaisePageFaultEventsButRaiseSIGBUSSignalInstead` is requested (and supported by Linux) for a user fault file descriptor in `UserFaultFileDescriptor::new()`.
	fn missing_write_page_fault(&mut self, page_aligned_address_of_page_that_caused_page_fault: VirtualAddress, thread_identifier: Option<ThreadIdentifier>);
	
	/// The thread causing this page fault is suspended and will be resumed ('woken up') when we call `UserFaultFileDescriptor.disable_write_protection_of_registered_memory_subrange(true)`.
	///
	/// Called if the memory in `page_aligned_address_of_page_that_caused_page_fault` was within a range registered with `PageFaultEventNotificationSetting::IfWriteProtectedPageAccess` or `PageFaultEventNotificationSetting::BothIfMissingAndIfWriteProtectedPageAccess`.
	///
	/// If the memory for `page_aligned_address_of_page_that_caused_page_fault` was registered with `PageFaultEventNotificationSetting::RaisePageFaultEventIfMissing` then the `UserFaultEventHandler` should ensure `copy_memory_range()` or `zero_memory_range()` (the latter is particularly important if the `virtual_address` is for a range notified with the `remove()` event) is called and the thread causing the page fault event is then woken up.
	/// `page_aligned_address_of_page_that_caused_page_fault` is not necessarily page aligned, it can be anywhere from the inclusive start of the page to the inclusive end of the page (eg, for a 4Kb page, anywhere from offset 0 inclusive to offset 4091 inclusive).
	/// `thread_identifier` is `Some` if the `Feature::ThreadIdentifier` is requested (and supported by Linux) for a user fault file descriptor in `UserFaultFileDescriptor::new()`.
	///
	/// __NOTE__: This event handler method will NOT BE CALLED if the feature `Feature::DoNotRaisePageFaultEventsButRaiseSIGBUSSignalInstead` is requested (and supported by Linux) for a user fault file descriptor in `UserFaultFileDescriptor::new()`.
	fn write_protection_page_fault(&mut self, page_aligned_address_of_page_that_caused_page_fault: VirtualAddress, thread_identifier: Option<ThreadIdentifier>);
	
	/// The process has forked.
	///
	/// This is intended for non-cooperative monitoring of another process: the thread causing this event to occur has already resumed.
	///
	/// __NOTE__: This event handler method will NOT BE CALLED if the feature `Feature::RaiseForkEvents` is NOT requested (or is NOT supported by Linux) for a user fault file descriptor in `UserFaultFileDescriptor::new()`.
	fn fork(&mut self, child_process_user_fault_file_descriptor: UserFaultFileDescriptor);
	
	/// The process used `mremap()` on a registered memory range.
	///
	/// This is intended for non-cooperative monitoring of another process: the thread causing this event to occur has already resumed.
	///
	/// __NOTE__: This event handler method will NOT BE CALLED if the feature `Feature::RaiseRemapEvents` is NOT requested (or is NOT supported by Linux) for a user fault file descriptor in `UserFaultFileDescriptor::new()`.
	fn remap(&mut self, from_registered_memory_subrange: FastAbsoluteMemoryRange, to: VirtualAddress);
	
	/// The process used `madvise(MADV_DONT_NEED)` or `madvise(MADV_REMOVE)` on a registered memory range.
	///
	/// This is intended for non-cooperative monitoring of another process: the thread causing this event to occur has already resumed.
	///
	/// The memory is not automatically unregistered.
	///
	/// Future page faults in this range should do `zero_memory_range()` (if possible) or just unregister the memory range `from_mapped_absolute_memory_range` (ie pass `from_mapped_absolute_memory_range` as the `memory_range` argument of `UserFaultFileDescriptor::unregister_memory_range()`.
	///
	/// __NOTE__: This event handler method will NOT BE CALLED if the feature `Feature::RaiseMAdviseDoNotNeedOrRemoveEvents` is NOT requested (or is NOT supported by Linux) for a user fault file descriptor in `UserFaultFileDescriptor::new()`.
	fn remove(&mut self, from_registered_memory_subrange: FastAbsoluteMemoryRange);
	
	/// The process used `munmap()` on a registered memory range.
	///
	/// This is intended for non-cooperative monitoring of another process: the thread causing this event to occur has already resumed.
	///
	/// ?The memory is automatically unregistered.
	///
	/// __NOTE__: This event handler method will NOT BE CALLED if the feature `Feature::RaiseUnmapEvents` is NOT requested (or is NOT supported by Linux) for a user fault file descriptor in `UserFaultFileDescriptor::new()`.
	fn unmap(&mut self, from_registered_memory_subrange: FastAbsoluteMemoryRange);
}
