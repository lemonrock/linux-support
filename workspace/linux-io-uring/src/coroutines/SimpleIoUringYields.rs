// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Assumes that only the 'zero' user bits are in use.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum SimpleIoUringYields
{
	SubmissionQueueFull,
	
	AwaitingIoUring,
}

impl SimpleIoUringYields
{
	/// Assumes that only the 'zero' user bits are in use.
	///
	/// Suitable for a server loop.
	///
	/// Called from coroutine management thread.
	#[inline(always)]
	fn yielded_start_for_server_loop<Complete: Sized>(start_outcome: StartOutcome<Self, Complete>) -> ()
	{
		use self::StartOutcome::*;
		use self::SimpleIoUringYields::*;
		
		match start_outcome
		{
			WouldLikeToResume(SubmissionQueueFull) => panic!("The submission queue should not be full when starting a server loop coroutine (unless only a tiny queue depth has been specified or a server loop is being started a long time after application start)"),
			
			WouldLikeToResume(AwaitingIoUring) => (),
			
			Complete(_) => panic!("A server loop should never complete"),
		}
	}
	
	/// Called from coroutine.
	///
	/// Yields to coroutine manager thread.
	///
	/// Returns to coroutine.
	#[inline(always)]
	fn yield_submit_io_uring<Complete: Sized>(yielder: &mut Yielder<SimpleIoUringResumeArguments, Self, Complete>, io_uring: &Rc<IoUring<'static>>, add_entry: &mut impl FnMut(SubmissionQueueEntry)) -> bool
	{
		const SubmissionSucceeded: Result<(), ()> = Ok(());
		const SubmissionQueueIsFull: Result<(), ()> = Err(());
		
		loop
		{
			use self::SimpleIoUringResumeArguments::*;
			
			match io_uring.push_submission_queue_entry(|submission_queue_entry| add_entry(submission_queue_entry))
			{
				SubmissionSucceeded => return false,
				
				SubmissionQueueIsFull => match yielder.yields(SimpleIoUringYields::SubmissionQueueFull, Self::KillError)
				{
					Ok(TrySubmissionQueueAgain) => continue,
					
					Ok(Submitted(_, _)) => unreachable!("Logic design flaw"),
					
					Err(_kill_error) => return true
				}
			}
		}
	}
	
	/// Called from coroutine.
	///
	/// Yields to coroutine manager thread.
	///
	/// Returns to coroutine.
	///
	/// Assumes that only the 'zero' user bits are in use.
	#[inline(always)]
	fn yield_awaiting_io_uring<Complete: Sized>(yielder: &mut Yielder<SimpleIoUringResumeArguments, Self, Complete>) -> Result<CompletionResponse, ()>
	{
		use self::SimpleIoUringResumeArguments::*;
		
		match yielder.yields(SimpleIoUringYields::AwaitingIoUring, Self::KillError)
		{
			Ok(Submitted(UserBits::Zero, completion_response)) => Ok(completion_response),
			
			Ok(Submitted(_, _)) => panic!("Only `UserBits::Zero` is supported"),
			
			Ok(TrySubmissionQueueAgain) => unreachable!("Logic design flaw"),
			
			Err(_kill_error) => Err(())
		}
	}
	
	const KillError: () = ();
}
