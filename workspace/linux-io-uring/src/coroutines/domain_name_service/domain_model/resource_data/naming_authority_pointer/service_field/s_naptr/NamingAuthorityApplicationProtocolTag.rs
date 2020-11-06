// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See <https://www.iana.org/assignments/s-naptr-parameters/s-naptr-parameters.xhtml#s-naptr-parameters-2>.
///
/// Defined by RFC 3958, Section 6.5 Service Parameters.
///
/// * Compared case-insensitive;
/// * Can not be empty;
/// * Can not be more than 32 characters;
/// * The first byte must be `A-Z` or `a-z`;
/// * Subsequenty bytes can be `A-Z`, `a-z`, `0-9`, `+`, `-` and `.` (the later symbols are specified by [Errata 2106](https://www.rfc-editor.org/errata/eid2106));
/// * Experimental application services start `x-`, which is *not* compared case insensitively but probably should be.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NamingAuthorityApplicationServiceTag
{
	/// `CCMP`.
	/// Defined by RFC 6503.
	CCMP,

	/// `diameter.dtls.sctp`.
	/// Defined by RFC 6733.
	diameter_dtls_sctp,

	/// `diameter.sctp`.
	/// Defined by RFC 6408.
	diameter_sctp,

	/// `diameter.tcp`.
	/// Defined by RFC 6408.
	diameter_tcp,

	/// `diameter.tls.tcp`.
	/// Defined by RFC 6408.
	diameter_tls_tcp,

	/// `HELD`.
	/// Defined by RFC 5986.
	HELD,

	/// `http`.
	/// Defined by RFC 5222.
	http,

	/// `https`.
	/// Defined by RFC 5222.
	https,

	/// `iris.beep`.
	/// Defined by RFC 3983.
	iris_beep,

	/// `iris.lwz`.
	/// Defined by RFC 4993.
	iris_lwz,

	/// `iris.xpc`.
	/// Defined by RFC 4992.
	iris_xpc,

	/// `iris.xpcs`.
	/// Defined by RFC 4992.
	iris_xpcs,

	/// `radius.dtls.udp`.
	/// Defined by RFC 7585.
	radiu_dtls_udp,

	/// `radius.tls.tcp`.
	/// Defined by RFC 7585.
	radius_tls_tcp,

	/// `turn.dtls`.
	/// Defined by RFC 7350.
	turn_dtls,

	/// `turn.tcp`.
	/// Defined by RFC 5928.
	turn_tcp,

	/// `turn.tls`.
	/// Defined by RFC 5928.
	turn_tls,

	/// `turn.udp`.
	/// Defined by RFC 5928.
	turn_udp,

	/*
CCMP	RFC6503
diameter.dtls.sctp	RFC6733
diameter.sctp	RFC6408
diameter.tcp	RFC6408
diameter.tls.tcp	RFC6408
HELD	RFC5986
http	RFC2616RFC5222
https	RFC2818RFC5222
iris.beep	RFC3983
iris.lwz	RFC4993
iris.xpc	RFC4992
iris.xpcs	RFC4992
radius.dtls.udp	RFC7585
radius.tls.tcp	RFC7585
turn.dtls	RFC7350
turn.tcp	RFC5928
turn.tls	RFC5928
turn.udp	RFC5928
	 */
}

impl NamingAuthorityApplicationServiceTag
{
	#[inline(always)]
	pub(crate) fn parse(bytes: &[u8]) -> Result<Either<Self, NamingAuthorityResourceRecordIgnoredReason>, TagKeyParseError>
	{
		TagKey::parse()
	}
}
