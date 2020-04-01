// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright ©2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// 
pub(crate) const TCP_NODELAY: c_int = 1;

///
#[allow(dead_code)]
pub(crate) const TCP_MAXSEG: c_int = 2;

///
#[allow(dead_code)]
pub(crate) const TCP_CORK: c_int = 3;

/// 
pub(crate) const TCP_KEEPIDLE: c_int = 4;

/// 
pub(crate) const TCP_KEEPINTVL: c_int = 5;

/// 
pub(crate) const TCP_KEEPCNT: c_int = 6;

/// 
pub(crate) const TCP_SYNCNT: c_int = 7;

/// 
pub(crate) const TCP_LINGER2: c_int = 8;

/// 
pub(crate) const TCP_DEFER_ACCEPT: c_int = 9;

///
#[allow(dead_code)]
pub(crate) const TCP_WINDOW_CLAMP: c_int = 10;

///
#[allow(dead_code)]
pub(crate) const TCP_INFO: c_int = 11;

///
#[allow(dead_code)]
pub(crate) const TCP_QUICKACK: c_int = 12;

///
#[allow(dead_code)]
pub(crate) const TCP_CONGESTION: c_int = 13;

///
#[allow(dead_code)]
pub(crate) const TCP_MD5SIG: c_int = 14;

///
#[allow(dead_code)]
pub(crate) const TCP_THIN_LINEAR_TIMEOUTS: c_int = 16;

///
#[allow(dead_code)]
pub(crate) const TCP_THIN_DUPACK: c_int = 17;

///
#[allow(dead_code)]
pub(crate) const TCP_USER_TIMEOUT: c_int = 18;

///
#[allow(dead_code)]
pub(crate) const TCP_REPAIR: c_int = 19;

///
#[allow(dead_code)]
pub(crate) const TCP_REPAIR_QUEUE: c_int = 20;

///
#[allow(dead_code)]
pub(crate) const TCP_QUEUE_SEQ: c_int = 21;

///
#[allow(dead_code)]
pub(crate) const TCP_REPAIR_OPTIONS: c_int = 22;

/// 
pub(crate) const TCP_FASTOPEN: c_int = 23;

///
#[allow(dead_code)]
pub(crate) const TCP_TIMESTAMP: c_int = 24;

///
#[allow(dead_code)]
pub(crate) const TCP_NOTSENT_LOWAT: c_int = 25;

///
#[allow(dead_code)]
pub(crate) const TCP_CC_INFO: c_int = 26;

///
#[allow(dead_code)]
pub(crate) const TCP_SAVE_SYN: c_int = 27;

///
#[allow(dead_code)]
pub(crate) const TCP_SAVED_SYN: c_int = 28;

///
#[allow(dead_code)]
pub(crate) const TCP_REPAIR_WINDOW: c_int = 29;

///
#[allow(dead_code)]
pub(crate) const TCP_FASTOPEN_CONNECT: c_int = 30;

///
#[allow(dead_code)]
pub(crate) const TCP_ULP: c_int = 31;

///
#[allow(dead_code)]
pub(crate) const TCP_MD5SIG_EXT: c_int = 32;

///
#[allow(dead_code)]
pub(crate) const TCP_FASTOPEN_KEY: c_int = 33;

///
#[allow(dead_code)]
pub(crate) const TCP_FASTOPEN_NO_COOKIE: c_int = 34;

///
#[allow(dead_code)]
pub(crate) const TCP_ZEROCOPY_RECEIVE: c_int = 35;

/// 
#[allow(dead_code)]
pub(crate) const TCP_INQ: c_int = 36;
