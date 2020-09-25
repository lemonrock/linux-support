// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Hash or specification (`tcp_ip4_spec`).
pub(crate) const TCP_V4_FLOW: u32 = 0x01;

/// Hash or specification (`udp_ip4_spec`).
pub(crate) const UDP_V4_FLOW: u32 = 0x02;

/// Hash or specification (`sctp_ip4_spec`).
pub(crate) const SCTP_V4_FLOW: u32 = 0x03;

/// Hash only.
pub(crate) const AH_ESP_V4_FLOW: u32 = 0x04;

/// Hash or specification (`tcp_ip6_spec`; nfc only).
pub(crate) const TCP_V6_FLOW: u32 = 0x05;

/// Hash or specification (`udp_ip6_spec`; nfc only).
pub(crate) const UDP_V6_FLOW: u32 = 0x06;

/// Hash or specification (`sctp_ip6_spec`; nfc only).
pub(crate) const SCTP_V6_FLOW: u32 = 0x07;

/// Hash only.
pub(crate) const AH_ESP_V6_FLOW: u32 = 0x08;

/// Hash or specification (`ah_ip4_spec`).
pub(crate) const AH_V4_FLOW: u32 = 0x09;

/// Hash or specification (`esp_ip4_spec`).
pub(crate) const ESP_V4_FLOW: u32 = 0x0A;

/// Hash or specification (`ah_ip6_spec`; nfc only).
pub(crate) const AH_V6_FLOW: u32 = 0x0B;

/// Hash or specification (`esp_ip6_spec`; nfc only).
pub(crate) const ESP_V6_FLOW: u32 = 0x0C;

/// Specification only (`usr_ip4_spec`).
pub(crate) const IPV4_USER_FLOW: u32 = 0x0D;

pub(crate) const IP_USER_FLOW: u32 = IPV4_USER_FLOW;

/// Specification only (usr_ip6_spec; nfc only).
pub(crate) const IPV6_USER_FLOW: u32 = 0x0E;

/// Hash only.
pub(crate) const IPV4_FLOW: u32 = 0x10;

/// Hash only.
pub(crate) const IPV6_FLOW: u32 = 0x11;

/// Specification only (`ether_spec`).
pub(crate) const ETHER_FLOW: u32 = 0x12;
