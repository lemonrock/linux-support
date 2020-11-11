// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Defined in various RFCS, including 3982, 4414, 4698 and 5144.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InternetRegistryInformationServiceRegistryType
{
	/// `AREG1`.
	///
	/// Address Registry (`areg`) Type.
	///
	/// Defined by RFC 4698.
	Address,
	
	/// `DCHK1`.
	///
	/// Domain Availability Check (`DCHK`) Registry Type.
	///
	/// Defined by RFC 5144.
	DomainAvailabilityCheck,
	
	/// `DREG1`.
	///
	/// Domain Registry (`dreg`) Type.
	///
	/// Defined by RFC 3982.
	DomainRegistry,
	
	/// `EREG1`.
	///
	/// ENUM Registry Type.
	///
	/// Defined by RFC 4414.
	EnumRegistry,
}
