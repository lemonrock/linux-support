// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Is not permitted to be empty.
///
/// `SD-ID`.
#[repr(transparent)]
pub struct StructuredDataIdentifier(StructuredDataName);

impl Deref for StructuredDataIdentifier
{
	type Target = [PrintableAsciiCharacter];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0[..]
	}
}

impl StructuredDataIdentifier
{
	/// `timeQuality`.
	#[inline(always)]
	pub fn timeQuality() -> &'static Self
	{
		lazy_static!
		{
			static ref timeQuality: StructuredDataIdentifier = StructuredDataIdentifier(StructuredDataName::iana_registered(b"timeQuality"));
		}
		
		&timeQuality
	}
	
	/// `origin`.
	#[inline(always)]
	pub fn origin() -> &'static Self
	{
		lazy_static!
		{
			static ref origin: StructuredDataIdentifier = StructuredDataIdentifier(StructuredDataName::iana_registered(b"origin"));
		}
		
		&origin
	}
	
	/// `meta`.
	#[inline(always)]
	pub fn meta() -> &'static Self
	{
		lazy_static!
		{
			static ref meta: StructuredDataIdentifier = StructuredDataIdentifier(StructuredDataName::iana_registered(b"meta"));
		}
		
		&meta
	}
	
	/// `snmp`.
	#[inline(always)]
	pub fn snmp() -> &'static Self
	{
		lazy_static!
		{
			static ref snmp: StructuredDataIdentifier = StructuredDataIdentifier(StructuredDataName::iana_registered(b"snmp"));
		}
		
		&snmp
	}
	
	/// `alarm`.
	#[inline(always)]
	pub fn alarm() -> &'static Self
	{
		lazy_static!
		{
			static ref alarm: StructuredDataIdentifier = StructuredDataIdentifier(StructuredDataName::iana_registered(b"alarm"));
		}
		
		&alarm
	}
	
	/// `ssign`.
	#[inline(always)]
	pub fn ssign() -> &'static Self
	{
		lazy_static!
		{
			static ref ssign: StructuredDataIdentifier = StructuredDataIdentifier(StructuredDataName::iana_registered(b"ssign"));
		}
		
		&ssign
	}
	
	/// `ssign-cert`.
	#[inline(always)]
	pub fn ssign_cert() -> &'static Self
	{
		lazy_static!
		{
			static ref ssign_cert: StructuredDataIdentifier = StructuredDataIdentifier(StructuredDataName::iana_registered(b"ssign-cert"));
		}
		
		&ssign_cert
	}
	
	/// `PCNNode`.
	#[inline(always)]
	pub fn PCNNode() -> &'static Self
	{
		lazy_static!
		{
			static ref PCNNode: StructuredDataIdentifier = StructuredDataIdentifier(StructuredDataName::iana_registered(b"PCNNode"));
		}
		
		&PCNNode
	}
	
	/// `PCNTerm`.
	#[inline(always)]
	pub fn PCNTerm() -> &'static Self
	{
		lazy_static!
		{
			static ref PCNTerm: StructuredDataIdentifier = StructuredDataIdentifier(StructuredDataName::iana_registered(b"PCNTerm"));
		}
		
		&PCNTerm
	}
	
	/// ***SLOW***.
	#[inline(always)]
	pub fn private(name: &[u8], private_enterprise_number: &PrivateEnterpriseNumber) -> Result<Self, PrintableAsciiCharacterPushError>
	{
		StructuredDataName::private(name, private_enterprise_number).map(|inner| Self(inner))
	}
}
