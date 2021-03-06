// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use super::*;
use self::c::*;
use crate::user_and_groups::assert_effective_user_id_is_root;


mod c;


include!("InotifyAddError.rs");
include!("InotifyAddWatchFlags.rs");
include!("InotifyEventFlags.rs");
include!("InotifyFileDescriptor.rs");
include!("InotifyWatchDescriptor.rs");
include!("maximum_number_of_events_that_can_be_queued.rs");
include!("maximum_number_of_inotify_instances_per_user.rs");
include!("maximum_number_of_inotify_instances_per_user_namespaced.rs");
include!("maximum_number_of_watches_per_user.rs");
include!("maximum_number_of_watches_per_user_namespaced.rs");
include!("set_maximum_number_of_events_that_can_be_queued.rs");
include!("set_maximum_number_of_inotify_instances_per_user.rs");
include!("set_maximum_number_of_inotify_instances_per_user_namespaced.rs");
include!("set_maximum_number_of_watches_per_user.rs");
include!("set_maximum_number_of_watches_per_user_namespaced.rs");
