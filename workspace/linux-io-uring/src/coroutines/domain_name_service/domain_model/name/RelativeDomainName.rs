// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A relative domain name.
///
/// Also known as an UnqualifiedMulti-label Domain Name.
#[derive(, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelativeDomainName(EfficientCaseFoldedName);

impl<'label> TryFrom<&'label [u8]> for RelativeDomainName
{
	type Error = EfficientCaseFoldedNameParseError;
	
	#[inline(always)]
	fn try_from(value: &'label [u8]) -> Result<Self, Self::Error>
	{
		EfficientCaseFoldedName::from_byte_string_ending_with_optional_trailing_period(value).map(Self)
	}
}

impl AsRef<FullyQualifiedDomainName> for RelativeDomainName
{
	#[inline(always)]
	fn as_ref(&self) -> &FullyQualifiedDomainName
	{
		&self.0
	}
}

impl Debug for RelativeDomainName
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.0.display_without_trailing_period(f)
	}
}

impl Display for RelativeDomainName
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.0.display_without_trailing_period(f)
	}
}

impl RelativeDomainName
{
	/// Host name.
	#[inline(always)]
	pub fn host_name(&self) -> HostNameLabel
	{
		self.0.host_name().expect("Should never be just root")
	}
	
	#[inline(always)]
	pub(crate) fn number_of_periods(&self) -> u8
	{
		let number_of_labels = self.0.number_of_labels_including_root().get() - 1;
		debug_assert_ne!(number_of_labels, 0, "Should never be just root");
		number_of_labels - 1
	}
}
