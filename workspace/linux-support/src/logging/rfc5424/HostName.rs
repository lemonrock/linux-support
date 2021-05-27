// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Is not permitted to be empty.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct HostName(ArrayVec<PrintableAsciiCharacter, 255>);

impl Deref for HostName
{
	type Target = [PrintableAsciiCharacter];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0[..]
	}
}

impl HostName
{
	/// New instance.
	///
	/// ***SLOW***.
	pub fn new(host_name: Option<&LinuxKernelHostName>, domain_name: Option<&LinuxKernelDomainName>) -> Result<Self, PrintableAsciiCharacterPushError>
	{
		let mut bytes = ArrayVec::new();
		match (host_name, domain_name)
		{
			(None, None) => PrintableAsciiCharacter::NILVALUE.push_into_array_vec(&mut bytes)?,
			
			(Some(host_name), None) => PrintableAsciiCharacter::push_raw_slice_into_array_vec(&host_name[..], &mut bytes)?,
			
			(Some(host_name), Some(domain_name)) =>
			{
				PrintableAsciiCharacter::push_raw_slice_into_array_vec(&host_name[..], &mut bytes)?;
				PrintableAsciiCharacter::Period.push_into_array_vec(&mut bytes)?;
				PrintableAsciiCharacter::push_raw_slice_into_array_vec(&domain_name[..], &mut bytes)?;
			}
			
			(None, Some(domain_name)) =>
			{
				PrintableAsciiCharacter::NILVALUE.push_into_array_vec(&mut bytes)?;
				PrintableAsciiCharacter::Period.push_into_array_vec(&mut bytes)?;
				PrintableAsciiCharacter::push_raw_slice_into_array_vec(&domain_name[..], &mut bytes)?;
			}
		}
		
		Ok(Self(bytes))
	}
}
