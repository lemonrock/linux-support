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
	AuditControl = capability::CAP_AUDIT_CONTROL as u8,
	AuditRead = capability::CAP_AUDIT_READ as u8,
	AuditWrite = capability::CAP_AUDIT_WRITE as u8,

	BlockSuspend = capability::CAP_BLOCK_SUSPEND as u8,

	Chown = capability::CAP_CHOWN as u8,
	DiscretionaryAccessControlBypass = capability::CAP_DAC_OVERRIDE as u8,
	DiscretionaryAccessControlFileReadBypass = capability::CAP_DAC_READ_SEARCH as u8,
	FileOwnerBypass = capability::CAP_FOWNER as u8,
	FileSetId = capability::CAP_FSETID as u8,

	LockMemory = capability::CAP_IPC_LOCK as u8,

	IpcOwner = capability::CAP_IPC_OWNER as u8,

	Kill = capability::CAP_KILL as u8,

	Lease = capability::CAP_LEASE as u8,

	Immutable = capability::CAP_LINUX_IMMUTABLE as u8,

	MandatoryAccessControlBypass = capability::CAP_MAC_ADMIN as u8,
	MandatoryAccessControlOverride = capability::CAP_MAC_OVERRIDE as u8,

	MakeNodes = capability::CAP_MKNOD as u8,

	SystemAdministration = capability::CAP_SYS_ADMIN as u8,
	NetworkAdministration = capability::CAP_NET_ADMIN as u8,
	BindPortsBelow1024 = capability::CAP_NET_BIND_SERVICE as u8,
	NetRaw = capability::CAP_NET_RAW as u8,

	SetUid = capability::CAP_SETUID as u8,
	SetGid = capability::CAP_SETGID as u8,

	SetFileCapabilities = capability::CAP_SETFCAP as u8,

	SetProcessCapabilities = capability::CAP_SETPCAP as u8,

	RebootAndKexecLoad = capability::CAP_SYS_BOOT as u8,

	Chroot = capability::CAP_SYS_CHROOT as u8,

	KernelModule = capability::CAP_SYS_MODULE as u8,

	Nice = capability::CAP_SYS_NICE as u8,

	ProcessAccounting = capability::CAP_SYS_PACCT as u8,

	PTrace = capability::CAP_SYS_PTRACE as u8,

	RawIO = capability::CAP_SYS_RAWIO as u8,

	Resource = capability::CAP_SYS_RESOURCE as u8,

	Time = capability::CAP_SYS_TIME as u8,

	TtyConfig = capability::CAP_SYS_TTY_CONFIG as u8,

	Syslog = capability::CAP_SYSLOG as u8,

	WakeAlarm = capability::CAP_WAKE_ALARM as u8,
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
	const LinuxMaximum: u16 = capability::CAP_LAST_CAP as u16;

	const InclusiveMinimum: Self = unsafe { transmute(0u8) };

	const InclusiveMaximum: Self = unsafe { transmute(Self::LinuxMaximum as u8) };

	#[inline(always)]
	fn from_validated_u16(value: u16) -> Self
	{
		debug_assert!(value < Self::LinuxMaximum);

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
		let result = unsafe { prctl(PR_CAP_AMBIENT, PR_CAP_AMBIENT_RAISE, self as c_ulong, 0, 0) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			use self::AmbientCapabilityError::*;
			match errno().0
			{
				EPERM => Err(PermissionDenied),
				EINVAL => Err(CapabilityNotKnownByThisLinuxKernel),

				unexpected @ _ => panic!("Unexpected error code '{}' from prctl()", unexpected),
			}
		}
		else
		{
			unreachable!("prctl() failed with unexpected result {}", result)
		}
	}

	/// Removes capability from current thread's ambient set.
	///
	/// The specified capability must already be present in both the permitted and the inheritable sets of the current thread.
	/// Not permitted if the thread has the securebit `SECBIT_NO_CAP_AMBIENT_RAISE`.
	#[inline(always)]
	pub fn remove_from_current_thread_ambient_set(self) -> Result<(), AmbientCapabilityError>
	{
		let result = unsafe { prctl(PR_CAP_AMBIENT, PR_CAP_AMBIENT_LOWER, self as c_ulong, 0, 0) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			use self::AmbientCapabilityError::*;
			match errno().0
			{
				EPERM => Err(PermissionDenied),
				EINVAL => Err(CapabilityNotKnownByThisLinuxKernel),

				unexpected @ _ => panic!("Unexpected error code '{}' from prctl()", unexpected),
			}
		}
		else
		{
			unreachable!("prctl() failed with unexpected result {}", result)
		}
	}

	/// Does the current thread have this capability in its ambient set?
	///
	/// Error can only ever be `AmbientCapabilityError::CapabilityNotKnownByThisLinuxKernel`.
	#[inline(always)]
	pub fn is_in_current_thread_ambient_set(self) -> Result<bool, AmbientCapabilityError>
	{
		match unsafe { prctl(PR_CAP_AMBIENT, PR_CAP_AMBIENT_IS_SET, self as c_ulong, 0, 0) }
		{
			1 => Ok(true),

			0 => Ok(false),

			-1 => match errno().0
			{
				EINVAL => Err(AmbientCapabilityError::CapabilityNotKnownByThisLinuxKernel),

				illegal @ _ => panic!("Illegal error code '{}' from prctl()", illegal),
			},

			illegal @ _ => panic!("prctl() returned illegal result '{}'", illegal),
		}
	}

	/// Does the current thread have this capability in its bounding set?
	///
	/// Returns None if this capability isn't recognised by the Linux kernel.
	#[inline(always)]
	pub fn is_in_current_thread_bounding_set(self) -> Option<bool>
	{
		match unsafe { prctl(PR_CAPBSET_READ, self as c_ulong) }
		{
			1 => Some(true),

			0 => Some(false),

			-1 => match errno().0
			{
				EINVAL => None,

				illegal @ _ => panic!("Illegal error code '{}' from prctl()", illegal),
			},

			illegal @ _ => panic!("prctl() returned illegal result '{}'", illegal),
		}
	}

	/// Returns `Err` if a lack-of-permissions error occurred.
	///
	/// This is because the current thread does not have the capability `CAP_SETPCAP` in its `Bounding` set.
	#[inline(always)]
	pub fn remove_from_current_thread_bounding_set(self) -> Result<(), ()>
	{
		match unsafe { prctl(PR_CAPBSET_DROP, self as c_ulong) }
		{
			0 => Ok(()),

			-1 => match errno().0
			{
				EPERM => Err(()),
				EINVAL => panic!("Kernel does not support 'file' capabilities. Or capability `{:?}` is not a valid capability on this kernel", self),

				illegal @ _ => panic!("Illegal error code '{}' from prctl()", illegal),
			},

			illegal @ _ => panic!("prctl() returned illegal result '{}'", illegal),
		}
	}
}
