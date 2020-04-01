// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Errors that can occur when converting an network interface name to an index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkInterfaceNameToIndexConversionError
{
	/// Whilst technically legal it is never valid on Linux or BSDs - have you made a configuration error?
	ZeroSized,

	/// Longer than IF_NAMESIZE.
	TooLong,

	/// Invalid as a CString.
	InvalidCString(NulError),

	/// Does not exist as an interface.
	DoesNotExistAsAnInterface,
}

impl<'a> Display for NetworkInterfaceNameToIndexConversionError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<NetworkInterfaceNameToIndexConversionError as Debug>::fmt(self, f)
	}
}

impl<'a> error::Error for NetworkInterfaceNameToIndexConversionError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::NetworkInterfaceNameToIndexConversionError::*;

		match self
		{
			&ZeroSized => None,

			&TooLong => None,

			&InvalidCString(ref cause) => Some(cause),

			&DoesNotExistAsAnInterface => None,
		}
	}
}
