// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A parse erorr.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EfficientCaseFoldedLabelParseError
{
	#[allow(missing_docs)]
	LabelExceeded63Bytes,
	
	/// In theory, this is permitted.
	///
	/// However, it is almost certainly a misconfiguration.
	LabelContainedUppercaseBytes,
	
	/// In theory, this is permitted.
	/// In practice, it is almost certainly a misconfiguration or an attempt to attack a code vulnerability.
	LabelContainedPeriod,
}

impl Display for EfficientCaseFoldedLabelParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for EfficientCaseFoldedLabelParseError
{
}
