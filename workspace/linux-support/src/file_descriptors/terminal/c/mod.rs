// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use super::*;
use libc::c_uchar;
use libc::c_uint;


include!("_POSIX_VDISABLE.rs");
include!("B.rs");
include!("C.rs");
include!("cc_t.rs");
include!("cfgetispeed.rs");
include!("cfgetospeed.rs");
include!("cfmakeraw.rs");
include!("cfsetispeed.rs");
include!("cfsetospeed.rs");
include!("cfsetspeed.rs");
include!("EXT.rs");
include!("I.rs");
include!("L.rs");
include!("NCCS.rs");
include!("O.rs");
include!("speed_t.rs");
include!("TCSA.rs");
include!("tcdrain.rs");
include!("tcflag_t.rs");
include!("tcflow.rs");
include!("tcflush.rs");
include!("tcgetattr.rs");
include!("tcgetsid.rs");
include!("tcsendbreak.rs");
include!("tcsetattr.rs");
include!("termios.rs");
include!("V.rs");
