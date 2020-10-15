// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Left-justified, zero-padded 0 to 4 byte (inclusive) ASCII explanatory code.
///
/// Defined in RFC 4330, Section 8, Figure 3.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct KissOfDeathCode([i8; 4]);

impl KissOfDeathCode
{
	/// The association belongs to a anycast server.
	///
	/// Defined in RFC 4330, Section 8, Figure 3.
	pub const ACST: Self = Self(b"ACST");
	
	/// Server authentication failed.
	///
	/// Defined in RFC 4330, Section 8, Figure 3.
	pub const AUTH: Self = Self(b"AUTH");
	
	/// Autokey sequence failed.
	///
	/// Defined in RFC 4330, Section 8, Figure 3.
	pub const AUTO: Self = Self(b"AUTO");
	
	/// The association belongs to a broadcast server.
	///
	/// Defined in RFC 4330, Section 8, Figure 3.
	pub const BCST: Self = Self(b"BCST");
	
	/// Cryptographic authentication or identification failed.
	///
	/// Defined in RFC 4330, Section 8, Figure 3.
	pub const CRYP: Self = Self(b"CRYP");
	
	/// Access denied by remote server.
	///
	/// Defined in RFC 4330, Section 8, Figure 3.
	pub const DENY: Self = Self(b"DENY");
	
	/// Lost peer in symmetric mode.
	///
	/// Defined in RFC 4330, Section 8, Figure 3.
	pub const DROP: Self = Self(b"DROP");
	
	/// Access denied due to local policy.
	///
	/// Defined in RFC 4330, Section 8, Figure 3.
	pub const RSTR: Self = Self(b"RSTR");
	
	/// The association has not yet synchronized for the first time.
	///
	/// Defined in RFC 4330, Section 8, Figure 3.
	pub const INIT: Self = Self(b"INIT");
	
	/// The association belongs to a manycast server.
	///
	/// Defined in RFC 4330, Section 8, Figure 3.
	pub const MCST: Self = Self(b"MCST");
	
	/// No key found.  Either the key was never installed or is not trusted.
	///
	/// Defined in RFC 4330, Section 8, Figure 3.
	pub const NKEY: Self = Self(b"NKEY");
	
	/// Rate exceeded.
	///
	/// The server has temporarily denied access because the client exceeded the rate threshold.
	///
	/// Defined in RFC 4330, Section 8, Figure 3.
	pub const RATE: Self = Self(b"RATE");
	
	/// Somebody is tinkering with the association from a remote host running ntpdc.
	///
	/// Not to worry unless some rascal has stolen your keys.
	///
	/// Defined in RFC 4330, Section 8, Figure 3.
	pub const RMOT: Self = Self(b"RMOT");
	
	/// A step change in system time has occurred, but the association has not yet resynchronized.
	///
	/// Defined in RFC 4330, Section 8, Figure 3.
	pub const STEP: Self = Self(b"STEP");
}
