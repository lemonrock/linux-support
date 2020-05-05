// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Owns the contents of an `/etc/passwd` file.
///
/// Exists because Rust iterators can not return `Item`s with a lifetime linked to the iterator.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EtcPasswd(Box<[u8]>);

impl EtcPasswd
{
	/// Open.
	#[inline(always)]
	pub fn open<'a>(etc_path: &EtcPath) -> Result<EtcPasswd, EtcPasswdParseError>
	{
		Ok(Self(etc_path.passwd().read_raw()?))
	}

	/// Exists because the `IntoIterator` trait does not support a lifetime on the `into_iter()` function.
	#[inline(always)]
	pub fn iter(&self) -> EtcPasswdIterator
	{
		EtcPasswdIterator
		{
			memchr: memchr_iter(b'\n', &self.0[..]),
			bytes: &self.0[..],
			last_end_of_line: 0,
		}
	}
}
