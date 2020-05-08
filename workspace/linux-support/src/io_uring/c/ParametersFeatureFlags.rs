// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	//// `io_uring_params->features` flags.
	pub(super) struct ParametersFeatureFlags: u32
	{
		/// If this flag is set, the two SQ and CQ rings can be mapped with a single mmap(2) call.
		///
		/// The SQEs must still be allocated separately.
		/// This brings the necessary mmap(2) calls down from three to two.
		const SingleMMap = IORING_FEAT_SINGLE_MMAP;

		/// If this flag is set, io_uring supports never dropping completion events.
		///
		/// If a completion event occurs and the CQ ring is full, the kernel stores the event internally until such a time that the CQ ring has room for more entries.
		/// If this overflow condition is entered, attempting to submit more IO with fail with the -EBUSY error value, if it can't flush the overflown events to the CQ ring.
		///
		/// If this happens, the application must reap events from the CQ ring and attempt the submit again.
		const NoDrop = IORING_FEAT_NODROP;

		/// If this flag is set, applications can be certain that any data for async offload has been consumed when the kernel has consumed the SQE.
		const SubmitStable = IORING_FEAT_SUBMIT_STABLE;

		/// If this flag is set, applications can specify offset == -1 with IORING_OP_{READV,WRITEV} , IORING_OP_{READ,WRITE}_FIXED, and IORING_OP_{READ,WRITE} to mean current file position, which behaves like preadv2(2) and pwritev2(2) with offset == -1.
		///
		/// It'll use (and update) the current file position.
		/// This obviously comes with the caveat that if the application has multiple reads or writes in flight, then the end result will not be as expected.
		/// This is similar to threads sharing a file descriptor and doing IO using the current file position.
		const ReadWriteCurrentPosition = IORING_FEAT_RW_CUR_POS;

		/// If this flag is set, then io_uring guarantees that both sync and async execution of a request assumes the credentials of the task that called io_uring_enter(2) to queue the requests.
		///
		/// If this flag isn't set, then requests are issued with the credentials of the task that originally registered the io_uring.
		/// If only one task is using a ring, then this flag doesn't matter as the credentials will always be the same.
		/// Note that this is the default behavior, tasks can still register different personalities through io_uring_register(2) with IORING_REGISTER_PERSONALITY and specify the personality to use in the sqe.
		const Personality = IORING_FEAT_CUR_PERSONALITY;

		#[allow(missing_docs)]
		const FastPoll = IORING_FEAT_FAST_POLL;

		const AllAsOfLinux57 = IORING_FEAT_SINGLE_MMAP | IORING_FEAT_NODROP | IORING_FEAT_SUBMIT_STABLE | IORING_FEAT_RW_CUR_POS | IORING_FEAT_CUR_PERSONALITY | IORING_FEAT_FAST_POLL;
	}
}

impl Default for ParametersFeatureFlags
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::empty()
	}
}
