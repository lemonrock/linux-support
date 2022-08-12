// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive Descriptor Control.
pub const ALLRXDCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::split_array(0x01028, 0x40, 0, 63, 0x0D028, 0x40, 64, 127);

/// Destination Address Queue Filter.
pub const DAQF: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0E200, 0x04, 0, 127);

/// Five Tuple Queue Filter.
pub const FTQF: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0E600, 0x04, 0, 127);

/// EtherType Queue Filter.
pub const ETQF: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x05128, 0x04, 0, 7);

/// EtherType Queue Select.
pub const ETQS: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0EC00, 0x04, 0, 7);

/// MAC Pool Select Array.
pub const MPSAR: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0A600, 0x04, 0, 255);

/// PF Unicast Table Array.
pub const PFUTA: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0F400, 0x04, 0, 127);

/// PF VM VLAN Pool Filter.
pub const PFVLVF: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0F100, 0x04, 0, 63);

/// PF VM VLAN Pool Filter Bitmap.
pub const PFVLVFB: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0F200, 0x04, 0, 127);

/// PF Mirror Rule Control.
pub const PFMRCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0F600, 0x04, 0, 3);

/// PF Mirror Rule VLAN.
pub const PFMRVLAN: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0F610, 0x04, 0, 7);

/// PF Mirror Rule Pool.
pub const PFMRVM: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0F630, 0x04, 0, 7);

/// PF VF Receive Enable.
pub const PFVFRE: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x051E0, 0x04, 0, 1);

/// PF VF Transmit Enable.
pub const PFVFTE: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x08110, 0x04, 0, 1);

/// PF VM Transmit Switch Loopback Enable.
pub const PFVMTXSW: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x05180, 0x04, 0, 1);

/// PF VF Anti Spoof Control.
pub const PFVFSPOOF: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x08200, 0x04, 0, 7);

/// PF VM VLAN Insert Register.
pub const PFVMVIR: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x08000, 0x04, 0, 63);

/// PF VM L2 Control Register.
pub const PFVML2FLT: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0F000, 0x04, 0, 63);

/// Queue Packets Received Count.
pub const QPRC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::array(0x01030, 0x40, 0, 15);

/// Queue Packets Received Drop Count.
pub const QPRDC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::array(0x01430, 0x40, 0, 15);

/// Queue Bytes Received Count.
pub const QBRC64: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u64>, u64> = RegisterDefinition::array(0x01034, 0x40, 0, 15);

/// Queue Packets Transmitted Count.
pub const QPTC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::array(0x08680, 0x04, 0, 15);

/// Queue Bytes Transmitted Count Low.
pub const QBTC64: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u64>, u64> = RegisterDefinition::array(0x08700, 0x08, 0, 15);

/// Source Address Queue Filter.
pub const SAQF: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0E000, 0x04, 0, 127);

/// Source Destination Port Queue Filter.
pub const SDPQF: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0E400, 0x04, 0, 127);

/// Packet Split Receive Type Register.
pub const PSRTYPE: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0EA00, 0x04, 0, 63);

/// Receive Address High.
pub const RAH: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0A204, 0x08, 0, 127);

/// Receive Address Low.
pub const RAL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0A200, 0x08, 0, 127);

/// Receive Address Low and High.
pub const RAL64: RegisterDefinition<ReadWriteRegister<u64>, u64> = RegisterDefinition::array(0x0A200, 0x08, 0, 127);

/// Receive Queue Statistic Mapping Registers.
pub const RQSM: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x02300, 0x04, 0, 31);

/// DCB Transmit Descriptor Plane T2 Config.
pub const RTTDT2C: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x04910, 0x04, 0, 7);

/// DCB Transmit Packet Plane T2 Config.
pub const RTTPT2C: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0CD20, 0x04, 0, 7);

/// DCB Receive Packet Plane T4 Config.
pub const RTRPT4C: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x02140, 0x04, 0, 7);

/// Receive Packet Buffer Size.
pub const RXPBSIZE: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x03C00, 0x04, 0, 7);

/// Receive Queue Statistic Mapping Registers.
pub const RQSMR: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x02300, 0x04, 0, 31);

/// Transmit Queue Statistic Mapping Registers.
pub const TQSM: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x08600, 0x04, 0, 31);

/// Transmit Packet Buffer Size.
pub const TXPBSIZE: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0CC00, 0x04, 0, 7);

/// Transmit Packet Buffer Threshold.
pub const TXPBTHRESH: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x04950, 0x04, 0, 7);

/// VLAN Filter Table Array.
pub const VFTA: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0A000, 0x04, 0, 127);

/// Flow Control Receive Threshold High.
pub const FCRTH: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x03260, 0x40, 0, 7);

/// Receive DCA Control Register.
pub const DCA_RXCTRL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::split_array(0x0100C, 0x40, 0, 63, 0x0D00C, 0x40, 64, 127);

/// Split Receive Control Registers.
pub const SRRCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::split_array(0x01014, 0x40, 0, 63, 0x0D014, 0x40, 64, 127);

/// Receive Descriptor Base Address Low.
pub const RDBAL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::split_array(0x01000, 0x40, 0, 63, 0x0D000, 0x40, 64, 127);

/// Receive Descriptor Base Address High.
pub const RDBAH: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::split_array(0x01004, 0x40, 0, 63, 0x0D004, 0x40, 64, 127);

/// Receive Descriptor Length.
pub const RDLEN: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::split_array(0x01008, 0x40, 0, 63, 0x0D008, 0x40, 64, 127);

/// Receive Descriptor Head.
pub const RDH: RegisterDefinition<ReadOnlyRegister<u32>, u32> = RegisterDefinition::split_array(0x01010, 0x40, 0, 63, 0x0D010, 0x40, 64, 127);

/// Receive Descriptor Tail.
pub const RDT: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::split_array(0x01018, 0x40, 0, 63, 0x0D018, 0x40, 64, 127);

/// Receive Descriptor Control.
pub const RXDCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::split_array(0x01028, 0x40, 0, 63, 0x0D028, 0x40, 64, 127);

/// Auto Negotiation Control.
pub const AUTOC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x042A0);

/// Auto Negotiation Control 2.
pub const AUTOC2: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x042A8);

/// DMA Transmit Control.
pub const DMATXCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x04A80);

/// DMA Transmit Map Allow Size Requests.
pub const DTXMXSZRQ: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x08100);

/// EEPROM/Flash Control Register.
pub const EEC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x10010);

/// Extended Interrupt Mask Clear.
pub const EIMC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00888);

/// Flow Control Configuration.
pub const FCCFG: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x03D00);

/// Receive Packets Dropped Count.
pub const FCOERPDC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x0241C);

/// Filter Control.
pub const FCTRL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x05080);

/// MAC Core Control 0.
pub const HLREG0: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x04240);

/// Illegal Byte Error Count.
pub const ILLERRC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04004);

/// Link Status Register.
pub const LINKS: RegisterDefinition<ReadOnlyRegister<u32>, u32> = RegisterDefinition::singleton(0x042A4);

/// Max Frame Size.
pub const MAXFRS: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x04268);

/// MAC Flow Control Register.
pub const MFLCN: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x04294);

/// Management Packets Dropped Count.
pub const MNGPDC: RegisterDefinition<ReadOnlyRegister<u32>, u32> = RegisterDefinition::singleton(0x040B8);

/// Multiple Receive Queues Command Register.
pub const MRQC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x0EC80);

/// Multiple Transmit Queues Command Register.
pub const MTQC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x08120);

/// PF Virtual Control Register.
pub const PFVTCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x051B0);

/// PF Queue Drop Enable Register.
pub const PFQDE: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x02F04);

/// PF DMA Transmit General Switch Control.
pub const PFDTXGSWC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x08220);

/// Receive DMA Control Register.
pub const RDRXCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x02F00);

/// DCB Receive Packet plane Control and Status.
pub const RTRPCS: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x02430);

/// DCB Transmit Descriptor Plane Control and Status.
pub const RTTDCS: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x04900);

/// DCB Transmit Packet Plane Control and Status.
pub const RTTPCS: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x0CD00);

/// DCB Receive User Priority to Traffic Class.
pub const RTRUP2TC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x03020);

/// DCB Transmit User Priority to Traffic Class.
pub const RTTUP2TC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x0C800);

/// DCB Transmit Descriptor Plane Queue Select.
pub const RTTDQSEL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x04904);

/// DCB Transmit Descriptor Plane T1 Config.
pub const RTTDT1C: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x04908);

/// DCB Transmit Rate-Scheduler Config.
pub const RTTBCNRC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x04984);

/// Receive Filter Control Register.
pub const RFCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x05008);

/// Receive Control.
pub const RXCTRL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x03000);

/// DMA Good Receive Packet Counter.
pub const RXDGPC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x02F50);

/// DMA Good Transmit Packet Counter.
pub const TXDGPC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x087A0);

/// Receive DMA Statistic Counter Control.
pub const RXDSTATCTRL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x02F40);

/// Receive Under size Count.
pub const RUC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x040A4);

/// Receive Fragment Count.
pub const RFC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x040A8);

/// Receive Oversize Count.
pub const ROC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x040AC);

/// Receive Jabber Count.
pub const RJC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x040B0);

/// Software Semaphore.
pub const SWSM: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x10140);

/// VLAN Control Register.
pub const VLNCTRL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x05088);

/// Error Byte Count.
pub const ERRBC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04008);

/// Good Octets Received Count 64-bit.
pub const GORC64: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u64>, u64> = RegisterDefinition::singleton(0x04088);

/// Good Octets Transmitted Count 64-bit.
pub const GOTC64: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u64>, u64> = RegisterDefinition::singleton(0x04090);

/// Good Octets Received Count Low.
pub const GORCL: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04088);

/// Good Octets Transmitted Count Low.
pub const GOTCL: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::singleton(0x04090);

/// Transmit DCA Control Register.
pub const DCA_TXCTRL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0600C, 0x40, 0, 127);

/// Transmit Descriptor Base Address Low.
pub const TDBAL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x06000, 0x40, 0, 127);

/// Transmit Descriptor Base Address High.
pub const TDBAH: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x06004, 0x40, 0, 127);

/// Transmit Descriptor Length.
pub const TDLEN: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x06008, 0x40, 0, 127);

/// Transmit Descriptor Head.
pub const TDH: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x06010, 0x40, 0, 127);

/// Transmit Descriptor Tail.
pub const TDT: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x06018, 0x40, 0, 127);

/// Transmit Descriptor Control.
pub const TXDCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x06028, 0x40, 0, 127);

/// Transmit Descriptor Completion Write Back Address Low.
pub const TDWBAL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x06038, 0x40, 0, 127);

/// Transmit Descriptor Completion Write Back Address High.
pub const TDWBAH: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0603C, 0x40, 0, 127);
