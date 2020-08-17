// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A tunable to configure.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum TunableChoice
{
	#[allow(missing_docs)]
	PriorityFlowControlStormPrevention(PriorityFlowControlStormPreventionTunable),
	
	#[allow(missing_docs)]
	DownshiftRetriesCount(DownshiftRetriesCountTunable),
	
	#[allow(missing_docs)]
	ReceiveCopyBreak(ReceiveCopyBreakTunable),
	
	#[allow(missing_docs)]
	TransmitCopyBreak(TransmitCopyBreakTunable),
	
	#[allow(missing_docs)]
	FastLinkDownMilliseconds(FastLinkDownMillisecondsTunable),
	
	#[allow(missing_docs)]
	EnergyDetectPowerDownMilliseconds(EnergyDetectPowerDownMillisecondsTunable),
}

impl TunableChoice
{
	/// Set tunable.
	#[inline(always)]
	pub fn set(&self, network_device_input_output_control: &NetworkDeviceInputOutputControl) -> Result<Option<()>, NetworkDeviceInputOutputControlError<TunableOutOfRangeError>>
	{
		use self::TunableChoice::*;
		
		match self
		{
			&PriorityFlowControlStormPrevention(tunable) => network_device_input_output_control.set_tunable(tunable),
			
			&DownshiftRetriesCount(tunable) => network_device_input_output_control.set_tunable(tunable),
			
			&ReceiveCopyBreak(tunable) => network_device_input_output_control.set_tunable(tunable),
			
			&TransmitCopyBreak(tunable) => network_device_input_output_control.set_tunable(tunable),
			
			&FastLinkDownMilliseconds(tunable) => network_device_input_output_control.set_tunable(tunable),
			
			&EnergyDetectPowerDownMilliseconds(tunable) => network_device_input_output_control.set_tunable(tunable),
		}
		
	}
}
