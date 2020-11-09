// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See RFC 6408.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum ModernDiameterApplicationIdentifier
{
	#[allow(missing_docs)]
	_1 = 1,
	
	#[allow(missing_docs)]
	_2 = 2,
	
	#[allow(missing_docs)]
	_3 = 3,
	
	#[allow(missing_docs)]
	_4 = 4,
	
	#[allow(missing_docs)]
	_5 = 5,
	
	#[allow(missing_docs)]
	_6 = 6,
	
	#[allow(missing_docs)]
	_7 = 7,
	
	#[allow(missing_docs)]
	_8 = 8,
	
	#[allow(missing_docs)]
	_9 = 9,
	
	#[allow(missing_docs)]
	_16777250 = 16777250,
	
	#[allow(missing_docs)]
	_16777251 = 16777251,
	
	#[allow(missing_docs)]
	_16777264 = 16777264,
	
	#[allow(missing_docs)]
	_16777267 = 16777267,
	
	#[allow(missing_docs)]
	_16777281 = 16777281,
	
	#[allow(missing_docs)]
	_16777282 = 16777282,
	
	#[allow(missing_docs)]
	_16777283 = 16777283,
	
	#[allow(missing_docs)]
	_16777284 = 16777284,
	
	#[allow(missing_docs)]
	_16777285 = 16777285,
	
	#[allow(missing_docs)]
	_16777286 = 16777286,
	
	#[allow(missing_docs)]
	_16777287 = 16777287,
	
	#[allow(missing_docs)]
	_16777288 = 16777288,
	
	#[allow(missing_docs)]
	_16777289 = 16777289,
	
	#[allow(missing_docs)]
	_16777290 = 16777290,
	
	#[allow(missing_docs)]
	_4294967295 = 4294967295,
}
