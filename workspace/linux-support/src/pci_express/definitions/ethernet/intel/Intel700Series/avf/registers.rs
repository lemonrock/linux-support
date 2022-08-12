// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// VF Reset Status.
pub const VFGEN_RSTAT: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00008800);

/// VF Interrupt Dynamic Control Zero.
pub const VFINT_DYN_CTL0: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00005C00);

/// VF MailBox Transmit Queue e Base Address Low.
pub const VF_ATQBAL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00007C00);

/// VF MailBox Transmit Queue Base Address High.
pub const VF_ATQBAH: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00007800);

/// VF MailBox Transmit Queue Length.
pub const VF_ATQLEN: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00006800);

/// VF MailBox Transmit Head.
pub const VF_ATQH: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00006400);

/// VF MailBox Transmit Tail.
pub const VF_ATQT: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00008400);

/// VF MailBox Receive Queue Base Address Low.
pub const VF_ARQBAL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00006C00);

/// VF MailBox Receive Queue Base Address High.
pub const VF_ARQBAH: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00006000);

/// VF MailBox Receive Queue Length.
pub const VF_ARQLEN: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00008000);

/// VF MailBox Receive Head.
pub const VF_ARQH: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00007400);

/// VF MailBox Receive Tail.
pub const VF_ARQT: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00007000);

/// VF Interrupt Dynamic Control N.
pub const VFINT_DYN_CTLN: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x00003800, 0x4, 0, 63);

/// VF Interrupt Throttling Zero.
pub const VFINT_ITR0: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x00004C00, 0x4, 0, 2);

/// Transmit Queue Tail.
pub const QTX_TAIL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x00000000, 0x4, 0, 255);

/// Receive Queue Tail.
pub const QRX_TAIL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x00002000, 0x4, 0, 255);
