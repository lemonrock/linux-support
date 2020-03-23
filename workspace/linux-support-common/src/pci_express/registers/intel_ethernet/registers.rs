// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Redirection Table.
pub const RETA: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x5c00, 0x04, 0, 31);

/// RSS Random Key.
pub const RSSRK: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x5C80, 0x04, 0, 9);

/// Broadcast Packets Received Count.
pub const BPRC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04078);

/// Broadcast Packets Transmitted Count.
pub const BPTC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x040F4);

/// Device Control.
pub const CTRL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00000);

/// Extended Device Control.
pub const CTRL_EXT: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00018);

/// Device Status.
pub const STATUS: RegisterDefinition<ReadOnlyRegister<u32>, u32> = RegisterDefinition::singleton(0x00008);

/// RX Control.
pub const RCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00100);

/// CRC Error Count.
pub const CRCERRS: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04000);

/// Good Packets Received Count.
pub const GPRC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04074);

/// Good Packets Transmitted Count.
pub const GPTC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04080);

/// Good Octets Received Count 64-bit.
pub const GORC64: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u64>, u64> = RegisterDefinition::singleton(0x04088);

/// Good Octets Transmitted Count 64-bit.
pub const GOTC64: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u64>, u64> = RegisterDefinition::singleton(0x04090);

/// Multicast Packets Received Count.
pub const MPRC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x0407C);

/// Multicast Packets Transmitted Count.
pub const MPTC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x040F0);
