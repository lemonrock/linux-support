// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct ParsedPanicErrorLoggerProcessLoggingConfiguration;

impl ParsedPanicErrorLogger for ParsedPanicErrorLoggerProcessLoggingConfiguration
{
	#[inline(always)]
	fn log(&self, parsed_panic: ParsedPanic)
	{
		let current_hyper_thread: u16 = HyperThread::current().1.into();

		let message = format!("ThreadName:{}:ThreadId:{:?}:CurrentHyperThread:{}:File:{}:Line:{}:Column:{}:Cause:{}:Backtrace:{}", parsed_panic.thread_name(), parsed_panic.thread_id(), current_hyper_thread, parsed_panic.source_file, parsed_panic.line_number, parsed_panic.column_number, parsed_panic.cause, parsed_panic.backtrace);
		
		lazy_static!
		{
			static ref Template: Rfc3164MessageTemplate = StaticLoggingConfiguration::rfc_3164_message_template(KnownFacility::security_or_authorization_messages_0, Severity::Critical);
		}
		LocalSyslogSocket::syslog_falling_back_to_standard_error(Template.deref(), &message)
	}
}
