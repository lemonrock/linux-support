// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Is not permitted to be empty.
///
/// `PARAM-NAME`.
#[repr(transparent)]
pub struct ParameterName(StructuredDataName);

impl Deref for ParameterName
{
	type Target = [PrintableAsciiCharacter];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0[..]
	}
}

impl ParameterName
{
	/// `tzKnown`.
	#[inline(always)]
	pub fn tzKnown() -> &'static Self
	{
		lazy_static!
		{
			static ref tzKnown: ParameterName = ParameterName(StructuredDataName::iana_registered(b"tzKnown"));
		}
		
		&tzKnown
	}
	
	/// `isSynced`.
	#[inline(always)]
	pub fn isSynced() -> &'static Self
	{
		lazy_static!
		{
			static ref isSynced: ParameterName = ParameterName(StructuredDataName::iana_registered(b"isSynced"));
		}
		
		&isSynced
	}
	
	/// `syncAccuracy`.
	#[inline(always)]
	pub fn syncAccuracy() -> &'static Self
	{
		lazy_static!
		{
			static ref syncAccuracy: ParameterName = ParameterName(StructuredDataName::iana_registered(b"syncAccuracy"));
		}
		
		&syncAccuracy
	}
	
	/// `ip`.
	#[inline(always)]
	pub fn ip() -> &'static Self
	{
		lazy_static!
		{
			static ref ip: ParameterName = ParameterName(StructuredDataName::iana_registered(b"ip"));
		}
		
		&ip
	}
	
	/// `enterpriseId`.
	#[inline(always)]
	pub fn enterpriseId() -> &'static Self
	{
		lazy_static!
		{
			static ref enterpriseId: ParameterName = ParameterName(StructuredDataName::iana_registered(b"enterpriseId"));
		}
		
		&enterpriseId
	}
	
	/// `software`.
	#[inline(always)]
	pub fn software() -> &'static Self
	{
		lazy_static!
		{
			static ref software: ParameterName = ParameterName(StructuredDataName::iana_registered(b"software"));
		}
		
		&software
	}
	
	/// `swVersion`.
	#[inline(always)]
	pub fn swVersion() -> &'static Self
	{
		lazy_static!
		{
			static ref swVersion: ParameterName = ParameterName(StructuredDataName::iana_registered(b"swVersion"));
		}
		
		&swVersion
	}
	
	/// `sequenceId`.
	#[inline(always)]
	pub fn sequenceId() -> &'static Self
	{
		lazy_static!
		{
			static ref sequenceId: ParameterName = ParameterName(StructuredDataName::iana_registered(b"sequenceId"));
		}
		
		&sequenceId
	}
	
	/// `sysUpTime`.
	#[inline(always)]
	pub fn sysUpTime() -> &'static Self
	{
		lazy_static!
		{
			static ref sysUpTime: ParameterName = ParameterName(StructuredDataName::iana_registered(b"sysUpTime"));
		}
		
		&sysUpTime
	}
	
	/// `language`.
	#[inline(always)]
	pub fn language() -> &'static Self
	{
		lazy_static!
		{
			static ref language: ParameterName = ParameterName(StructuredDataName::iana_registered(b"language"));
		}
		
		&language
	}
	
	/// ***SLOW***.
	#[inline(always)]
	pub fn new(name: &[u8]) -> Result<Self, PrintableAsciiCharacterPushError>
	{
		StructuredDataName::name(name).map(|inner| Self(inner))
	}
}
