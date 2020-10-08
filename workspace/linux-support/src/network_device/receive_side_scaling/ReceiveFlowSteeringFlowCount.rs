// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Number of flows steered to any one `HyperThread` by Receive Packet Steering (RPS)'s Receive Flow Steering (RFS).
///
/// RFS keeps track of a global hash table of all flows.
///
/// Inclusive maximum is `1<<29`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ReceiveFlowSteeringFlowCount(u32);

impl From<u16> for ReceiveFlowSteeringFlowCount
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		Self::from_u16(value)
	}
}

impl TryFrom<u32> for ReceiveFlowSteeringFlowCount
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		if unlikely!(value > Self::InclusiveMaximum.0)
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl Div<QueueCount> for ReceiveFlowSteeringFlowCount
{
	type Output = usize;
	
	#[inline(always)]
	fn div(self, rhs: QueueCount) -> Self::Output
	{
		let queue_count: usize = rhs.into();
		(self.0 as usize) / queue_count
	}
}

impl ReceiveFlowSteeringFlowCount
{
	/// Typical default.
	///
	/// From `/proc/sys/net/core/rps_sock_flow_entries`.
	pub const UsualDefaultGlobalMaximum: Self = Self(0);
	
	/// Maximum.
	pub const InclusiveMaximum: Self = Self(1 << 29);
	
	/// Safe construction.
	#[inline(always)]
	pub fn capped(value: usize) -> Self
	{
		if value > (u32::MAX as usize)
		{
			return Self::InclusiveMaximum
		}
		let value = value as u32;
		if value > Self::InclusiveMaximum.0
		{
			Self::InclusiveMaximum
		}
		else
		{
			Self(value)
		}
	}
	
	/// Safe construction.
	#[inline(always)]
	pub const fn from_u16(value: u16) -> Self
	{
		Self(value as u32)
	}
	
	/// Value of `/proc/sys/net/core/rps_sock_flow_entries`.
	///
	/// Rounded up to power of 2.
	#[inline(always)]
	pub fn global_maximum(proc_path: &ProcPath) -> io::Result<Self>
	{
		Self::sys_net_core_rps_sock_flow_entries_file_path(proc_path).read_value().map(Self)
	}
	
	/// Set value of `/proc/sys/net/core/rps_sock_flow_entries` if it exists.
	///
	/// Rounded up to power of 2.
	#[inline(always)]
	pub fn set_global_maximum(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/core/rps_sock_flow_entries");
		
		let file_path = Self::sys_net_core_rps_sock_flow_entries_file_path(proc_path);
		
		if file_path.exists()
		{
			file_path.write_value(UnpaddedDecimalInteger(self.0))
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	pub(crate) fn per_receive_queue_receive_flow_steering_table_count(self, receive_queue_count: QueueCount) -> usize
	{
		assert_ne!(self, Self(0), "Receive Flow Steering (RFS) must have a flow count to be used per-receive-queue");
		
		self / receive_queue_count
	}
	
	#[inline(always)]
	fn sys_net_core_rps_sock_flow_entries_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_core_file_path("rps_sock_flow_entries")
	}
}
