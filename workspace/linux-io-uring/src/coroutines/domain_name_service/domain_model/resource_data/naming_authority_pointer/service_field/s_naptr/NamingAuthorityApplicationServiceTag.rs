// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See <https://www.iana.org/assignments/s-naptr-parameters/s-naptr-parameters.xhtml#s-naptr-parameters-1>.
///
/// Defined by RFC 3958, Section 6.5 Service Parameters.
///
/// * Compared case-insensitive;
/// * Can not be empty;
/// * Can not be more than 32 characters;
/// * The first byte must be `A-Z` or `a-z`;
/// * Subsequenty bytes can be `A-Z`, `a-z`, `0-9`, `+`, `-` and `.`;
/// * Experimental application services start `x-`, which is *not* compared case insensitively but probably should be.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NamingAuthorityApplicationServiceTag
{
	/// `aaa`.
	/// Defined in RFC 6408.
	aaa,

	/// `aaa+acct`.
	/// Defined in RFC 7585.
	aaa_acct,

	/// `aaa+ap1`.
	/// Defined in RFC 6408.
	aaa_ap1,

	/// `aaa+ap2`.
	/// Defined in RFC 6408.
	aaa_ap2,

	/// `aaa+ap3`.
	/// Defined in RFC 6408.
	aaa_ap3,

	/// `aaa+ap4`.
	/// Defined in RFC 6408.
	aaa_ap4,

	/// `aaa+ap5`.
	/// Defined in RFC 6408.
	aaa_ap5,

	/// `aaa+ap6`.
	/// Defined in RFC 6408.
	aaa_ap6,

	/// `aaa+ap7`.
	/// Defined in RFC 6408.
	aaa_ap7,

	/// `aaa+ap8`.
	/// Defined in RFC 6408.
	aaa_ap8,

	/// `aaa+ap9`.
	/// Defined in RFC 6408.
	aaa_ap9,

	/// `aaa+ap16777250`.
	/// Defined in RFC 6408.
	aaa_ap16777250,

	/// `aaa+ap16777251`.
	/// Defined in RFC 6408.
	aaa_ap16777251,

	/// `aaa+ap16777264`.
	/// Defined in RFC 6408.
	aaa_ap16777264,

	/// `aaa+ap16777267`.
	/// Defined in RFC 6408.
	aaa_ap16777267,

	/// `aaa+ap16777281`.
	/// Defined in RFC 6408.
	aaa_ap16777281,

	/// `aaa+ap16777282`.
	/// Defined in RFC 6408.
	aaa_ap16777282,

	/// `aaa+ap16777283`.
	/// Defined in RFC 6408.
	aaa_ap16777283,

	/// `aaa+ap16777284`.
	/// Defined in RFC 6408.
	aaa_ap16777284,

	/// `aaa+ap16777285`.
	/// Defined in RFC 6408.
	aaa_ap16777285,

	/// `aaa+ap16777286`.
	/// Defined in RFC 6408.
	aaa_ap16777286,

	/// `aaa+ap16777287`.
	/// Defined in RFC 6408.
	aaa_ap16777287,

	/// `aaa+ap16777288`.
	/// Defined in RFC 6408.
	aaa_ap16777288,

	/// `aaa+ap16777289`.
	/// Defined in RFC 6408.
	aaa_ap16777289,

	/// `aaa+ap16777290`.
	/// Defined in RFC 6408.
	aaa_ap16777290,

	/// `aaa+ap4294967295`.
	/// Defined in RFC 6408.
	aaa_ap4294967295,

	/// `aaa+auth`.
	/// Defined in RFC 7585.
	aaa_auth,

	/// `aaa+dynauth`.
	/// Defined in RFC 7585.
	aaa_dynauth,

	/// `ALTO`.
	/// Defined in RFC 7286.
	ALTO,

	/// `AREG1`.
	/// Defined in RFC 4698.
	AREG1,

	/// `DCHK1`.
	/// Defined in RFC 5144.
	DCHK1,

	/// `DREG1`.
	/// Defined in RFC 3982.
	DREG1,

	/// `EREG1`.
	/// Defined in RFC 4414.
	EREG1,

	/// `LIS`.
	/// Defined in RFC 5986.
	LIS,

	/// `LoST`.
	/// Defined in RFC 5222.
	LoST,

	/// `LoST-Validation`.
	/// Defined in RFC 8917.
	LoST_Validation,

	/// `no-solicit`.
	/// Defined in RFC 4095.
	no_solicit,

	/// `RELAY`.
	/// Defined in RFC 5928.
	RELAY,

	/// `SFUA.CFG`.
	/// Defined in RFC 6011.
	SFUA_CFG,

	/// `XCON`.
	/// Defined in RFC 6503.
	XCON,

	/*
aaa	RFC 6408
aaa+acct	RFC 7585
aaa+ap1	RFC 6408
aaa+ap2	RFC 6408
aaa+ap3	RFC 6408
aaa+ap4	RFC 6408
aaa+ap5	RFC 6408
aaa+ap6	RFC 6408
aaa+ap7	RFC 6408
aaa+ap8	RFC 6408
aaa+ap9	RFC 6408
aaa+ap16777250	RFC 6408
aaa+ap16777251	RFC 6408
aaa+ap16777264	RFC 6408
aaa+ap16777267	RFC 6408
aaa+ap16777281	RFC 6408
aaa+ap16777282	RFC 6408
aaa+ap16777283	RFC 6408
aaa+ap16777284	RFC 6408
aaa+ap16777285	RFC 6408
aaa+ap16777286	RFC 6408
aaa+ap16777287	RFC 6408
aaa+ap16777288	RFC 6408
aaa+ap16777289	RFC 6408
aaa+ap16777290	RFC 6408
aaa+ap4294967295	RFC 6408
aaa+auth	RFC 7585
aaa+dynauth	RFC 7585
ALTO	RFC 7286
AREG1	RFC 4698
DCHK1	RFC 5144
DREG1	RFC 3982
EREG1	RFC 4414
LIS	RFC 5986
LoST	RFC 5222
LoST-Validation	RFC 8917
no-solicit	RFC 4095
RELAY	RFC 5928
SFUA.CFG	RFC 6011
XCON	RFC 6503
	 */
}
