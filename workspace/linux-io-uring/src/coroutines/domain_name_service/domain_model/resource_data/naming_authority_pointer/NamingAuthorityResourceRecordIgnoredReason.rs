// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Why was a `NAPTR` record ignored?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NamingAuthorityResourceRecordIgnoredReason
{
	/// A flag byte of `0` to `9` inclusive is for local experimentation.
	NumericFlagBytesAreForLocalExpermination(u8),
	
	/// After 20 years since RFC 2915 (as redefined by RFCs 3401 to 3404 inclusive), the use of bytes other than `s`, `a`, `d`, `u`, `p`, `S`, `A`, `D`, `U`, `P` and `0` to `9` has not been defined.
	UndefinedAlphaFlagByte(u8),
	
	/// More than one flag.
	MultipleFlags,

	/// Ignored service field.
	IgnoredServiceField(IgnoredServiceFieldReason),

	/// Could this actually be permissible for the `P` flag?
	EmptyServiceFieldForTerminalRecord(NamingAuthorityMutuallyExclusiveFlag),
}
