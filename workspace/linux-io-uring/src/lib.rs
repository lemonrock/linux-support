// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]


//! #linux-io-uring
//!
//! This is a mid-level library to provide a coroutine based way to work with io-uring across many cores.
//!
//! See <https://github.com/lemonrock/linux-support> for more details.


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_pointer_width = "64");



use message_dispatch::PerThreadQueueSubscriber;
use std::fmt::Debug;
use terminate::Terminate;
use std::error;
use std::sync::Arc;

// These arg
type MessageHandlerArgumentsWhichMustBeCommonToAllPossibleMessageTypes = ();
type DequeuedMessageProcessingErrorWhichMustBeCommonToAllPossibleMessageTypes = DequeuedMessageProcessingError;

pub struct ThreadLoop<T: Terminate>
{
	terminate: Arc<T>,
	incoming_messages_queue: PerThreadQueueSubscriber<T, MessageHandlerArgumentsWhichMustBeCommonToAllPossibleMessageTypes, DequeuedMessageProcessingErrorWhichMustBeCommonToAllPossibleMessageTypes>,
}

impl<T: Terminate> ThreadLoopBodyFunction for ThreadLoop<T>
{
	fn invoke(&mut self)
	{
		use self::DequeuedMessageProcessingError::*;

		let message_handler_arguments = ();
		let result = self.incoming_messages_queue.receive_and_handle_messages(message_handler_arguments);
		match result
		{
			Err(Fatal(ref cause)) => self.terminate.begin_termination_due_to_irrecoverable_error(cause),

			Err(CarryOn(ref cause)) => ProcessLoggingConfiguration::warn("CarryOn", format!("{}", cause)),

			Ok(()) => (),
		}
	}
}

pub enum DequeuedMessageProcessingError
{
	Fatal(Box<error::Error>),

	CarryOn(Box<error::Error>),
}
