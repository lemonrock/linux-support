// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use super::NumberOfPages;
use super::NonZeroNumberOfPages;
use super::NonZeroKilobyte;
use super::PageSize;
use crate::memory::information::MemoryInformation;
use crate::memory::information::MemoryInformationName;
use crate::paths::*;
use crate::user_and_groups::assert_effective_user_id_is_root;
use crate::process_control::process_control_wrapper2;


include!("change_transparent_huge_pages.rs");
include!("DefaultPageSizeAndHugePageSizes.rs");
include!("GlobalHugePagePoolSize.rs");
include!("HugePageSize.rs");
include!("HugePagePoolStatistics.rs");
include!("PageSizeOrHugePageSize.rs");
include!("TransparentHugePageDefragmentationChoice.rs");
include!("TransparentHugePageRegularMemoryChoice.rs");
include!("TransparentHugePageSharedMemoryChoice.rs");
