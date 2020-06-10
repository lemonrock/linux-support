// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Back log can not exceed `i32::MAX`.
///
/// Back log is constrained globally by the maximum in `/proc/sys/net/core/somaxconn`.
/// The default in this file is `128`; this is also the constant `SOMAXCONN`.
///
/// Old, but informative, advice adapted from <https://tangentsoft.net/wskfaq/advanced.html#backlog>:-
///
/// "When a connection request comes into a network stack, it first checks to see if any program is listening on the requested port.
/// If so, the stack replies to the remote peer, completing the connection.
/// The stack stores the connection information in a queue called the connection backlog.
/// (When there are connections in the backlog, the accept() call simply causes the stack to remove the oldest connection from the connection backlog and return a socket for it).
///
/// One of the parameters to the listen() call sets the size of the connection backlog for a particular socket.
/// When the backlog fills up, the stack begins rejecting connection attempts.
///
/// Rejecting connections is a good thing if your program is written to accept new connections as fast as it reasonably can.
/// If the backlog fills up despite your program’s best efforts, it means your server has hit its load limit.
/// If the stack were to accept more connections, your program wouldn’t be able to handle them as well as it should, so the client will think your server is hanging.
/// At least if the connection is rejected, the client will know the server is too busy and will try again later.
///
/// The proper value for listen()’s backlog parameter depends on how many connections you expect to see in the time between accept() calls.
/// Let’s say you expect an average of 1000 connections per second, with a burst value of 3000 connections per second.
/// (I picked these values because they’re easy to manipulate, not because they’re representative of the real world)!
/// To handle the burst load with a short connection backlog, your server’s time between accept() calls must be under 0.3 milliseconds.
/// Let’s say you’ve measured your time-to-accept under load, and it’s 0.8 milliseconds: fast enough to handle the normal load, but too slow to handle your burst value.
/// In this case, you could make backlog relatively large to let the stack queue up connections under burst conditions.
/// Assuming that these bursts are short, your program will quickly catch up and clear out the connection backlog.
///	///
/// If your program is quick about calling accept(), low backlog limits are not normally a problem.
/// However, it does mean that concerted attempts to make lots of connections in a short period of time can fill the backlog queue. This makes low backlog values a bad choice for a high-load server: either a legitimate load or a SYN flood attack can overload a server on such a platform.
///
/// Beware that large backlogs make SYN flood attacks much more, shall we say, effective.
/// (In Windows NT: since the backlog queue is in non-pageable system memory, a SYN flood can cause the queue to eat a lot of this precious memory resource).
///
/// You will note that SYN attacks are dangerous for systems with both very short and very long backlog queues.
/// The point is that a middle ground is the best course if you expect your server to withstand SYN attacks.
//
/// A program can rely too much on the backlog feature.
/// Consider a single-threaded blocking server: the design means it can only handle one connection at a time
/// However, it can set up a large backlog, making the stack accept and hold connections until the program gets around to handling the next one.
/// You should not take advantage of the feature this way unless your connection rate is very low and the connection times are very short.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct BackLog(u32);

impl From<u16> for BackLog
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		Self::from_u16(value)
	}
}

impl TryFrom<u32> for BackLog
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
			Ok(Self(value as u32))
		}
	}
}

impl Default for BackLog
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::UsualMaximum
	}
}

impl BackLog
{
	/// `SOMAXCONN`.
	pub const UsualMaximum: Self = Self(128);
	
	/// Inclusive minimum.
	pub const InclusiveMinimum: Self = Self(0);
	
	/// Inclusive maximum.
	pub const InclusiveMaximum: Self = Self(i32::MAX as u32);
	
	/// Safe construction.
	#[inline(always)]
	pub const fn from_u16(value: u16) -> Self
	{
		Self(value as u32)
	}
	
	/// Value of `/proc/sys/net/core/somaxconn`.
	#[inline(always)]
	pub fn global_maximum(proc_path: &ProcPath) -> Self
	{
		Self(Self::sys_net_core_somaxconn_file_path(proc_path).read_value().unwrap())
	}
	
	/// Set value of `/proc/sys/net/core/somaxconn` if it exists.
	#[inline(always)]
	pub fn set_global_maximum(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/core/somaxconn");
		
		let file_path = Self::sys_net_core_somaxconn_file_path(proc_path);
		
		if file_path.exists()
		{
			file_path.write_value(UnpaddedDecimalInteger(self.0))
		}
		else
		{
			Ok(())
		}
	}
	
	/// Usually 128 (identical to `SOMAXCONN`).
	pub const UsualSynMaximum: Self = Self(128);
	
	/// Value of `/proc/sys/net/ipv4/tcp_max_syn_backlog`.
	#[inline(always)]
	pub fn global_maximum_syn(proc_path: &ProcPath) -> Self
	{
		Self(Self::sys_net_ipv4_tcp_max_syn_backlog_file_path(proc_path).read_value().unwrap())
	}
	
	/// Set value of `/proc/sys/net/ipv4/tcp_max_syn_backlog` if it exists.
	#[inline(always)]
	pub fn set_global_maximum_syn(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/ipv4/tcp_max_syn_backlog");
		
		let file_path = Self::sys_net_ipv4_tcp_max_syn_backlog_file_path(proc_path);
		
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
	fn sys_net_core_somaxconn_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_core_file_path("somaxconn")
	}
	
	#[inline(always)]
	fn sys_net_ipv4_tcp_max_syn_backlog_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_core_file_path("tcp_max_syn_backlog")
	}
}
