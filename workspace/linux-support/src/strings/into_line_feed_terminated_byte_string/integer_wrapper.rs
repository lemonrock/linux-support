// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! integer_wrapper
{
	($name: ident, $method: ident) =>
	{
		#[allow(missing_docs)]
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[derive(Deserialize, Serialize)]
		#[repr(transparent)]
		pub struct $name<IILFTBS: IntegerIntoLineFeedTerminatedByteString>(pub IILFTBS);
		
		impl<IILFTBS: IntegerIntoLineFeedTerminatedByteString> From<IILFTBS> for $name<IILFTBS>
		{
			#[inline(always)]
			fn from(value: IILFTBS) -> Self
			{
				Self(value)
			}
		}
		
		impl<IILFTBS: IntegerIntoLineFeedTerminatedByteString> IntoLineFeedTerminatedByteString<'static> for $name<IILFTBS>
		{
			#[inline(always)]
			fn into_line_feed_terminated_byte_string(self) -> Cow<'static, [u8]>
			{
				(self.0).$method()
			}
		}
	}
}
