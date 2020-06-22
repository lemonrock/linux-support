// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An alert event suitable as a log message template.
#[macro_export]
macro_rules! alert
{
	($title: literal, $aggregation_key: literal, $priority: ident, $alert_type: ident) =>
	{
		{
			use $crate::dogstatsd::event::EventTemplate;
			use $crate::dogstatsd::event::EventPriority::*;
			use $crate::dogstatsd::event::EventAlertType::*;
			
			lazy_static!
			{
				static ref AggregationKey: ArrayString<[u8; 100]> = ArrayString::from($aggregation_key).unwrap();
			}
			#[thread_local] static Template: EventTemplate = EventTemplate::new_alert_with_common_tags($title, $priority, $alert_type, &AggregationKey);
			(&Template)
		}
	}
}
