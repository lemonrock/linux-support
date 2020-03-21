use errno::errno;
use likely::likely;
use likely::unlikely;
use libc::*;
use serde::Deserialize;
use serde::Serialize;
use std::convert::TryInto;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::NonNull;


include!("assert_effective_user_id_is_root.rs");
include!("initgroups_wrapper.rs");
include!("OriginalRealUserAndGroupIdentifierUser.rs");
include!("setresgid_wrapper.rs");
include!("setresuid_wrapper.rs");
include!("UserAndGroupSettings.rs");
