// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;


include!("__NR_SCMP_ERROR.rs");
include!("SCMP_ACT.rs");
include!("SCMP_ACT_ERRNO.rs");
include!("SCMP_ACT_TRACE.rs");
include!("scmp_arch.rs");
include!("scmp_arg_cmp.rs");
include!("scmp_compare.rs");
include!("scmp_datum_t.rs");
include!("scmp_filter_attr.rs");
include!("scmp_filter_ctx.rs");
include!("seccomp.functions.rs");
