// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// ?Legacy? media selection option.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum IF_PORT
{
	/// Unknown.
	///
	/// Default.
	IF_PORT_UNKNOWN = 0,
	
	#[allow(missing_docs)]
	IF_PORT_10BASE2 = 1,
	
	#[allow(missing_docs)]
	IF_PORT_10BASET = 2,
	
	#[allow(missing_docs)]
	IF_PORT_AUI = 3,
	
	#[allow(missing_docs)]
	IF_PORT_100BASET = 4,
	
	#[allow(missing_docs)]
	IF_PORT_100BASETX = 5,
	
	#[allow(missing_docs)]
	IF_PORT_100BASEFX = 6,
}
