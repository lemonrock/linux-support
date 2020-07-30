// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Message data.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExpressDataPathGetLinkMessageData
{
	pub(crate) attached: XDP_ATTACHED,
	
	pub(crate) program_identifier: Option<ExtendedBpfProgramIdentifier>,
	
	pub(crate) generic_program_identifier: Option<ExtendedBpfProgramIdentifier>,
	
	pub(crate) native_program_identifier: Option<ExtendedBpfProgramIdentifier>,
	
	pub(crate) offloaded_program_identifier: Option<ExtendedBpfProgramIdentifier>,
}

impl ExpressDataPathGetLinkMessageData
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new()
	}
}

impl ExpressDataPathGetLinkMessageData
{
	#[inline(always)]
	pub(crate) const fn new() -> Self
	{
		Self
		{
			attached: XDP_ATTACHED::XDP_ATTACHED_NONE,
			program_identifier: None,
			generic_program_identifier: None,
			native_program_identifier: None,
			offloaded_program_identifier: None,
		}
	}
	
	/// Gets either a program identifier or the multiply-attached program identifiers, which should (a) differ from each other an (b) at least two of the three will be `Some()`.
	pub fn program_identifier(&self) -> Option<MultipleProgramIdentifiers>
	{
		use self::XDP_ATTACHED::*;
		
		use self::MultipleProgramIdentifiers::*;
		
		match self.attached
		{
			XDP_ATTACHED_NONE =>
			{
				debug_assert!(self.program_identifier.is_none());
				debug_assert!(self.generic_program_identifier.is_none());
				debug_assert!(self.native_program_identifier.is_none());
				debug_assert!(self.offloaded_program_identifier.is_none());
				
				None
			}
			
			XDP_ATTACHED_SKB =>
			{
				debug_assert!(self.program_identifier.is_some());
				
				Some(Generic(self.program_identifier.unwrap()))
			}
			
			XDP_ATTACHED_DRV =>
			{
				debug_assert!(self.program_identifier.is_some());
				
				Some(Native(self.program_identifier.unwrap()))
			}
			
			XDP_ATTACHED_HW =>
			{
				debug_assert!(self.program_identifier.is_some());
				
				Some(Offloaded(self.program_identifier.unwrap()))
			}
			
			XDP_ATTACHED_MULTI =>
			{
				debug_assert_ne!(self.generic_program_identifier, self.offloaded_program_identifier);
				debug_assert_ne!(self.native_program_identifier, self.offloaded_program_identifier);
				debug_assert_ne!(self.native_program_identifier, self.generic_program_identifier);
				
				match (self.generic_program_identifier, self.native_program_identifier, self.offloaded_program_identifier)
				{
					(Some(generic_program_identifier), Some(native_program_identifier), None) => Some(Right(GenericAndNative(generic_program_identifier, native_program_identifier))),
					(Some(generic_program_identifier), None, Some(offloaded_program_identifier)) => Right(GenericAndOffloaded(generic_program_identifier, offloaded_program_identifier))),
					(None, Some(native_program_identifier), Some(offloaded_program_identifier)) => Some(Right(NativeAndOffloaded(native_program_identifier, offloaded_program_identifier))),
					(Some(generic_program_identifier), Some(native_program_identifier), Some(offloaded_program_identifier)) => Some(Right(GenericAndNativeAndOffloaded(generic_program_identifier, native_program_identifier, offloaded_program_identifier))),
				}
			}
		}
		
		
		//static __u32 get_xdp_id(struct xdp_link_info *info, __u32 flags)
		// {
		// 	if (info->attach_mode != XDP_ATTACHED_MULTI)
		// 		return info->prog_id;
		// 	if (flags & XDP_FLAGS_DRV_MODE)
		// 		return info->drv_prog_id;
		// 	if (flags & XDP_FLAGS_HW_MODE)
		// 		return info->hw_prog_id;
		// 	if (flags & XDP_FLAGS_SKB_MODE)
		// 		return info->skb_prog_id;
		//
		// 	return 0;
		// }
	}
}
