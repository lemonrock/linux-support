// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See also <https://linux-hacks.blogspot.com/2009/01/sample-code-to-learn-netlink.html>.
#[repr(C)]
pub(super) struct nlmsghdr
{
	pub(super) nlmsg_len: u32,
	
	pub(super) nlmsg_type: NetlinkMessageType,
	
	pub(super) nlmsg_flags: NetlinkMessageFlags,
	
	pub(super) nlmsg_seq: SequenceNumber,
	
	pub(super) nlmsg_pid: PortIdentifier,
}

impl nlmsghdr
{
	#[inline(always)]
	pub(super) fn error(&self) -> &nlmsgerr
	{
		unsafe { & * (self.NLMSG_DATA() as *const nlmsgerr) }
	}
	
	#[inline(always)]
	pub(super) fn data(&self) -> &[u8]
	{
		unsafe { from_raw_parts(self.NLMSG_DATA(), self.NLMSG_PAYLOAD()) }
	}
	
	#[inline(always)]
	fn data_mut(&mut self) -> &[u8]
	{
		unsafe { from_raw_parts_mut(self.NLMSG_DATA() as *mut u8, self.NLMSG_PAYLOAD()) }
	}
	
	#[inline(always)]
	pub(crate) fn NLMSG_OK(remaining_length: usize, might_be_invalid_pointer: *const Self) -> bool
	{
		if remaining_length < Self::NLMSG_HDRLEN
		{
			return false
		}
		
		let our_length = unsafe { & * might_be_invalid_pointer }.length();
		
		our_length >= Self::NLMSG_HDRLEN && our_length <= remaining_length
	}
	
	#[inline(always)]
	pub(crate) fn NLMSG_NEXT(&self, remaining_length: &mut usize) -> *const Self
	{
		let length = Self::NLMSG_ALIGN(self.length());
		
		*remaining_length = (*remaining_length) - length;
		
		unsafe { (self as *const Self as *const u8).add(length) as *const Self }
	}
	
	const NLMSG_ALIGNTO: usize = 4;
	
	const NLMSG_HDRLEN: usize = size_of::<Self>();
	
	#[inline(always)]
	pub const fn NLMSG_ALIGN(length: usize) -> usize
	{
		(length + Self::NLMSG_ALIGNTO - 1) & !(Self::NLMSG_ALIGNTO - 1)
	}
	
	#[inline(always)]
	pub(crate) fn NLMSG_LENGTH(length: usize) -> usize
	{
		length + Self::NLMSG_HDRLEN
	}
	
	#[inline(always)]
	fn NLMSG_SPACE(length: usize) -> usize
	{
		Self::NLMSG_ALIGN(Self::NLMSG_LENGTH(length))
	}
	
	#[inline(always)]
	fn NLMSG_DATA(&self) -> *const u8
	{
		unsafe { (self as *const Self as *const u8).add(Self::NLMSG_LENGTH(0)) }
	}
	
	#[inline(always)]
	fn NLMSG_DATALEN(&self) -> usize
	{
		self.length() - Self::NLMSG_HDRLEN
	}
	
	#[inline(always)]
	fn NLMSG_DATAEND(&self) -> *const u8
	{
		unsafe { (self as *const Self as *const u8).add(self.length()) }
	}
	
	#[inline(always)]
	fn NLMSG_PAYLOAD(&self) -> usize
	{
		self.length() - Self::NLMSG_SPACE(0)
	}
	
	#[inline(always)]
	pub(crate) fn NLMSG_RTA(&self, length: usize) -> *const rtattr
	{
		unsafe { (self as *const Self as *const u8).add(Self::NLMSG_HDRLEN).add(Self::NLMSG_ALIGN(length)) as *const rtattr }
	}
	
	#[inline(always)]
	pub(crate) fn length(&self) -> usize
	{
		self.nlmsg_len as usize
	}
}
