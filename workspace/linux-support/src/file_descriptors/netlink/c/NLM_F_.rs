// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Modifiers to DELETE request: do not delete recursively.
pub(crate) const NLM_F_NONREC: c_int = 0x100;

/// Flags for ACK message: request was capped.
pub(crate) const NLM_F_CAPPED: c_int = 0x100;

/// Flags for ACK message: extended ACK TVLs were included.
pub(crate) const NLM_F_ACK_TLVS: c_int = 0x200;
