// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Digest.
pub enum DnsSecDigest<'a>
{
	/// SHA2-256 digest.
	Sha2_256(&'a [u8; 256 / 8]),

	/// SHA2-384 digest.
	Sha2_384(&'a [u8; 384 / 8]),
}
