// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// C string fragments.
///
/// Must not include a trailing NULL.
///
/// Used by `NulTerminatedCStringArray::new()`.
pub trait CStringFragments
{
	/// Specialized iteration as general iterators are too difficult to use with the various lifetimes and variable size of collections of fragments (which would require heap allocation).
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ());
}

impl CStringFragments for &[u8]
{
	#[inline(always)]
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ())
	{
		provide_fragment(self)
	}
}

impl CStringFragments for Box<[u8]>
{
	#[inline(always)]
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ())
	{
		provide_fragment(&self[..])
	}
}

impl CStringFragments for &Box<[u8]>
{
	#[inline(always)]
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ())
	{
		provide_fragment(&self[..])
	}
}

impl CStringFragments for Vec<u8>
{
	#[inline(always)]
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ())
	{
		provide_fragment(&self[..])
	}
}

impl CStringFragments for &Vec<u8>
{
	#[inline(always)]
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ())
	{
		provide_fragment(&self[..])
	}
}

impl CStringFragments for CString
{
	#[inline(always)]
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ())
	{
		provide_fragment(self.to_bytes())
	}
}

impl CStringFragments for &CStr
{
	#[inline(always)]
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ())
	{
		provide_fragment(self.to_bytes())
	}
}

impl CStringFragments for OsString
{
	#[inline(always)]
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ())
	{
		provide_fragment(self.as_bytes())
	}
}

impl CStringFragments for &OsStr
{
	#[inline(always)]
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ())
	{
		provide_fragment(self.as_bytes())
	}
}

impl CStringFragments for PathBuf
{
	#[inline(always)]
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ())
	{
		provide_fragment(self.as_os_str().as_bytes())
	}
}

impl CStringFragments for &Path
{
	#[inline(always)]
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ())
	{
		provide_fragment(self.as_os_str().as_bytes())
	}
}

impl CStringFragments for String
{
	#[inline(always)]
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ())
	{
		provide_fragment(self.as_bytes())
	}
}

impl CStringFragments for &str
{
	#[inline(always)]
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ())
	{
		provide_fragment(self.as_bytes())
	}
}

/// This implementation supports Environment.
impl CStringFragments for (&Box<[u8]>, &Option<Box<[u8]>>)
{
	#[inline(always)]
	fn iterate(self, provide_fragment: &mut impl FnMut(&[u8]) -> ())
	{
		let name = self.0;
		let value = self.1;
		match value
		{
			&None =>
			{
				debug_assert!(!name.is_empty(), "An environment variable name can not be an empty string if it has no value");
				provide_fragment(&name[..])
			}

			&Some(ref value) =>
			{
				provide_fragment(&name[..]);
				provide_fragment(b"=");
				provide_fragment(&value[..])
			}
		}
	}
}
