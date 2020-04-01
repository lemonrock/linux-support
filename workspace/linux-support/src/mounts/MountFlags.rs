// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	// Flags commented '// ' are special

	/// Mount flags.
	#[allow(missing_docs)]
	#[derive(Deserialize, Serialize)]
	pub struct MountFlags: u64
	{
		///
		const BindMount = MS_BIND;

		///
		const DirectoryChangesAreSynchronous = MS_DIRSYNC;

		///
		const PermitMandatoryLocking = MS_MANDLOCK;

		///
		const Move = MS_MOVE;

		///
		const DoNotUpdateAccessTimes = MS_NOATIME;

		///
		const DoNotAllowDeviceFiles = MS_NODEV;

		/// Implicit if `DoNotUpdateAccessTimes` is specified.
		const DoNotUpdateDirectoryAccessTimes = MS_NODIRATIME;

		///
		const DoNotAllowProgramsToBeExecuted = MS_NOEXEC;

		///
		const DoNotHonourSetUidAndSetGidPermissions = MS_NOSUID;

		// const PRIVATE = MS_PRIVATE;

		///
		const RecursiveBindMount = MS_REC;

		///
		const RelaxedAccessTimeUpdates = MS_RELATIME;

		// const REMOUNT = MS_REMOUNT;

		// const SHARED = MS_SHARED;

		///
		const SilenceSomeKernelWarningMessages = MS_SILENT;

		// const SLAVE = MS_SLAVE;

	 	/// Overrides `DoNotUpdateAccessTimes` and `DoNotUpdateDirectoryAccessTimes`.
		const AlwaysUpdateTheLastAccessTime = MS_STRICTATIME;

		///
		const FileWritesAreSynchronous = MS_SYNCHRONOUS;

		// const UNBINDABLE = MS_UNBINDABLE;

		// MS_RDONLY
		// MS_LAZYTIME
	}
}
