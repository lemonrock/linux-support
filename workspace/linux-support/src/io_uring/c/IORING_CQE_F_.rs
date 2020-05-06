// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// `cqe->flags`

/// If set, the upper 16 bits are the buffer ID.
///
/// See `IORING_CQE::IORING_CQE_BUFFER_SHIFT`.
pub(super) const IORING_CQE_F_BUFFER: u32 = 1 << 0;
