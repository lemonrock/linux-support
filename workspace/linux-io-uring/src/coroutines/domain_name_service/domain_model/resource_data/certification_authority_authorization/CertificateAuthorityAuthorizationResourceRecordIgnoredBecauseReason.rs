// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Why was a `CAA` record ignored?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CertificateAuthorityAuthorizationResourceRecordIgnoredBecauseReason<'a>
{
	/// The tag length exceeded 15.
	TagLengthExceeded15(usize),

	/// The flag bits contained unassigned values.
	UseOfUnassignedFlagBits(u8),

	/// RFC Errata 3547 clarified that RFC 6844 reserves the property names `auth`, `path` and `policy`.
	TagReservedByRfcErrata3547(&'a [u8]),

	/// Unassigned property name.
	TagUnassigned(&'a [u8]),
}
