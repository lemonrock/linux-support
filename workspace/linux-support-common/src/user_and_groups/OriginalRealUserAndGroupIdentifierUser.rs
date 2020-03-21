// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Typically used to create a PID file in `/var/run` before switching user and group.
pub trait OriginalRealUserAndGroupIdentifierUser
{
	/// Do something with the effective user and effective group identifiers before switching user and group.
	fn create_pid_file_before_switching_user_and_group(&self, effective_user_identifier: uid_t, effective_group_identifier: gid_t);
}
