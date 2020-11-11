// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Valid resolution services defined by RFC 2483, but not registered with IANA; these seem to only be applicable when using NAPTR records for URNs and URIs as per RFC 3404.
///
/// Seems to go with RFC 2169.
///
/// Zero, one or more of these may be present but not duplicated.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Hash)]
pub enum Rfc2483ResolutionService
{
	/// `I2`.
	///
	/// URI to URL.
	I2L,
	
	/// `I2Ls`.
	///
	/// URI to URLs.
	I2Ls,
	
	/// `I2R`.
	///
	/// URI to Resource.
	I2R,
	
	/// `I2Rs`.
	///
	/// URI to Resources.
	I2Rs,
	
	/// `I2C`.
	///
	/// URI to URC.
	I2C,
	
	/// `I2CS`.
	///
	/// URI to URCs.
	///
	/// This is a capitalized `S`, unlike the other plurals defintions!
	I2CS,
	
	/// `I2N`.
	///
	/// URI to URN.
	I2N,
	
	/// `I2Ns`.
	///
	/// URI to URNs.
	I2Ns,

	/// `I=I`.
	///
	/// Is URI equal to URI) (URI = URI).
	I_I,
}
