// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


use ::libc::c_uint;
use ::libc::c_int;


pub const TCGETS: c_int = 0x5401;
pub const TCSETS: c_int = 0x5402;
pub const TCSETSW: c_int = 0x5403;
pub const TCSETSF: c_int = 0x5404;
pub const TCGETA: c_int = 0x5405;
pub const TCSETA: c_int = 0x5406;
pub const TCSETAW: c_int = 0x5407;
pub const TCSETAF: c_int = 0x5408;
pub const TCSBRK: c_int = 0x5409;
pub const TCXONC: c_int = 0x540A;
pub const TCFLSH: c_int = 0x540B;
pub const TIOCEXCL: c_int = 0x540C;
pub const TIOCNXCL: c_int = 0x540D;
pub const TIOCSCTTY: c_int = 0x540E;
pub const TIOCGPGRP: c_int = 0x540F;
pub const TIOCSPGRP: c_int = 0x5410;
pub const TIOCOUTQ: c_int = 0x5411;
pub const TIOCSTI: c_int = 0x5412;
pub const TIOCGWINSZ: c_int = 0x5413;
pub const TIOCSWINSZ: c_int = 0x5414;
pub const TIOCMGET: c_int = 0x5415;
pub const TIOCMBIS: c_int = 0x5416;
pub const TIOCMBIC: c_int = 0x5417;
pub const TIOCMSET: c_int = 0x5418;
pub const TIOCGSOFTCAR: c_int = 0x5419;
pub const TIOCSSOFTCAR: c_int = 0x541A;
pub const FIONREAD: c_int = 0x541B;
pub const TIOCINQ: c_int = FIONREAD;
pub const TIOCLINUX: c_int = 0x541C;
pub const TIOCCONS: c_int = 0x541D;
pub const TIOCGSERIAL: c_int = 0x541E;
pub const TIOCSSERIAL: c_int = 0x541F;
pub const TIOCPKT: c_int = 0x5420;
pub const FIONBIO: c_int = 0x5421;
pub const TIOCNOTTY: c_int = 0x5422;
pub const TIOCSETD: c_int = 0x5423;
pub const TIOCGETD: c_int = 0x5424;
pub const TCSBRKP: c_int = 0x5425;
pub const TIOCSBRK: c_int = 0x5427;
pub const TIOCCBRK: c_int = 0x5428;
pub const TIOCGSID: c_int = 0x5429;

// mips, mips64, powerpc and powerpc64 do not use the '2' versions so has no definition
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64"))] #[allow(overflowing_literals)] pub const TCGETS2: c_int = 0x802c542a;
// #[cfg(target_arch = "sparc64")] pub const TCGETS2: c_int = 0x402c540c;

// mips, mips64, powerpc and powerpc64 do not use the '2' versions so has no definition
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64"))] pub const TCSETS2: c_int = 0x402c542b;
// #[cfg(target_arch = "sparc64")] pub const TCSETS2: c_int = 0x802c540d;

// mips, mips64, powerpc and powerpc64 do not use the '2' versions so has no definition
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64"))] pub const TCSETSW2: c_int = 0x402c542c;
// #[cfg(target_arch = "sparc64")] pub const TCSETSW2: c_int = 0x802c540e;

// mips, mips64, powerpc and powerpc64 do not use the '2' versions so has no definition
#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64"))] pub const TCSETSF2: c_int = 0x402c542d;
// #[cfg(target_arch = "sparc64")] pub const TCSETSF2: c_int = 0x802c540f;

cfg_if!
{
	if #[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch="powerpc", target_arch="powerpc64", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64"))]
	{
		pub const TIOCSRS485: c_int = 0x542F;
	}
	else if #[cfg(target_arch = "mips64")]
	{
	 	pub const TIOCSRS485: c_int = 0xc0205442;
	}
	/*
	else if #[cfg(target_arch = "cris")]
	{
	 	pub const TIOCSRS485: c_int = 0x5463;
	}
	else if #[cfg(target_arch = "sparc64")]
	{
	 	pub const TIOCSRS485: c_int = 0x802c540f;
	}
	*/
	/* TODO: Probable but not certain: 
	else if #[cfg(target_arch = "mips")]
	{
	 	pub const TIOCSRS485: c_int = 0xc0205442;
	}
	*/
	else
	{
	}
}

#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64"))] #[allow(overflowing_literals)] pub const TIOCGPTN: c_int = 0x80045430;
#[cfg(any(target_arch = "mips64", target_arch = "powerpc64"))] pub const TIOCGPTN: c_int = 0x40045430;
// #[cfg(target_arch = "sparc64")] pub const TIOCGPTN: c_int = 0x40047486;
// TODO: Probable but not certain: #[cfg(any(target_arch = "mips", target_arch = "powerpc"))] pub const TIOCGPTN: c_int = 0x40045430;

#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64"))] #[allow(overflowing_literals)] pub const TIOCSPTLCK: c_int = 0x40045431;
#[cfg(any(target_arch = "mips64", target_arch = "powerpc64"))] pub const TIOCSPTLCK: c_int = 0x80045431;
// #[cfg(target_arch = "sparc64")] pub const TIOCSPTLCK: c_int = 0x80045431;
// TODO: Probable but not certain: #[cfg(any(target_arch = "mips", target_arch = "powerpc"))] pub const TIOCSPTLCK: c_int = 0x80045431;

#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64"))] #[allow(overflowing_literals)] pub const TIOCGDEV: c_int = 0x80045432;
#[cfg(any(target_arch = "mips64", target_arch = "powerpc64"))] pub const TIOCGDEV: c_int = 0x40045432;
// #[cfg(target_arch = "sparc64")] pub const TIOCGDEV: c_int = 0x40045432;
// TODO: Probable but not certain: #[cfg(any(target_arch = "mips", target_arch = "powerpc"))] pub const TIOCGDEV: c_int = 0x40045432;

pub const TCGETX: c_int = 0x5432;

pub const TCSETX: c_int = 0x5433;

pub const TCSETXF: c_int = 0x5434;

pub const TCSETXW: c_int = 0x5435;

#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64"))] pub const TIOCSIG: c_int = 0x40045436;
#[cfg(any(target_arch = "mips64", target_arch = "powerpc64"))] pub const TIOCSIG: c_int = 0x80045436;
// TODO: Probable but not certain: #[cfg(any(target_arch = "mips", target_arch = "powerpc"))] pub const TIOCSIG: c_int = 0x80045436;
// #[cfg(target_arch = "sparc64")] pub const TIOCSIG: c_int = 0x80045436;

pub const TIOCVHANGUP: c_int = 0x5437;

#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64"))] #[allow(overflowing_literals)] pub const TIOCGPKT: c_int = 0x80045438;
#[cfg(any(target_arch = "mips64", target_arch = "powerpc64"))] pub const TIOCGPKT: c_int = 0x40045438;
// TODO: Probable but not certain: #[cfg(any(target_arch = "mips", target_arch = "powerpc"))] pub const TIOCGPKT: c_int = 0x40045438;
// #[cfg(target_arch = "sparc64")] pub const TIOCGPKT: c_int = 0x40045438;

#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64"))] #[allow(overflowing_literals)] pub const TIOCGPTLCK: c_int = 0x80045439;
#[cfg(any(target_arch = "mips64", target_arch = "powerpc64"))] pub const TIOCGPTLCK: c_int = 0x40045439;
// TODO: Probable but not certain: #[cfg(any(target_arch = "mips", target_arch = "powerpc"))] pub const TIOCGPTLCK: c_int = 0x40045439;
// #[cfg(target_arch = "sparc64")] pub const TIOCGPTLCK: c_int = 0x40045439;

#[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64"))] #[allow(overflowing_literals)] pub const TIOCGEXCL: c_int = 0x80045440;
#[cfg(any(target_arch = "mips64", target_arch = "powerpc64"))] pub const TIOCGEXCL: c_int = 0x40045440;
// TODO: Probable but not certain: #[cfg(any(target_arch = "mips", target_arch = "powerpc"))] pub const TIOCGEXCL: c_int = 0x40045440;
// #[cfg(target_arch = "sparc64")] pub const TIOCGEXCL: c_int = 0x40045440;

pub const FIONCLEX: c_int = 0x5450;

pub const FIOCLEX: c_int = 0x5451;

pub const FIOASYNC: c_int = 0x5452;

pub const TIOCSERCONFIG: c_int = 0x5453;

pub const TIOCSERGWILD: c_int = 0x5454;

pub const TIOCSERSWILD: c_int = 0x5455;

pub const TIOCGLCKTRMIOS: c_int = 0x5456;

pub const TIOCSLCKTRMIOS: c_int = 0x5457;

pub const TIOCSERGSTRUCT: c_int = 0x5458;

pub const TIOCSERGETLSR: c_int = 0x5459;

pub const TIOCSERGETMULTI: c_int = 0x545A;

pub const TIOCSERSETMULTI: c_int = 0x545B;

pub const TIOCMIWAIT: c_int = 0x545C;

pub const TIOCGICOUNT: c_int = 0x545D;

cfg_if!
{
	// also alpha, sh / sh64, sparc / sparc64 and xtensa
	if #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
	{
		// see http://tomoyo.osdn.jp/cgi-bin/lxr/source/include/uapi/asm-generic/ioctl.h#L77 to work out value.
		// _IOR('f', 128, loff_t);
		pub const FIOQSIZE: c_int = 0x80086680;
	}
	else if #[cfg(any(target_arch = "mips", target_arch = "mips64"))]
	{
		pub const FIOQSIZE: c_int = 0x667F;
	}
	// also blackfin. frv and m68k
	else if #[cfg(any(target_arch = "arm", target_arch = "s390x"))]
	{
		pub const FIOQSIZE: c_int = 0x545E;
	}
	else
	{
		pub const FIOQSIZE: c_int = 0x5460;
	}
}

// Used for packet mode
pub const TIOCPKT_DATA: c_uint = 0;
pub const TIOCPKT_FLUSHREAD: c_uint = 1;
pub const TIOCPKT_FLUSHWRITE: c_uint = 2;
pub const TIOCPKT_STOP: c_uint = 4;
pub const TIOCPKT_START: c_uint = 8;
pub const TIOCPKT_NOSTOP: c_uint = 16;
pub const TIOCPKT_DOSTOP: c_uint = 32;
pub const TIOCPKT_IOCTL: c_uint = 64;

// Transmitter physically empty
pub const TIOCSER_TEMT: c_uint = 0x01;
