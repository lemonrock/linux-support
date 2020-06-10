// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// TCP connection information.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tcp_info
{
	pub tcpi_state: u8,
	pub tcpi_ca_state: u8,
	pub tcpi_retransmits: u8,
	pub tcpi_probes: u8,
	pub tcpi_backoff: u8,
	pub tcpi_options: u8,
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2], u8>,
	pub tcpi_rto: u32,
	pub tcpi_ato: u32,
	pub tcpi_snd_mss: u32,
	pub tcpi_rcv_mss: u32,
	pub tcpi_unacked: u32,
	pub tcpi_sacked: u32,
	pub tcpi_lost: u32,
	pub tcpi_retrans: u32,
	pub tcpi_fackets: u32,
	pub tcpi_last_data_sent: u32,
	pub tcpi_last_ack_sent: u32,
	pub tcpi_last_data_recv: u32,
	pub tcpi_last_ack_recv: u32,
	pub tcpi_pmtu: u32,
	pub tcpi_rcv_ssthresh: u32,
	pub tcpi_rtt: u32,
	pub tcpi_rttvar: u32,
	pub tcpi_snd_ssthresh: u32,
	pub tcpi_snd_cwnd: u32,
	pub tcpi_advmss: u32,
	pub tcpi_reordering: u32,
	pub tcpi_rcv_rtt: u32,
	pub tcpi_rcv_space: u32,
	pub tcpi_total_retrans: u32,
	pub tcpi_pacing_rate: u64,
	pub tcpi_max_pacing_rate: u64,
	pub tcpi_bytes_acked: u64,
	pub tcpi_bytes_received: u64,
	pub tcpi_segs_out: u32,
	pub tcpi_segs_in: u32,
	pub tcpi_notsent_bytes: u32,
	pub tcpi_min_rtt: u32,
	pub tcpi_data_segs_in: u32,
	pub tcpi_data_segs_out: u32,
	pub tcpi_delivery_rate: u64,
	pub tcpi_busy_time: u64,
	pub tcpi_rwnd_limited: u64,
	pub tcpi_sndbuf_limited: u64,
	pub tcpi_delivered: u32,
	pub tcpi_delivered_ce: u32,
	pub tcpi_bytes_sent: u64,
	pub tcpi_bytes_retrans: u64,
	pub tcpi_dsack_dups: u32,
	pub tcpi_reord_seen: u32,
	pub tcpi_rcv_ooopack: u32,
	pub tcpi_snd_wnd: u32,
}

#[allow(missing_docs)]
impl tcp_info
{
	#[inline(always)]
	pub fn tcpi_snd_wscale(&self) -> u8
	{
		unsafe { transmute(self._bitfield_1.get(0, 4) as u8) }
	}
	
	#[inline(always)]
	pub fn set_tcpi_snd_wscale(&mut self, val: u8)
	{
		unsafe
		{
			let val: u8 = transmute(val);
			self._bitfield_1.set(0, 4, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn tcpi_rcv_wscale(&self) -> u8
	{
		unsafe { transmute(self._bitfield_1.get(4, 4) as u8) }
	}
	
	#[inline(always)]
	pub fn set_tcpi_rcv_wscale(&mut self, val: u8)
	{
		unsafe
		{
			let val: u8 = transmute(val);
			self._bitfield_1.set(4, 4, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn tcpi_delivery_rate_app_limited(&self) -> u8
	{
		unsafe { transmute(self._bitfield_1.get(8, 1) as u8) }
	}
	
	#[inline(always)]
	pub fn set_tcpi_delivery_rate_app_limited(&mut self, val: u8)
	{
		unsafe
		{
			let val: u8 = transmute(val);
			self._bitfield_1.set(8, 1, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn tcpi_fastopen_client_fail(&self) -> u8
	{
		unsafe { transmute(self._bitfield_1.get(9, 2) as u8) }
	}
	
	#[inline(always)]
	pub fn set_tcpi_fastopen_client_fail(&mut self, val: u8)
	{
		unsafe
		{
			let val: u8 = transmute(val);
			self._bitfield_1.set(9, 2, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn new_bitfield_1(
		tcpi_snd_wscale: u8,
		tcpi_rcv_wscale: u8,
		tcpi_delivery_rate_app_limited: u8,
		tcpi_fastopen_client_fail: u8,
	) -> __BindgenBitfieldUnit<[u8; 2usize], u8> {
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize], u8> =
			Default::default();
		__bindgen_bitfield_unit.set(0usize, 4u8, {
			let tcpi_snd_wscale: u8 = unsafe { transmute(tcpi_snd_wscale) };
			tcpi_snd_wscale as u64
		});
		__bindgen_bitfield_unit.set(4usize, 4u8, {
			let tcpi_rcv_wscale: u8 = unsafe { transmute(tcpi_rcv_wscale) };
			tcpi_rcv_wscale as u64
		});
		__bindgen_bitfield_unit.set(8usize, 1u8, {
			let tcpi_delivery_rate_app_limited: u8 =
				unsafe { transmute(tcpi_delivery_rate_app_limited) };
			tcpi_delivery_rate_app_limited as u64
		});
		__bindgen_bitfield_unit.set(9usize, 2u8, {
			let tcpi_fastopen_client_fail: u8 =
				unsafe { transmute(tcpi_fastopen_client_fail) };
			tcpi_fastopen_client_fail as u64
		});
		__bindgen_bitfield_unit
	}
}
