// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// In a system with the `_POSIX_CHOWN_RESTRICTED` option defined, this overrides the restriction of changing file ownership and group ownership.
pub(super) const CAP_CHOWN: u8 = 0;

/// Override all DAC access, including ACL execute access if `_POSIX_ACL` is defined.
///
/// Excluding DAC access covered by `CAP_LINUX_IMMUTABLE`.
pub(super) const CAP_DAC_OVERRIDE: u8 = 1;

/// Overrides all DAC restrictions regarding read and search on files and directories, including ACL restrictions if `_POSIX_ACL` is defined.
///
/// Excluding DAC access covered by `CAP_LINUX_IMMUTABLE`.
pub(super) const CAP_DAC_READ_SEARCH: u8 = 2;

/// Overrides all restrictions about allowed operations on files, where file owner ID must be equal to the user ID, except where `CAP_FSETID` is applicable.
///
/// It doesn't override MAC and DAC restrictions.
pub(super) const CAP_FOWNER: u8 = 3;

/// Overrides the following restrictions that the effective user ID shall match the file owner ID when setting the `S_ISUID` and `S_ISGID` bits on that file; that the effective group ID (or one of the supplementary group IDs) shall match the file owner ID when setting the `S_ISGID` bit on that file; that the `S_ISUID` and `S_ISGID` bits are cleared on successful return from `chown()` (not implemented).
pub(super) const CAP_FSETID: u8 = 4;

/// Overrides the restriction that the real or effective user ID of a process sending a signal must match the real or effective user ID of the process receiving the signal.
pub(super) const CAP_KILL: u8 = 5;

/// Allows setgid(2) manipulation.
/// Allows setgroups(2).
/// Allows forged gids on socket credentials passing.
pub(super) const CAP_SETGID: u8 = 6;

/// Allows set*uid(2) manipulation (including fsuid).
/// Allows forged pids on socket credentials passing.
pub(super) const CAP_SETUID: u8 = 7;


/// Without VFS support for capabilities: transfer any capability in your permitted set to any pid, remove any capability in your permitted set from any pid.
/// With VFS support for capabilities (neither of above, but).
/// Add any capability from current's capability bounding set to the current process' inheritable set.
/// Allow taking bits out of capability bounding set.
/// Allow modification of the securebits for a process.
pub(super) const CAP_SETPCAP: u8 = 8;

/// Allow modification of S_IMMUTABLE and S_APPEND file attributes.
pub(super) const CAP_LINUX_IMMUTABLE: u8 = 9;

/// Allows binding to TCP/UDP sockets below 1024.
/// Allows binding to ATM VCIs below 32.
pub(super) const CAP_NET_BIND_SERVICE: u8 = 10;

/// Allow broadcasting, listen to multicast.
pub(super) const CAP_NET_BROADCAST: u8 = 11;

/// Allow interface configuration.
/// Allow administration of IP firewall, masquerading and accounting.
/// Allow setting debug option on sockets.
/// Allow modification of routing tables.
/// Allow setting arbitrary process / process group ownership on sockets.
/// Allow binding to any address for transparent proxying (also via NET_RAW).
/// Allow setting TOS (type of service).
/// Allow setting promiscuous mode.
/// Allow clearing driver statistics.
/// Allow multicasting.
/// Allow read/write of device-specific registers.
/// Allow activation of ATM control sockets.
pub(super) const CAP_NET_ADMIN: u8 = 12;

/// Allow use of RAW sockets.
/// Allow use of PACKET sockets.
/// Allow binding to any address for transparent proxying (also via NET_ADMIN).
pub(super) const CAP_NET_RAW: u8 = 13;

/// Allow locking of shared memory segments.
/// Allow mlock and mlockall (which doesn't really have anything to do with IPC).
pub(super) const CAP_IPC_LOCK: u8 = 14;

/// Override IPC ownership checks.
pub(super) const CAP_IPC_OWNER: u8 = 15;

/// Insert and remove kernel modules - modify kernel without limit.
pub(super) const CAP_SYS_MODULE: u8 = 16;

/// Allow ioperm/iopl access.
/// Allow sending USB messages to any device via `/dev/bus/usb`.
pub(super) const CAP_SYS_RAWIO: u8 = 17;

/// Allow use of `chroot()`.
pub(super) const CAP_SYS_CHROOT: u8 = 18;

/// Allow `ptrace()` of any process.
pub(super) const CAP_SYS_PTRACE: u8 = 19;

/// Allow configuration of process accounting.
pub(super) const CAP_SYS_PACCT: u8 = 20;

/// Allow configuration of the secure attention key.
/// Allow administration of the random device.
/// Allow examination and configuration of disk quotas.
/// Allow setting the domainname.
/// Allow setting the hostname.
/// Allow calling `bdflush()`.
/// Allow `mount()` and `umount()`, setting up new SMB connection.
/// Allow some autofs root ioctls.
/// Allow `nfsservctl`.
/// Allow `VM86_REQUEST_IRQ`.
/// Allow to read/write pci config on alpha.
/// Allow `irix_prctl` on MIPS (`setstacksize`).
/// Allow flushing all cache on m68k (`sys_cacheflush`).
/// Allow removing semaphores.
/// Used instead of `CAP_CHOWN` to "chown" IPC message queues, semaphores and shared memory.
/// Allow locking/unlocking of shared memory segment.
/// Allow turning swap on/off.
/// Allow forged pids on socket credentials passing.
/// Allow setting readahead and flushing buffers on block devices.
/// Allow setting geometry in floppy driver.
/// Allow turning DMA on/off in xd driver.
/// Allow administration of md devices (mostly the above, but some extra ioctls).
/// Allow tuning the ide driver.
/// Allow access to the nvram device.
/// Allow administration of apm_bios, serial and bttv (TV) device.
/// Allow manufacturer commands in isdn CAPI support driver.
/// Allow reading non-standardized portions of pci configuration space.
/// Allow DDI debug ioctl on sbpcd driver.
/// Allow setting up serial ports.
/// Allow sending raw qic-117 commands.
/// Allow enabling/disabling tagged queuing on SCSI controllers and sending arbitrary SCSI commands.
/// Allow setting encryption key on loopback filesystem.
/// Allow setting zone reclaim policy.
pub(super) const CAP_SYS_ADMIN: u8 = 21;

/// Allow use of `reboot()`.
pub(super) const CAP_SYS_BOOT: u8 = 22;

/// Allow raising priority and setting priority on other (different UID) processes.
/// Allow use of FIFO and round-robin (realtime) scheduling on own processes and setting the scheduling algorithm used by another process.
/// Allow setting cpu affinity on other processes.
pub(super) const CAP_SYS_NICE: u8 = 23;

/// Override resource limits. Set resource limits.
/// Override quota limits.
/// Override reserved space on ext2 filesystem.
/// Modify data journaling mode on ext3 filesystem (uses journaling resources).
/// NOTE: ext2 honors fsuid when checking for resource overrides, so you can override using fsuid too.
/// Override size restrictions on IPC message queues.
/// Allow more than 64hz interrupts from the real-time clock.
/// Override max number of consoles on console allocation.
/// Override max number of keymaps.
/// Control memory reclaim behavior.
pub(super) const CAP_SYS_RESOURCE: u8 = 24;

/// Allow manipulation of system clock.
/// Allow irix_stime on mips.
/// Allow setting the real-time clock.
pub(super) const CAP_SYS_TIME: u8 = 25;

/// Allow configuration of tty devices.
/// Allow `vhangup()` of tty.
pub(super) const CAP_SYS_TTY_CONFIG: u8 = 26;

/// Allow the privileged aspects of `mknod()`.
pub(super) const CAP_MKNOD: u8 = 27;

/// Allow taking of leases on files.
pub(super) const CAP_LEASE: u8 = 28;

/// Allow writing the audit log via unicast netlink socket.
pub(super) const CAP_AUDIT_WRITE: u8 = 29;

/// Allow configuration of audit via unicast netlink socket.
pub(super) const CAP_AUDIT_CONTROL: u8 = 30;

pub(super) const CAP_SETFCAP: u8 = 31;

/// Override MAC access.
/// The base kernel enforces no MAC policy.
// An LSM may enforce a MAC policy, and if it does and it chooses to implement capability based overrides of that policy, this is the capability it should use to do so.
pub(super) const CAP_MAC_OVERRIDE: u8 = 32;

/// Allow MAC configuration or state changes.
/// The base kernel requires no MAC configuration.
/// An LSM may enforce a MAC policy, and if it does and it chooses to implement capability based checks on modifications to that policy or the data required to maintain it, this is the capability it should use to do so.
pub(super) const CAP_MAC_ADMIN: u8 = 33;

/// Allow configuring the kernel's syslog (printk behaviour).
pub(super) const CAP_SYSLOG: u8 = 34;

/// Allow triggering something that will wake the system.
pub(super) const CAP_WAKE_ALARM: u8 = 35;

/// Allow preventing system suspends.
pub(super) const CAP_BLOCK_SUSPEND: u8 = 36;

/// Allow reading the audit log via multicast netlink socket.
pub(super) const CAP_AUDIT_READ: u8 = 37;

pub(super) const CAP_LAST_CAP: u8 = CAP_AUDIT_READ;
