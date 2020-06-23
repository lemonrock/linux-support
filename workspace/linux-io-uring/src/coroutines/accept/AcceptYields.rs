// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum AcceptYields
{
	SubmissionQueueFull,

	AwaitingIoUring,
}

impl AcceptYields
{
	#[inline(always)]
	fn yield_start(result: Result<StartOutcome<Self, AcceptComplete>, AllocErr>) -> Result<(), ThreadLoopInitializationError>
	{
		use self::StartOutcome::*;
		use self::AcceptYields::*;
		
		match result
		{
			Err(error) => return Err(ThreadLoopInitializationError::CouldNotAllocateAcceptCoroutine(error)),
			
			Ok(WouldLikeToResume(AwaitingIoUring)) => Ok(()),
			
			Ok(WouldLikeToResume(SubmissionQueueFull)) => panic!("The submission queue should not be full when starting an accept coroutine"),
			
			Ok(Complete(())) => panic!("An accept loop should never complete"),
		}
	}
	
	#[inline(always)]
	fn yield_submit_io_uring(yielder: &mut Yielder<AcceptResumeArguments, AcceptYields, AcceptComplete>, io_uring: &Rc<IoUring<'static>>, add_entry: &mut impl FnMut(SubmissionQueueEntry)) -> bool
	{
		const SubmissionSucceeded: Result<(), ()> = Ok(());
		const SubmissionQueueIsFull: Result<(), ()> = Err(());
		
		loop
		{
			use self::AcceptResumeArguments::*;
			
			match io_uring.push_submission_queue_entry(|submission_queue_entry| add_entry(submission_queue_entry))
			{
				SubmissionSucceeded => return false,
				
				SubmissionQueueIsFull => match yielder.yields(AcceptYields::SubmissionQueueFull, Self::KillError)
				{
					Ok(TrySubmissionQueueAgain) => continue,
					
					Ok(Submitted(_, _)) => unreachable!("Logic design flaw"),
					
					Err(_kill_error) => return true
				}
			}
		}
	}
	
	#[inline(always)]
	fn yield_awaiting_io_uring(yielder: &mut Yielder<AcceptResumeArguments, AcceptYields, AcceptComplete>) -> Result<CompletionResponse, ()>
	{
		use self::AcceptResumeArguments::*;
		
		match yielder.yields(AcceptYields::AwaitingIoUring, Self::KillError)
		{
			Ok(Submitted(UserBits::Zero, completion_response)) => Ok(completion_response),
			
			Ok(Submitted(_, _)) => unreachable!("Logic design flaw"),
			
			Ok(TrySubmissionQueueAgain) => unreachable!("Logic design flaw"),
			
			Err(_kill_error) => Err(())
		}
	}
	
	const KillError: () = ();
}
