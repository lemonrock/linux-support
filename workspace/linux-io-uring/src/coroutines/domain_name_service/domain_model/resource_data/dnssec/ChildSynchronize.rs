// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A child synchronize record (`CSYNC`).
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct ChildSynchronize
{
	/// Start-of-Authority (`SOA`) serial number.
	pub start_of_authority_serial: SerialNumber,

	/// `immediate` flag.
	pub immediate: bool,

	/// `soaminimum` flag.
	pub start_of_authority_minimum: bool,

	/// Type bitmaps.
	pub type_bitmaps: TypeBitmaps,
}
