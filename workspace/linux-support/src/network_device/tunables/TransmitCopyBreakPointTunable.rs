// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Transmit copy break point: frames greater than this are copied.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct TransmitCopyBreakPointTunable(pub CopyBreakPointTunable);

impl Tunable for TransmitCopyBreakPointTunable
{
	const Identifier: TunableIdentifier = TunableIdentifier::normal(tunable_id::ETHTOOL_TX_COPYBREAK);
	
	const TypeIdentifier: tunable_type_id = CopyBreakPointTunable::TypeIdentifier;
	
	const Commands: Commands = CopyBreakPointTunable::Commands;
}
