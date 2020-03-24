// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// VM VLAN insert.
pub const VMVIR: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x03700, 0x04, 0, 7);

/// Packet Split Receive Type.
pub const PSRTYPE: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x05480, 0x04, 0, 7);

/// VM Offload.
pub const VMOLR: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x05AD0, 0x04, 0, 7);

/// VLAN VM Filter.
pub const VLVF: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x05d00, 0x04, 0, 31);

/// DMA VM Offload.
pub const DVMOLR: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x0C038, 0x04, 0, 7);

/// Virtual Mirror rule control.
pub const VMRCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x05D80, 0x04, 0, 7);

/// Virtual Mirror rule VLAN.
pub const VMRVLAN: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x05D90, 0x04, 0, 7);

/// Virtual Mirror rule VM.
pub const VMRVM: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::array(0x05DA0, 0x04, 0, 7);

/// Receive Queue Drop Packet Count.
pub const RQDPC: RegisterDefinition<ReadOnlyCounterResetOnReadRegister<u32>, u32> = RegisterDefinition::array(0x0C030, 0x40, 0, 7);

/// Transmit Queue Drop Packet Count.
pub const TQDPC: RegisterDefinition<ReadOnlyCounterNotResetOnReadRegister<u32>, u32> = RegisterDefinition::array(0x0E030, 0x40, 0, 7);

/// Per Queue Good Packets Received Count.
pub const PQGPRC: RegisterDefinition<ReadOnlyCounterNotResetOnReadRegister<u32>, u32> = RegisterDefinition::array(0x10010, 0x100, 0, 7);

/// Per Queue Good Packets Transmitted Count.
pub const PQGPTC: RegisterDefinition<ReadOnlyCounterNotResetOnReadRegister<u32>, u32> = RegisterDefinition::array(0x10014, 0x100, 0, 7);

/// Per Queue Good Octets Received Count.
pub const PQGORC: RegisterDefinition<ReadOnlyCounterNotResetOnReadRegister<u32>, u32> = RegisterDefinition::array(0x10018, 0x100, 0, 7);

/// Per Queue Octets Transmitted Count.
pub const PQGOTC: RegisterDefinition<ReadOnlyCounterNotResetOnReadRegister<u32>, u32> = RegisterDefinition::array(0x10034, 0x100, 0, 7);

/// Per Queue Multicast Packets Received.
pub const PQMPRC: RegisterDefinition<ReadOnlyCounterNotResetOnReadRegister<u32>, u32> = RegisterDefinition::array(0x10038, 0x100, 0, 7);

/// VF Receive Enable.
pub const VFRE: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00C8C);

/// VF Transmit Enable.
pub const VFTE: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00C90);

/// Manageability EEPROM-Mode Control.
pub const EEMNGCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x01010);

/// EEPROM-Mode Control.
pub const EEC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x00010);

/// Queue Drop Enable.
pub const QDE: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x02408);

/// DMA Transmit Control.
pub const DTXCTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x03590);

/// Replicated Packet Split Receive Type.
pub const RPLPSRTYPE: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x054C0);

/// VMDq Control.
pub const VT_CTL: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x0581C);

/// Transmit Switch Control.
pub const TXSWC: RegisterDefinition<ReadWriteRegister<u32>, u32> = RegisterDefinition::singleton(0x05ACC);
