// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::c::*;


pub(crate) mod c;


include!("change_io_flusher.rs");
include!("change_dumpable.rs");
include!("change_no_new_privileges.rs");
include!("error_number_to_io_error.rs");
include!("MachineCheckExceptionKillPolicy.rs");
include!("process_control_get_boolean.rs");
include!("process_control_wrapper.rs");
include!("process_control_wrapper1.rs");
include!("process_control_wrapper2.rs");
include!("process_control_wrapper3.rs");
include!("result_must_be_zero.rs");
include!("SecureBits.rs");
