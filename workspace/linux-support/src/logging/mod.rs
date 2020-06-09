// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use super::*;
use self::rfc5424::*;
use super::cpu::HyperThread;
use crate::file_descriptors::directory::AccessPermissions;
use crate::file_descriptors::socket::*;
use crate::linux_kernel_version::*;
use crate::logging::rfc3164::Rfc3164MessageTemplate;
use crate::paths::DevPath;
use crate::process::ProcessName;
/// RFC 5424 syslog.
pub mod rfc3164;


/// RFC 5424 syslog.
pub mod rfc5424;


include!("Facility.rs");
include!("KnownFacility.rs");
include!("LocalSyslogSocket.rs");
include!("MessageTemplate.rs");
include!("ParsedPanicErrorLoggerProcessLoggingConfiguration.rs");
include!("PriorityValue.rs");
include!("PrivateEnterpriseNumber.rs");
include!("ProcessLoggingConfiguration.rs");
include!("ProcessLoggingConfigurationError.rs");
include!("Severity.rs");
include!("StaticLoggingConfiguration.rs");
include!("system_information.rs");
include!("UnknownFacility.rs");
include!("write_message_with_line_feed_escaped_truncated.rs");
include!("write_slice_truncated.rs");
include!("write_slice_unchecked.rs");
include!("write_to_standard_error_ignoring_failure.rs");
include!("write_to_standard_out_ignoring_failure.rs");
include!("write_to_standard_stream_ignoring_failure.rs");
