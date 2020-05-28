// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) const IFA_FLAGS: c_ushort = IFA_MULTICAST + 1;

pub(super) const IFA_RT_PRIORITY: c_ushort = IFA_FLAGS + 1;

pub(super) const IFA_TARGET_NETNSID: c_ushort = IFA_RT_PRIORITY + 1;
