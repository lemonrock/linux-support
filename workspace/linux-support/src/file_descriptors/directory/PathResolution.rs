// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// How to resolve a path when opening relative to a directory.
	///
	/// The primary use case for these flags is to allow trusted programs to restrict how untrusted paths (or paths inside untrusted directories) are resolved.
	pub struct PathResolution: u64
	{
		/// Do not permit the path resolution to succeed if any component of the resolution is not a descendant of this directory.
		/// This causes absolute symbolic links (and absolute values of pathname) to be rejected.
		///
		/// Currently, this flag also disables magic-link resolution (see `NoMagicLinks` below).
		/// However, this may change in the future.
		/// Therefore, to ensure that magic links are not resolved, the caller should explicitly specify `NoMagicLinks`.
		///
		/// Equivalent to `RESOLVE_BENEATH`.
		const Beneath = 0x08;

		/// Treat the directory referred to by this directory as the root directory while resolving pathname.
		///
		/// Absolute symbolic links are interpreted relative to the directory.
		/// If a prefix component of pathname equates to dirfd, then an immediately following `..` component likewise equates to this directory (just as `/..` is traditionally equivalent to `/`).
		/// If pathname is an absolute path, it is also interpreted relative to this directory.
		///
		/// The effect of this flag is as though the calling process had used `chroot()` to (temporarily) modify its root directory (to the directory referred to by this directory).
		/// However, unlike `chroot()` (which changes the filesystem root permanently for a process), `InRoot` allows a program to efficiently restrict path resolution on a per-open basis.
		/// The `InRoot` flag also has several hardening features (such as detecting escape attempts during `.`. resolution) which `chroot()` does not.
		///
		/// Currently, this flag also disables magic-link resolution (see `NoMagicLinks` below).
		/// However, this may change in the future.
		/// Therefore, to ensure that magic links are not resolved, the caller should explicitly specify `NoMagicLinks`.
		///
		/// Equivalent to `RESOLVE_IN_ROOT`.
		const InRoot = 0x10;

		/// Disallow all magic-link resolution during path resolution.
		///
		/// If the trailing component (ie `basename`) of pathname is a magic link, and `openat() flags` contains both `O_PATH` and `O_NOFOLLOW`, then an `O_PATH` file descriptor referencing the magic link will be returned.
		///
		/// Magic links are symbolic link-like objects that are most notably found in `/proc` (examples include `/proc/[pid]/exe` and `/proc/[pid]/fd/*`).
		///
		/// Due to the potential danger of unknowingly opening these magic links, it may be preferable for users to disable their resolution entirely.
		///
		/// Equivalent to `RESOLVE_NO_MAGICLINKS`.
		const NoMagicLinks = 0x02;

		/// Disallow resolution of symbolic links during path resolution.
		///
		/// If the trailing component (ie `basename`) of pathname is a symbolic link, and `openat() flags` contains both `O_PATH` and `O_NOFOLLOW`, then an `O_PATH` file descriptor referencing the symbolic link will be returned.
		///
		/// Note that the effect of this flag, which affects the treatment of symbolic links in all of the components of pathname, differs from the effect of the `O_NOFOLLOW` file creation flag , which affects the handling of symbolic links only in the final component of pathname.
		///
		/// Applications that employ this flag are encouraged to make its use configurable (unless it is used for a specific security purpose), as symbolic links are very widely used by end-users.
		/// Setting this flag indiscriminately for all uses of `openat()` may result in spurious errors on previously-functional systems.
		///
		/// Currently, this flag also disables magic-link resolution (see `NoMagicLinks` below).
		/// However, this may change in the future.
		/// Therefore, to ensure that magic links are not resolved, the caller should explicitly specify `NoMagicLinks`.
		///
		/// Equivalent to `RESOLVE_NO_SYMLINKS`.
		const NoSymlinks = 0x04;

		/// Disallow traversal of mount points during path resolution (including all bind mounts).
		///
		/// Consequently, pathname must be on the same mount as the directory referred to by this directory.
		///
		/// Applications that use this flags are encouraged to make its use configurable (unless it is used for a specific security purpose), as bind mounts are widely used by end-users.
		/// Setting this flag indiscriminately, eg for purposes not specifically related to security—for all uses of `openat()`, may result in spurious errors on previously-functional systems.
		/// This may occur if, for example, a system pathname that is used by an application is modified (eeg, in a new distribution release) so that a pathname component (now) contains a bind mount.
		///
		/// Equivalent to `RESOLVE_NO_XDEV`.
		const DisallowTraversalOfMountPoints = 0x01;
	}
}
