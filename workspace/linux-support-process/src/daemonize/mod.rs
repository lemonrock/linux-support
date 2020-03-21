use errno::errno;
use libc::*;
use likely::likely;
use likely::unlikely;
use linux_support_common::get_program_name;
use linux_support_common::logging::redirect_standard_out_and_standard_error_to_syslog;
use linux_support_common::paths::DevPath;
use linux_support_common::paths::PathExt;
use linux_support_common::strings::OsStrExtMore;
use linux_support_common::user_and_groups::OriginalRealUserAndGroupIdentifierUser;
use serde::Deserialize;
use serde::Serialize;
use std::cell::UnsafeCell;
use std::ffi::CStr;
use std::fs::remove_file;
use std::io;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::path::PathBuf;
use std::process;


include!("Daemonize.rs");
include!("DaemonizeCleanUpOnExit.rs");
