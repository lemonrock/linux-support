// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))] use c2rust_asm_casts::AsmCast;
#[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))] use c2rust_asm_casts::AsmCastTrait;
use errno::Errno;
use errno::set_errno;
use likely::unlikely;


include!("SYS.rs");
