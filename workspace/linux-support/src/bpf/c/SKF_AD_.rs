// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// Sourced from `linux/filter.h`.
// `SKF` seems to stand for `socket filter`.

pub(crate) const SKF_AD_OFF: i32 = -0x1000;
pub(crate) const SKF_AD_PROTOCOL: i32 = 0;
pub(crate) const SKF_AD_PKTTYPE: i32 = 4;
pub(crate) const SKF_AD_IFINDEX: i32 = 8;
pub(crate) const SKF_AD_NLATTR: i32 = 12;
pub(crate) const SKF_AD_NLATTR_NEST: i32 = 16;
pub(crate) const SKF_AD_MARK: i32 = 20;
pub(crate) const SKF_AD_QUEUE: i32 = 24;
pub(crate) const SKF_AD_HATYPE: i32 = 28;
pub(crate) const SKF_AD_RXHASH: i32 = 32;
pub(crate) const SKF_AD_CPU: i32 = 36;
pub(crate) const SKF_AD_ALU_XOR_X: i32 = 40;
pub(crate) const SKF_AD_VLAN_TAG: i32 = 44;
pub(crate) const SKF_AD_VLAN_TAG_PRESENT: i32 = 48;
pub(crate) const SKF_AD_PAY_OFFSET: i32 = 52;
pub(crate) const SKF_AD_RANDOM: i32 = 56;
pub(crate) const SKF_AD_VLAN_TPID: i32 = 60;
pub(crate) const SKF_AD_MAX: i32 = 64;
