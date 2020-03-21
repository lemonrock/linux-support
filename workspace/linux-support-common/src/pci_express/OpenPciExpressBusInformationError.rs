// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error caused when trying to open an ioctl socket.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenPciExpressBusInformationError
{
	/// Permission denied.
	PermissionDenied,

	/// Unsupported.
	///
	/// Field contains details.
	Unsupported(&'static str),

	/// Out of memory or resources.
	///
	/// Field contains details.
	OutOfMemoryOrResources(&'static str),

	/// ioctl call failed.
	IoctlCallFailed,

	/// Invalid C String.
	///
	/// This should not happen.
	InvalidCString(FromBytesWithNulError),

	/// Invalid UTF-8 String.
	///
	/// This should not happen.
	InvalidUtf8String(Utf8Error),

	/// Invalid format for PCI bus address.
	///
	/// This should not happen.
	ParseError(PciDeviceAddressStringParseError),
}

impl Display for OpenPciExpressBusInformationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<OpenPciExpressBusInformationError as Debug>::fmt(self, f)
	}
}

impl From<FromBytesWithNulError> for OpenPciExpressBusInformationError
{
	#[inline(always)]
	fn from(error: FromBytesWithNulError) -> Self
	{
		OpenPciExpressBusInformationError::InvalidCString(error)
	}
}

impl From<Utf8Error> for OpenPciExpressBusInformationError
{
	#[inline(always)]
	fn from(error: Utf8Error) -> Self
	{
		OpenPciExpressBusInformationError::InvalidUtf8String(error)
	}
}

impl From<PciDeviceAddressStringParseError> for OpenPciExpressBusInformationError
{
	#[inline(always)]
	fn from(error: PciDeviceAddressStringParseError) -> Self
	{
		OpenPciExpressBusInformationError::ParseError(error)
	}
}

impl error::Error for OpenPciExpressBusInformationError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::OpenPciExpressBusInformationError::*;

		match self
		{
			&PermissionDenied => None,

			&Unsupported(_) => None,

			&OutOfMemoryOrResources(_) => None,

			&IoctlCallFailed => None,

			&InvalidCString(ref cause) => Some(cause),

			&InvalidUtf8String(ref cause) => Some(cause),

			&ParseError(ref cause) => Some(cause),
		}
	}
}
