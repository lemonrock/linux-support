// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegisteredBufferSettings
{
	pub _4Kb: RegisteredBufferSetting<MemorySize4Kb>,
	pub _16Kb: RegisteredBufferSetting<MemorySize16Kb>,
	pub _64Kb: RegisteredBufferSetting<MemorySize64Kb>,
	pub _256Kb: RegisteredBufferSetting<MemorySize256Kb>,
	pub _1Mb: RegisteredBufferSetting<MemorySize1Mb>,
	pub _4Mb: RegisteredBufferSetting<MemorySize4Mb>,
	pub _16Mb: RegisteredBufferSetting<MemorySize16Mb>,
	pub _64Mb: RegisteredBufferSetting<MemorySize64Mb>,
	pub _256Mb: RegisteredBufferSetting<MemorySize256Mb>,
	pub _1Gb: RegisteredBufferSetting<MemorySize1Gb>,
}
