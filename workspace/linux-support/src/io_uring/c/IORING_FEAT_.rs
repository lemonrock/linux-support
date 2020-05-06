// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// `io_uring_params->features` flags.
const IORING_FEAT_SINGLE_MMAP: u32 = 1 << 0;
const IORING_FEAT_NODROP: u32 = 1 << 1;
const IORING_FEAT_SUBMIT_STABLE: u32 = 1 << 2;
const IORING_FEAT_RW_CUR_POS: u32 = 1 << 3;
const IORING_FEAT_CUR_PERSONALITY: u32 = 1 << 4;
const IORING_FEAT_FAST_POLL: u32 = 1 << 5;
