// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// 'Legacy' interface flags.
	#[derive(Deserialize, Serialize)]
	#[serde(deny_unknown_fields)]
	pub struct InterfaceFlags: u8
	{
		/// A secondary (alias) address.
		const Secondary = IFA_F_SECONDARY as u8;
		
		/// Only for Internet Protocol version 6.
		const NoDuplicateAddressDetection = IFA_F_NODAD as u8;
		
		/// Only for Internet Protocol version 6.
		const OptimisticDuplicateAddressDetection = IFA_F_OPTIMISTIC as u8;
		
		/// Only for Internet Protocol version 6.
		const DuplicateAddressDetectionFailed = IFA_F_DADFAILED as u8;
		
		#[allow(missing_docs)]
		const HomeAddress = IFA_F_HOMEADDRESS as u8;
		
		#[allow(missing_docs)]
		const Deprecated = IFA_F_DEPRECATED as u8;
		
		#[allow(missing_docs)]
		const Tentative = IFA_F_TENTATIVE as u8;
		
		/// A permanent address set by the user.
		const Permanent = IFA_F_PERMANENT as u8;
	}
}

impl InterfaceFlags
{
	#[allow(missing_docs)]
	pub const Temporary: Self = Self::Secondary;
}

impl TryFrom<ExtendedInterfaceFlags> for InterfaceFlags
{
	type Error = ();
	
	#[inline(always)]
	fn try_from(value: ExtendedInterfaceFlags) -> Result<Self, Self::Error>
	{
		let bits = value.bits;
		if bits > (u8::MAX as u32)
		{
			Err(())
		}
		else
		{
			Ok(unsafe { transmute(bits as u8) })
		}
	}
}
