// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct RegisteredBufferSettings
{
	pub _4Kb: RegisteredBufferSetting<[u8; RegisteredBuffers::_4Kb]>,
	pub _16Kb: RegisteredBufferSetting<[u8; RegisteredBuffers::_16Kb]>,
	pub _64Kb: RegisteredBufferSetting<[u8; RegisteredBuffers::_64Kb]>,
	pub _256Kb: RegisteredBufferSetting<[u8; RegisteredBuffers::_256Kb]>,
	pub _1Mb: RegisteredBufferSetting<[u8; RegisteredBuffers::_1Mb]>,
	pub _4Mb: RegisteredBufferSetting<[u8; RegisteredBuffers::_4Mb]>,
	pub _16Mb: RegisteredBufferSetting<[u8; RegisteredBuffers::_16Mb]>,
	pub _64Mb: RegisteredBufferSetting<[u8; RegisteredBuffers::_64Mb]>,
	pub _256Mb: RegisteredBufferSetting<[u8; RegisteredBuffers::_256Mb]>,
	pub _1Gb: RegisteredBufferSetting<[u8; RegisteredBuffers::_1Gb]>,
}
