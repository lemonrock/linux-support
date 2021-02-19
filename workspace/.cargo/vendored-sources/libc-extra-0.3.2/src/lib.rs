// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)] 



#[cfg(any(target_os = "android", target_os = "linux"))] #[macro_use] extern crate cfg_if;
#[cfg(unix)] extern crate const_cstr_fork;
#[cfg(unix)] extern crate libc;


/// Functionality for `#[cfg(any(target_os = "android", target_os = "linux"))]`.
#[cfg(any(target_os = "android", target_os = "linux"))] pub mod android_linux;


/// Functionality for `#[cfg(target_os = "linux")]`.
#[cfg(target_os = "linux")] pub mod linux;


/// Functionality for `#[cfg(unix)]`.
#[cfg(unix)] pub mod unix;
