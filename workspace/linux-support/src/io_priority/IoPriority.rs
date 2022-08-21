// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// I/O priorities are supported for reads and for synchronous (`O_DIRECT` or `O_SYNC`) writes.
///
/// Sometimes called I/O nice levels.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum IoPriority
{
	/// This is the idle scheduling class.
	///
	/// Processes running at this level get I/O time only when no one else needs the disk.
	/// The idle class has no class data.
	/// Attention is required when assigning this priority class to a process, since it may become starved if higher priority processes are constantly accessing the disk.
	Idle,

	///	This is the best-effort scheduling class, which is the default for any process that hasn't set a specific I/O priority.
	///
	/// The class data (priority) determines how much I/O bandwidth the process will get.
	/// Best-effort priority levels are analogous to CPU nice values (see getpriority(2)).
	/// The priority level determines a priority relative to other processes in the best-effort scheduling class
	/// Priority levels range from 0 (highest) to 7 (lowest).
	///
	/// The default is 4 (`IOPRIO_NORM`).
	BestEffort(RealTimeOrBestEffortIoPriorityLevel),

	/// This is the real-time I/O class.
	///
	/// This scheduling class is given higher priority than any other class: processes from this class are given first access to the disk every time.
	/// Thus, this I/O class needs to be used with some care: one I/O real-time process can starve the entire system.
	/// Within the real-time class, there are 8 levels of class data (priority) that determine exactly how much time this process needs the disk for on each service.
	/// The highest real-time priority level is 0; the lowest is 7.
	RealTime(RealTimeOrBestEffortIoPriorityLevel),
}

impl Default for IoPriority
{
	#[inline(always)]
	fn default() -> Self
	{
		IoPriority::BestEffort(RealTimeOrBestEffortIoPriorityLevel::_4)
	}
}

impl TryFrom<u16> for IoPriority
{
	type Error = &'static str;

	#[inline(always)]
	fn try_from(ioprio: u16) -> Result<Self, Self::Error>
	{
		match Self::parse_ioprio(ioprio)?
		{
			None => Err("Can not be IOPRIO_CLASS::IOPRIO_CLASS_NONE"),
			Some(value) => Ok(value),
		}
	}
}

impl const From<IoPriority> for usize
{
	#[inline(always)]
	fn from(value: IoPriority) -> Self
	{
		u16::from(value) as usize
	}
}

impl const From<IoPriority> for u16
{
	#[inline(always)]
	fn from(value: IoPriority) -> Self
	{
		use self::IoPriority::*;
		use RealTimeOrBestEffortIoPriorityLevel::_0;
		
		let (scheduling_class, scheduling_level) = match value
		{
			Idle => (IOPRIO_CLASS::IOPRIO_CLASS_IDLE, _0),
			BestEffort(priority) => (IOPRIO_CLASS::IOPRIO_CLASS_BE, priority),
			RealTime(priority) => (IOPRIO_CLASS::IOPRIO_CLASS_RT, priority),
		};
		
		IOPRIO_PRIO_VALUE(scheduling_class as u8 as u16, scheduling_level as u8 as u16)
	}
}

impl TryFrom<i32> for IoPriority
{
	type Error = &'static str;

	#[inline(always)]
	fn try_from(ioprio: i32) -> Result<Self, Self::Error>
	{
		if unlikely!(ioprio < 0)
		{
			return Err("ioprio can not be negative")
		}
		match Self::parse_ioprio_result(ioprio)?
		{
			None => Err("Can not be IOPRIO_CLASS::IOPRIO_CLASS_NONE"),
			Some(value) => Ok(value),
		}
	}
}

impl IoPriority
{
	/// Returns `Err(true)` if no matching process or processes found.
	/// Returns `Err(false)` if permission denied.
	pub fn get_for_process_group(process_group_identifier: ProcessGroupIdentifierChoice) -> Result<Self, bool>
	{
		Self::get(process_group_identifier, IOPRIO_WHO::IOPRIO_WHO_PGRP)
	}

	/// Returns `Err(true)` if no matching process or processes found.
	/// Returns `Err(false)` if permission denied.
	#[inline(always)]
	pub fn get_for_process(process_identifier: ProcessIdentifierChoice) -> Result<Self, bool>
	{
		Self::get(process_identifier, IOPRIO_WHO::IOPRIO_WHO_PROCESS)
	}

	/// Returns `Err(true)` if no matching process or processes found.
	/// Returns `Err(false)` if permission denied.
	#[inline(always)]
	pub fn get_for_thread(thread_identifier: ThreadIdentifier) -> Result<Self, bool>
	{
		Self::get(thread_identifier, IOPRIO_WHO::IOPRIO_WHO_PROCESS)
	}

	/// Returns `Err(true)` if no matching process or processes found.
	/// Returns `Err(false)` if permission denied.
	#[inline(always)]
	pub fn get_for_user(user_identifier: UserIdentifier) -> Result<Self, bool>
	{
		Self::get(user_identifier, IOPRIO_WHO::IOPRIO_WHO_USER)
	}
	
	#[inline(always)]
	pub(crate) fn explain_error(error: bool) -> &'static str
	{
		if error
		{
			"no such thread"
		}
		else
		{
			"permission denied"
		}
	}
	
	#[inline(always)]
	fn get(which: impl Into<i32>, who: IOPRIO_WHO) -> Result<Self, bool>
	{
		use IoPriority::*;
		use RealTimeOrBestEffortIoPriorityLevel::*;
		
		const MaximumInclusiveIoprioValue: usize = u16::MAX as usize;
		
		const Idle0: usize = Idle.into();
		const BestEffort0: usize = BestEffort(_0).into();
		const BestEffort1: usize = BestEffort(_1).into();
		const BestEffort2: usize = BestEffort(_2).into();
		const BestEffort3: usize = BestEffort(_3).into();
		const BestEffort4: usize = BestEffort(_4).into();
		const BestEffort5: usize = BestEffort(_5).into();
		const BestEffort6: usize = BestEffort(_6).into();
		const BestEffort7: usize = BestEffort(_7).into();
		const RealTime0: usize = RealTime(_0).into();
		const RealTime1: usize = RealTime(_1).into();
		const RealTime2: usize = RealTime(_2).into();
		const RealTime3: usize = RealTime(_3).into();
		const RealTime4: usize = RealTime(_4).into();
		const RealTime5: usize = RealTime(_5).into();
		const RealTime6: usize = RealTime(_6).into();
		const RealTime7: usize = RealTime(_7).into();
		
		match system_call_ioprio_get(which.into(), who).as_usize()
		{
			Idle0 => Ok(Idle),
			BestEffort0 => Ok(BestEffort(_0)),
			BestEffort1 => Ok(BestEffort(_1)),
			BestEffort2 => Ok(BestEffort(_2)),
			BestEffort3 => Ok(BestEffort(_3)),
			BestEffort4 => Ok(BestEffort(_4)),
			BestEffort5 => Ok(BestEffort(_5)),
			BestEffort6 => Ok(BestEffort(_6)),
			BestEffort7 => Ok(BestEffort(_7)),
			RealTime0 => Ok(RealTime(_0)),
			RealTime1 => Ok(RealTime(_1)),
			RealTime2 => Ok(RealTime(_2)),
			RealTime3 => Ok(RealTime(_3)),
			RealTime4 => Ok(RealTime(_4)),
			RealTime5 => Ok(RealTime(_5)),
			RealTime6 => Ok(RealTime(_6)),
			RealTime7 => Ok(RealTime(_7)),
			unexpected_ioprio @ 0 ..= MaximumInclusiveIoprioValue => panic!("Unexpected ioprio `{}`", (unexpected_ioprio as u16)),
			
			SystemCallResult::EPERM_usize => Err(true),
			SystemCallResult::ESRCH_usize => Err(false),
			SystemCallResult::EINVAL_usize => panic!("Invalid value for which"),
			unexpected_error @ _ => unexpected_error!(ioprio_get, SystemCallResult::usize_to_system_call_error_number(unexpected_error)),
			
			unexpected @ _ => unexpected_result!(ioprio_get, unexpected),
		}
	}

	/// Returns `Err(true)` if no matching process or processes found.
	/// Returns `Err(false)` if permission denied.
	#[inline(always)]
	pub fn set_for_process_group(self, process_group_identifier: ProcessGroupIdentifierChoice) -> Result<(), bool>
	{
		self.set(process_group_identifier, IOPRIO_WHO::IOPRIO_WHO_PGRP)
	}

	/// Returns `Err(true)` if no matching process or processes found.
	/// Returns `Err(false)` if permission denied.
	#[inline(always)]
	pub fn set_for_process(self, process_identifier: ProcessIdentifierChoice) -> Result<(), bool>
	{
		self.set(process_identifier, IOPRIO_WHO::IOPRIO_WHO_PROCESS)
	}

	/// Returns `Err(true)` if no matching process or processes found.
	/// Returns `Err(false)` if permission denied.
	#[inline(always)]
	pub fn set_for_thread(self, thread_identifier: ThreadIdentifier) -> Result<(), bool>
	{
		self.set(thread_identifier, IOPRIO_WHO::IOPRIO_WHO_PROCESS)
	}

	/// Returns `Err(true)` if no matching process or processes found.
	/// Returns `Err(false)` if permission denied.
	#[inline(always)]
	pub fn set_for_user(self, user_identifier: UserIdentifier) -> Result<(), bool>
	{
		self.set(user_identifier, IOPRIO_WHO::IOPRIO_WHO_USER)
	}

	#[inline(always)]
	fn set(self, which: impl Into<i32>, who: IOPRIO_WHO) -> Result<(), bool>
	{
		let value: u16 = self.into();
		Self::change(which, who, value)
	}

	/// Returns `Err(true)` if no matching process or processes found.
	/// Returns `Err(false)` if permission denied.
	#[inline(always)]
	pub fn reset_to_default_for_process_group(process_group_identifier: ProcessGroupIdentifierChoice) -> Result<(), bool>
	{
		Self::reset_to_default(process_group_identifier, IOPRIO_WHO::IOPRIO_WHO_PGRP)
	}

	/// Returns `Err(true)` if no matching process or processes found.
	/// Returns `Err(false)` if permission denied.
	#[inline(always)]
	pub fn reset_to_default_for_process(process_identifier: ProcessIdentifierChoice) -> Result<(), bool>
	{
		Self::reset_to_default(process_identifier, IOPRIO_WHO::IOPRIO_WHO_PROCESS)
	}

	/// Returns `Err(true)` if no matching process or processes found.
	/// Returns `Err(false)` if permission denied.
	#[inline(always)]
	pub fn reset_to_default_for_thread(thread_identifier: ThreadIdentifier) -> Result<(), bool>
	{
		Self::reset_to_default(thread_identifier, IOPRIO_WHO::IOPRIO_WHO_PROCESS)
	}

	/// Returns `Err(true)` if no matching process or processes found.
	/// Returns `Err(false)` if permission denied.
	#[inline(always)]
	pub fn reset_to_default_for_user(user_identifier: UserIdentifier) -> Result<(), bool>
	{
		Self::reset_to_default(user_identifier, IOPRIO_WHO::IOPRIO_WHO_USER)
	}

	#[inline(always)]
	fn reset_to_default(which: impl Into<i32>, who: IOPRIO_WHO) -> Result<(), bool>
	{
		Self::change(which, who, 0)
	}

	#[inline(always)]
	fn change(which: impl Into<i32>, who: IOPRIO_WHO, value: u16) -> Result<(), bool>
	{
		match system_call_ioprio_set(which.into(), who, value).as_usize()
		{
			0 => Ok(()),
			
			SystemCallResult::EPERM_usize => Err(true),
			SystemCallResult::ESRCH_usize => Err(false),
			SystemCallResult::EINVAL_usize => panic!("Invalid value for which"),
			unexpected_error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize ..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => unexpected_error!(ioprio_set, SystemCallResult::usize_to_system_call_error_number(unexpected_error)),
			
			unexpected @ _ => unexpected_result!(ioprio_get, unexpected),
		}
	}

	#[inline(always)]
	fn parse_ioprio_result(ioprio: i32) -> Result<Option<Self>, &'static str>
	{
		debug_assert!(ioprio >= 0);

		if unlikely!(ioprio > u16::MAX as i32)
		{
			return Err("ioprio does not fit in an u16")
		}
		Self::parse_ioprio(ioprio as u16)
	}

	#[inline(always)]
	fn parse_ioprio(ioprio: u16) -> Result<Option<Self>, &'static str>
	{
		use self::IoPriority::*;

		let scheduling_class = IOPRIO_PRIO_CLASS(ioprio) as u8;
		let scheduling_level  = IOPRIO_PRIO_DATA(ioprio);
		match scheduling_class
		{
			0 => if likely!(scheduling_level == 0)
			{
				Ok(None)
			}
			else
			{
				Err("IOPRIO_CLASS::IOPRIO_CLASS_NONE has a scheduling level")
			},

			1 => if likely!(scheduling_level <= 7)
			{
				Ok(Some(RealTime(unsafe { transmute(scheduling_level as u8) })))
			}
			else
			{
				Err("IOPRIO_CLASS::IOPRIO_CLASS_RT has a scheduling level greater than 7")
			},

			2 => if likely!(scheduling_level <= 7)
			{
				Ok(Some(BestEffort(unsafe { transmute(scheduling_level as u8) })))
			}
			else
			{
				Err("IOPRIO_CLASS::IOPRIO_CLASS_BE has a scheduling level greater than 7")
			},

			3 => if likely!(scheduling_level == 0)
			{
				Ok(Some(Idle))
			}
			else
			{
				Err("IOPRIO_CLASS::IOPRIO_CLASS_IDLE has a scheduling level")
			},

			_ => Err("Invalid IOPRIO_CLASS (scheduling class)")
		}
	}
}
