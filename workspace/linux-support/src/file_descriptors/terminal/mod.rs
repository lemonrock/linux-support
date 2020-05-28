// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use super::*;
use super::character_device::CharacterDeviceFileDescriptor;
use super::pipes_and_fifos::SpliceRecipient;
use super::pipes_and_fifos::SpliceSender;
use crate::process::ProcessGroupIdentifier;
use crate::terminal::*;
use crate::terminal::c::tcdrain;
use crate::terminal::c::tcflow;
use crate::terminal::c::tcflush;
use crate::terminal::c::TCIFLUSH;
use crate::terminal::c::TCIOFF;
use crate::terminal::c::TCIOFLUSH;
use crate::terminal::c::TCION;
use crate::terminal::c::tcgetattr;
use crate::terminal::c::tcgetsid;
use crate::terminal::c::TCOFLUSH;
use crate::terminal::c::TCOOFF;
use crate::terminal::c::TCOON;
use crate::terminal::c::tcsendbreak;
use crate::terminal::c::tcsetattr;
use crate::terminal::c::termios;
use crate::vectors::VectoredRead;
use crate::vectors::VectoredWrite;


include!("TerminalFileDescriptor.rs");
