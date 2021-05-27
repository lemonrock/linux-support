// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Private Enterprise Number, registered with IANA.
///
/// Defaults to `Reserved`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct PrivateEnterpriseNumber
{
	/// Private Enterprise Number, registered with IANA.
	pub private_enterprise_number: u32,
	
	/// Any sub-domain numbering.
	pub sub_domain: ArrayVec<u32, 4>
}

impl Display for PrivateEnterpriseNumber
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.to_string())
	}
}

impl Default for PrivateEnterpriseNumber
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Reserved()
	}
}

impl PrivateEnterpriseNumber
{
	/// Reserved.
	#[inline(always)]
	pub fn Reserved() -> Self
	{
		Self
		{
			private_enterprise_number: 0,
			sub_domain: ArrayVec::new(),
		}
	}
	
	/// Prefix of `iso.org.dod.internet.private.enterprise.`.
	const Prefix: &'static str = "1.3.6.1.4.1.";
	
	#[inline(always)]
	fn to_string(&self) -> String
	{
		let mut number_string = Self::Prefix.to_string();
		
		number_string.push_str(&format!("{}", self.private_enterprise_number));
		
		for sub_domain_private_enterprise_number in self.sub_domain.iter()
		{
			number_string.push('.');
			number_string.push_str(&format!("{}", sub_domain_private_enterprise_number));
		}
		
		number_string
	}
}
