// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive Descriptor Control Queue.
pub const ALLRXDCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0c028, 0x40, 0, 7);

/// Receive Address Low.
pub const RAL64: RegisterDefinition<ReadWriteRegister<u64>, u64> = RegisterDefinition::array(0x05400, 0x08, 0, 15);

/// Receive Address Low.
pub const RAL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x05400, 0x08, 0, 15);

/// Receive Address High.
pub const RAH: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x05404, 0x08, 0, 15);

/// VLAN Filter Table Array.
pub const VFTA: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x05600, 0x04, 0, 127);

/// Receive Descriptor Base low.
pub const RDBAL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0c000, 0x40, 0, 7);

/// Receive Descriptor Base High.
pub const RDBAH: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0c004, 0x40, 0, 7);

/// Receive Descriptor Ring Length.
pub const RDLEN: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0c008, 0x40, 0, 7);

/// Receive Descriptor Head.
pub const RDH: RegisterDefinition<ReadOnlyRegister<u32>, u32> = RegisterDefinition::array(0x0c010, 0x40, 0, 7);

/// Receive Descriptor Tail.
pub const RDT: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0c018, 0x40, 0, 7);

/// Re Descriptor Control Queue.
pub const RXDCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0c028, 0x40, 0, 7);

/// Receive DCA CTRL Register Queue.
pub const RXCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0c014, 0x40, 0, 7);

/// Split and Replication Receive Control.
pub const SRRCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0c00c, 0x40, 0, 7);

/// Alignment Error Count.
pub const ALGNERRC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04004);

/// Receive Error Count.
pub const RXERRC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x0400C);

/// Receive Length Error Count.
pub const RLEC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04040);

/// Cyclic Redundancy Check (CRC) Error Count.
pub const CRCERRS: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04000);

/// Missed Packets Count.
pub const MPC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04010);

/// Multiple Receive Queues Command Register.
pub const MRQC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x05818);

/// Energy Efficient Ethernet (EEE) Register.
pub const EEER: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00E30);

/// Extended Interrupt Mask Clear.
pub const EIMC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x01528);

/// Software Semaphore.
pub const SWSM: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x05b50);

/// Management Control.
pub const MANC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x05820);

/// MDI Control.
pub const MDIC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00020);

/// MDI Configuration.
pub const MDICNFG: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00E04);

/// Receive Long packet maximal length.
pub const RLPML: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x05004);

/// Receive Packets to host count.
pub const RPTHC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04104);

/// Software Firmware Synchronization.
pub const SW_FW_SYNC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x05b5c);

/// Transmit Control.
pub const TCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00400);

/// Extended Transmit Control.
pub const TCTL_EXT: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00400);

/// Excessive Collisions Count.
pub const ECOL: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04018);

/// Late Collisions Count.
pub const LATECOL: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x0401C);

/// Good Octets Received Count.
pub const GORCL: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04088);

/// Good Octets Received Count.
pub const GORCH: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x0408C);

/// Good Octets Transmitted Count.
pub const GOTCL: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04090);

/// Good Octets Transmitted Count.
pub const GOTCH: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04094);

/// Receive No Buffers Count.
pub const RNBC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x040A0);

/// Transmit Descriptor Base Low.
pub const TDBAL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0xe000, 0x40, 0, 7);

/// Transmit Descriptor Base High.
pub const TDBAH: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0xe004, 0x40, 0, 7);

/// Transmit Descriptor Ring Length.
pub const TDLEN: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0xe008, 0x40, 0, 7);

/// Transmit Descriptor Head.
pub const TDH: RegisterDefinition<ReadOnlyRegister<u32>, u32> = RegisterDefinition::array(0xe010, 0x40, 0, 7);

/// Transmit Descriptor Tail.
pub const TDT: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0xe018, 0x40, 0, 7);

/// Transmit Descriptor Control Queue.
pub const TXDCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0xe028, 0x40, 0, 7);

/// Transmit DCA CTRL Register Queue.
pub const TXCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0xe014, 0x40, 0, 7);
