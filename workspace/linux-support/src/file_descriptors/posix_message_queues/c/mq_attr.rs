// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// A `mq_attr` structure.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct mq_attr
{
	// The `#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]` syntax is for Linux x32 ABI compatibility (See <https://sourceware.org/bugzilla/show_bug.cgi?id=21279>).

	/// The `mq_flags` field contains flags associated with the open message queue description.
	/// This field is initialized when the queue is created by `mq_open()`.
	/// The only flag that can appear in this field is `O_NONBLOCK`.
	#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))] mq_flags: i64,
	#[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))] mq_flags: isize,

	/// The `mq_maxmsg` field is an upper limit on the number of messages that may be placed on the queue using `mq_send()`.
	/// This field is initialized when the queue is created by `mq_open()`.
	///
	/// It is always positive (never zero or negative), and can never exceed `65,536`.
	#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))] mq_maxmsg: i64,
	#[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))] mq_maxmsg: isize,

	/// The `mq_msgsize` field is an upper limit on the size of messages that may be placed on the queue.
	/// This field is initialized when the queue is created by `mq_open()`.
	///
	/// It is always positive (never zero or negative), and can never exceed `16,777,216`.
	#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))] mq_msgsize: i64,
	#[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))] mq_msgsize: isize,

	/// The `mq_curmsgs` field returns the number of messages currently held in the queue.
	#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))] mq_curmsgs: i64,
	#[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))] mq_curmsgs: isize,

	#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))] pad: [i64; 4],
	#[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))] pad: [isize; 4],
}

impl mq_attr
{
	#[inline(always)]
	pub(crate) fn for_create(optional_message_queue_create_settings: &OptionalPosixMessageQueueCreateSettings) -> Self
	{
		Self
		{
			mq_flags: unsafe_uninitialized(),
			mq_maxmsg: optional_message_queue_create_settings.maximum_number_of_enqueued_messages,
			mq_msgsize: optional_message_queue_create_settings.maximum_message_size_in_bytes,
			mq_curmsgs: unsafe_uninitialized(),
			pad: unsafe_uninitialized(),
		}
	}

	/// This is always positive (never zero), and can never exceed `65,536`.
	///
	/// It never changes once a queue has been created.
	#[inline(always)]
	pub fn maximum_number_of_enqueued_messages(&self) -> usize
	{
		self.mq_maxmsg as usize
	}

	/// This is always positive (never zero), and can never exceed `16,777,216`.
	///
	/// It never changes once a queue has been created.
	#[inline(always)]
	pub fn maximum_message_size_in_bytes(&self) -> usize
	{
		self.mq_msgsize as usize
	}

	/// This is the number of messages in the queue.
	///
	/// This can never exceed `65,536`; it can be zero.
	///
	/// This is the only interesting property that can vary after a queue has been created.
	#[inline(always)]
	pub fn queue_depth(&self) -> usize
	{
		self.mq_curmsgs as usize
	}
}
