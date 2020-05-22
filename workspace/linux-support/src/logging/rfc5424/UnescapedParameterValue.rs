// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An unescaped parameter value.
#[derive(Debug)]
pub struct UnescapedParameterValue<'a>(Cow<'a, str>);

impl UnescapedParameterValue<'static>
{
	#[allow(dead_code)]
	const False: Self = Self(Cow::Borrowed("0"));
	
	const True: Self = Self(Cow::Borrowed("1"));
	
	const EnglishLanguage: Self = Self(Cow::Borrowed("en"));
	
	const CargoPackageName: Self = Self(Cow::Borrowed(env!("CARGO_PKG_NAME")));
	
	const CargoPackageVersion: Self = Self(Cow::Borrowed(env!("CARGO_PKG_VERSION")));
	
	#[inline(always)]
	fn from_sequence_identifier(sequence_identifier: NonZeroU64) -> Self
	{
		let sequence_identifier_wrapped = (sequence_identifier.get() % 2147483647) + 1;
		
		Self(Cow::Owned(format!("{}", sequence_identifier_wrapped)))
	}
	
	#[inline(always)]
	fn from_internet_protocol_address(internet_protocol_address: &IpAddr) -> Self
	{
		Self(Cow::Owned(format!("{}", internet_protocol_address)))
	}
	
	#[inline(always)]
	fn from_private_enterprise_number(private_enterprise_number: &PrivateEnterpriseNumber) -> Self
	{
		Self(Cow::Owned(private_enterprise_number.to_string()))
	}
	
	/// ***SLOW*** as it uses a syscall whose results than have to be parsed by libc!
	#[inline(always)]
	fn system_up_time() -> Self
	{
		let system_information = system_information();
		Self(Cow::Owned(format!("{}", system_information.uptime)))
	}
}
