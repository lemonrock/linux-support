// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Start arguments.
pub(crate) type AcceptStartArguments<SA: SocketAddress, CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>, AC: AccessControl<SA::SD, AccessControlValue>> =
(
	Rc<IoUring<'static>>,
	AcceptPublisher<SA>,
	StreamingServerListenerSocketFileDescriptor<SA::SD>,
	AC,
	ServiceProtocolIdentifier,
	DogStatsDPublisher<CoroutineHeapSize, GTACSA>,
	&'static GTACSA,
	Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
	Rc<ThreadLocalNumericAdditionalDogStatsDTagsCache<HyperThread, CoroutineHeapSize, GTACSA>>,
	HyperThread,
);
