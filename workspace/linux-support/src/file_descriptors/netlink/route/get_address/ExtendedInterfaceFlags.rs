// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Interface flags.
	#[derive(Deserialize, Serialize)]
	pub struct ExtendedInterfaceFlags: u32
	{
		/// A secondary (alias) address.
		const Secondary = IFA_F_SECONDARY;
		
		#[allow(missing_docs)]
		const NoDuplicateAddressDetection = IFA_F_NODAD;
		
		/// ?actually is OptimisticDuplicateAddressDetection?
		const Optimistic = IFA_F_OPTIMISTIC;
		
		#[allow(missing_docs)]
		const DuplicateAddressDetectionFailed = IFA_F_DADFAILED;
		
		#[allow(missing_docs)]
		const HomeAddress = IFA_F_HOMEADDRESS;
		
		#[allow(missing_docs)]
		const Deprecated = IFA_F_DEPRECATED;
		
		#[allow(missing_docs)]
		const Tentative = IFA_F_TENTATIVE;
		
		/// A permanent address set by the user.
		const Permanent = IFA_F_PERMANENT;
		
		#[allow(missing_docs)]
		const ManageTemporaryAddress = IFA_F_MANAGETEMPADDR;
		
		#[allow(missing_docs)]
		const NoPrefixRoute = IFA_F_NOPREFIXROUTE;
		
		#[allow(missing_docs)]
		const MulticastAutoJoin = IFA_F_MCAUTOJOIN;
		
		#[allow(missing_docs)]
		const StablePrivacy = IFA_F_STABLE_PRIVACY;
	}
}

impl From<InterfaceFlags> for ExtendedInterfaceFlags
{
	#[inline(always)]
	fn from(value: InterfaceFlags) -> Self
	{
		unsafe { transmute(value.bits as u32) }
	}
}

impl ExtendedInterfaceFlags
{
	#[allow(missing_docs)]
	pub const Temporary: Self = Self::Secondary;
}
