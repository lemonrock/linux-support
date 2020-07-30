// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// RFC 2863 operational status.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum IF_OPER
{
	#[allow(missing_docs)]
	IF_OPER_UNKNOWN = 0,
	
	#[allow(missing_docs)]
	IF_OPER_NOTPRESENT = 1,
	
	#[allow(missing_docs)]
	IF_OPER_DOWN = 2,
	
	#[allow(missing_docs)]
	IF_OPER_LOWERLAYERDOWN = 3,
	
	#[allow(missing_docs)]
	IF_OPER_TESTING = 4,
	
	#[allow(missing_docs)]
	IF_OPER_DORMANT = 5,
	
	#[allow(missing_docs)]
	IF_OPER_UP = 6,
}
