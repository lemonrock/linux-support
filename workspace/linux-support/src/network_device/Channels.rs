// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Channels of a network device.
///
/// A channel has one or more ring queues associated with it.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, HAsh)]
pub struct Channels
{
	/// Receive and transmit channels count ('combined'); is ***NOT*** the sum of `receive_channels_count + transmit_channels_count`.
	///
	/// In some drivers this takes over from `receive_only_channels_count` and `transmit_only_channels_count` (eg Huawei `hinic` as of 29th May: <https://lkml.org/lkml/2020/5/29/1399>).
	/// In some drivers this is zero but `receive_only_channels_count` and `transmit_only_channels_count` are the same (eg Huawei `hinic` before 29th May).
	///
	/// It seems that the 'desired' option is for a network driver to report this value rather than separate values for `receive_only_channels_count` and `transmit_only_channels_count` if `receive_only_channels_count == transmit_only_channels_count`.
	///
	/// Ultimately, a network card must have at least one receive channel and one transmit channel, which might be shared for receive and transmit, but it has at least one for each.
	receive_and_transmit_channels_count: Option<NonZeroU32>,
	
	/// Receive channels count.
	receive_only_channels_count: Option<NonZeroU32>,
	
	/// Transmit channels count.
	transmit_only_channels_count: Option<NonZeroU32>,
}

impl Channels
{
	#[inline(always)]
	pub(crate) fn new(receive_and_transmit_channels_count: Option<NonZeroU32>, receive_only_channels_count: Option<NonZeroU32>, transmit_only_channels_count: Option<NonZeroU32>) -> Self
	{
		Self
		{
			receive_and_transmit_channels_count,
			receive_only_channels_count,
			transmit_only_channels_count,
		}
	}
	
	/// How many channels are available?
	///
	/// Logic based on `xsk_get_max_queues()` in `xsk.c` in the Linux kernel sources.
	#[inline(always)]
	pub fn maximum_channels_count(&self) -> NonZeroU32
	{
		max(self.receive_channels_count(), self.transmit_channels_count())
	}
	
	/// How many receive channels are available?
	#[inline(always)]
	pub fn receive_channels_count(&self) -> NonZeroU32
	{
		self.channels_count(self.receive_only_channels_count)
	}
	
	/// How many transmit channels are available?
	#[inline(always)]
	pub fn transmit_channels_count(&self) -> NonZeroU32
	{
		self.channels_count(self.transmit_only_channels_count)
	}
	
	/// Logic based on `xsk_get_max_queues()` in `xsk.c` in the Linux kernel sources.
	#[inline(always)]
	fn channels_count(&self, only_channels_count: Option<NonZeroU32>) -> NonZeroU32
	{
		const One: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(1) };
		
		let only_channels_count = Self::channel_count_to_u32(only_channels_count);
		let receive_and_transmit_channels_count = Self::channel_count_to_u32(self.receive_and_transmit_channels_count);
		match max(only_channels_count, receive_and_transmit_channels_count)
		{
			0 => One,
			non_zero @ _ => non_zero
		}
	}
	
	#[inline(always)]
	const fn channel_count_to_u32(channel_count: Option<NonZeroU32>) -> u32
	{
		unsafe { transmute(channel_count) }
	}
}
