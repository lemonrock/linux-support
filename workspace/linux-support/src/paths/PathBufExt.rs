// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An extension trait to make it easier to work with sys and proc files and folders.
pub trait PathBufExt
{
	/// Append a path component.
	///
	/// Convenience wrapper of `push()`.
	fn append<P: AsRef<Path>>(self, path: P) -> Self;
}

impl PathBufExt for PathBuf
{
	#[inline(always)]
	fn append<P: AsRef<Path>>(mut self, path: P) -> Self
	{
		self.push(path);
		self
	}
}
