// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A Linux object name, used for Commands, Processes, Threads, NetworkInterfaces, BPF maps and BPF programs.
///
/// Relies on the fact that the following are all the same length:-
///
/// * `TASK_COMM_LEN`.
/// * `BPF_OBJ_NAME_LEN`.
/// * `IFNAMSIZ`.
/// * `IF_NAMESIZE`.
pub type ObjectName16 = ObjectName::<16>;
