// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Linux Security Capabilities.
///
/// These are per-thread and per-file.
///
/// Any thread created inherits the Thread capability sets.
///
/// File capabilities and thread capabilities intertwine in a very complex way, but, in essence, are only of interest if executing third-party programs that need file capabilities (eg `ping`).
///
/// Get bounding sets by parsing `/proc/<pid>/status` (or the equivalent for a thread).
///
/// # File capability sets
///
/// Used when `execve()` is used.
///
///
/// ## Permitted
///
/// * These capabilities are automatically permitted to the thread, regardless of the thread's `Inheritable` set.
/// * Formerly known as `forced`.
///
///
/// ## Inheritable
///
/// * These capabilities are `ANDed` with the thread's `Inheritable` set then this set is `ORed` with the file's `Permitted` set to create the thread's `Permitted` set after `execve()`.
/// * Formerly known as `allowed`.
///
///
/// ## Effective Bit.
///
/// * If set, then any capabilities acquired on `execve()` are added to the `Effective` set.
///
///
/// # Thread capability sets
///
///
/// ## Permitted
///
/// * This is the superset for the effective capabilities that the thread may assume.
/// * This is the superset for the inheritable capabilities that the thread may have *unless* it has the `CAP_SETPCAP` capability in its effective set.
/// * Once a thread drops a capabilities from its permitted set, it can never regain it *unless* it `execve()`s a program with either:-
///		* file capabilities set
/// 	* setuid/segid bits set.
///
///
/// ## Effective
///
/// * This is the set of capabilities used by the kernel to perform permission checks for the thread.
/// * It is a subset of `Permitted`.
///
///
/// ## Inheritable
///
/// * This is a set of capabilities it is deseired to be preserved across an `execve()`.
/// * They are `ANDed` with the file capability's `Inheritable` set then `ORed` with the file capability's `Permitted` set to create the resultant `Permitted` set for a thread after `execve()`.
/// * Ordinarily, non-root programs lose their capabilities on `execve()` unless using `Ambient` capabilities.
///
///
/// ## Bounding
///
/// * The capability bounding set is a mechanism that can be used to limit the capabilities that are gained during `execve()`.
/// * Before Linux 2.6.25, this was a system-wide and not thread-specific set.
///
///
/// ## Ambient
///
/// * This is a set of capabilities that are preserved across an `execve()` of a program *unless* it `execve()`s a program with either:-
///		* file capabilities set
/// 	* setuid/segid bits set.
/// * In which case the new capabilities are applied.
/// * No capability can never be ambient if it is not both:-
///		* `Permitted`
///		* `Inheritable`.
/// * A capability is automatically removed this set if it is removed:-
/// 	* `Permitted`
/// 	* `Inheritable`
#[allow(missing_docs)]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[derive(EnumIter)]
#[serde(deny_unknown_fields)]
pub enum Capability
{
	AuditControl = CAP_AUDIT_CONTROL,
	
	AuditRead = CAP_AUDIT_READ,
	
	AuditWrite = CAP_AUDIT_WRITE,

	BlockSuspend = CAP_BLOCK_SUSPEND,
	
	Broadcast = CAP_NET_BROADCAST,

	Chown = CAP_CHOWN,
	
	DiscretionaryAccessControlBypass = CAP_DAC_OVERRIDE,
	
	DiscretionaryAccessControlFileReadBypass = CAP_DAC_READ_SEARCH,
	
	FileOwnerBypass = CAP_FOWNER,
	
	FileSetId = CAP_FSETID,

	LockMemory = CAP_IPC_LOCK,

	IpcOwner = CAP_IPC_OWNER,

	Kill = CAP_KILL,

	Lease = CAP_LEASE,

	Immutable = CAP_LINUX_IMMUTABLE,

	MandatoryAccessControlBypass = CAP_MAC_ADMIN,
	
	MandatoryAccessControlOverride = CAP_MAC_OVERRIDE,

	MakeNodes = CAP_MKNOD,

	SystemAdministration = CAP_SYS_ADMIN,
	
	NetworkAdministration = CAP_NET_ADMIN,
	
	BindPortsBelow1024 = CAP_NET_BIND_SERVICE,
	
	NetRaw = CAP_NET_RAW,

	SetUid = CAP_SETUID,
	
	SetGid = CAP_SETGID,

	SetFileCapabilities = CAP_SETFCAP,

	SetProcessCapabilities = CAP_SETPCAP,

	RebootAndKexecLoad = CAP_SYS_BOOT,

	Chroot = CAP_SYS_CHROOT,

	KernelModule = CAP_SYS_MODULE,

	Nice = CAP_SYS_NICE,

	ProcessAccounting = CAP_SYS_PACCT,

	PTrace = CAP_SYS_PTRACE,

	RawIO = CAP_SYS_RAWIO,

	Resource = CAP_SYS_RESOURCE,

	Time = CAP_SYS_TIME,

	TtyConfig = CAP_SYS_TTY_CONFIG,

	Syslog = CAP_SYSLOG,

	WakeAlarm = CAP_WAKE_ALARM,
}

bit_set_aware!(Capability);

impl Into<u16> for Capability
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self as u8 as u16
	}
}

impl BitSetAware for Capability
{
	const LinuxMaximum: u32 = CAP_LAST_CAP as u32;

	const InclusiveMinimum: Self = unsafe { transmute(0u8) };

	const InclusiveMaximum: Self = unsafe { transmute(Self::LinuxMaximum as u8) };

	#[inline(always)]
	fn from_validated_u16(value: u16) -> Self
	{
		debug_assert!((value as u32) < Self::LinuxMaximum);

		unsafe { transmute(value as u8) }
	}
}

impl Capability
{
	/// Adds capability to current thread's ambient set.
	///
	/// The specified capability must already be present in both the permitted and the inheritable sets of the current thread.
	/// Not permitted if the thread has the securebit `SECBIT_NO_CAP_AMBIENT_RAISE`.
	#[inline(always)]
	pub fn add_to_current_thread_ambient_set(self) -> Result<(), AmbientCapabilityError>
	{
		use self::AmbientCapabilityError::*;
		
		process_control_wrapper3
		(
			PR_CAP_AMBIENT,
			PR_CAP_AMBIENT_RAISE as usize,
			self as usize,
			result_must_be_zero,
			|error_number| match error_number.0
			{
				EPERM => Err(PermissionDenied),
				EINVAL => Err(CapabilityNotKnownByThisLinuxKernel),
				
				unexpected @ _ => panic!("Unexpected error code '{}' from prctl()", unexpected),
			}
		)
	}

	/// Removes capability from current thread's ambient set.
	///
	/// The specified capability must already be present in both the permitted and the inheritable sets of the current thread.
	/// Not permitted if the thread has the securebit `SECBIT_NO_CAP_AMBIENT_RAISE`.
	#[inline(always)]
	pub fn remove_from_current_thread_ambient_set(self) -> Result<(), AmbientCapabilityError>
	{
		use self::AmbientCapabilityError::*;
		
		process_control_wrapper3
		(
			PR_CAP_AMBIENT,
			PR_CAP_AMBIENT_LOWER as usize,
			self as usize,
			result_must_be_zero,
			|error_number| match error_number.0
			{
				EPERM => Err(PermissionDenied),
				EINVAL => Err(CapabilityNotKnownByThisLinuxKernel),
				
				unexpected @ _ => panic!("Unexpected error code '{}' from prctl()", unexpected),
			}
		)
	}

	/// Does the current thread have this capability in its ambient set?
	///
	/// Error can only ever be `AmbientCapabilityError::CapabilityNotKnownByThisLinuxKernel`.
	#[inline(always)]
	pub fn is_in_current_thread_ambient_set(self) -> Result<bool, AmbientCapabilityError>
	{
		use self::AmbientCapabilityError::*;
		
		process_control_wrapper3
		(
			PR_CAP_AMBIENT,
			PR_CAP_AMBIENT_IS_SET as usize,
			self as usize,
			|non_negative_result| match non_negative_result
			{
				0 => Ok(false),
				1 => Ok(true),
				_ => unreachable_code(format_args!("Non-boolean result from `prctl()`"))
			},
			|error_number| match error_number.0
			{
				EINVAL => Err(CapabilityNotKnownByThisLinuxKernel),
				
				unexpected @ _ => panic!("Unexpected error code '{}' from prctl()", unexpected),
			}
		)
	}

	/// Does the current thread have this capability in its bounding set?
	///
	/// Returns None if this capability isn't recognised by the Linux kernel.
	#[inline(always)]
	pub fn is_in_current_thread_bounding_set(self) -> Option<bool>
	{
		let result: Result<Option<bool>, io::Error> = process_control_wrapper2
		(
			PR_CAPBSET_READ,
			self as usize,
			|non_negative_result| match non_negative_result
			{
				0 => Ok(Some(false)),
				1 => Ok(Some(true)),
				_ => unreachable_code(format_args!("Non-boolean result from `prctl()`"))
			},
			|error_number| match error_number.0
			{
				EINVAL => Ok(None),
				
				unexpected @ _ => panic!("Unexpected error code '{}' from prctl()", unexpected),
			}
		);
		result.unwrap()
	}

	/// Returns `Err` if a lack-of-permissions error occurred.
	///
	/// This is because the current thread does not have the capability `CAP_SETPCAP` in its `Bounding` set.
	#[inline(always)]
	pub fn remove_from_current_thread_bounding_set(self) -> Result<(), ()>
	{
		process_control_wrapper2
		(
			PR_CAPBSET_DROP,
			self as usize,
			result_must_be_zero,
			|error_number| match error_number.0
			{
				EPERM => Err(()),
				EINVAL => panic!("Kernel does not support 'file' capabilities. Or capability `{:?}` is not a valid capability on this kernel", self),
				
				unexpected @ _ => panic!("Unexpected error code '{}' from prctl()", unexpected),
			}
		)
	}
}
