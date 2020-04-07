// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::c::*;
use crate::strings::Radix;
use crate::strings::parse_number::*;
use errno::errno;
use errno::Errno;
use libc::c_int;
use libc::c_uchar;
use libc::c_uint;
use libc::EINVAL;
use libc::pid_t;
use likely::*;
use std::collections::BTreeMap;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute;
use std::ops::Deref;
use std::ops::DerefMut;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;


pub(crate) mod c;


include!("BackspaceDelay.rs");
include!("BaudRate.rs");
include!("BitsPerByte.rs");
include!("CanonicalEchoKillCharacter.rs");
include!("CanonicalSettings.rs");
include!("CarriageReturnDelay.rs");
include!("Character.rs");
include!("CharacterSettings.rs");
include!("ControlModeFlagSettings.rs");
include!("CurrentTerminalSettings.rs");
include!("ControllingTerminal.rs");
include!("Echo.rs");
include!("FlagSetting.rs");
include!("FormFeedDelay.rs");
include!("HorizontalTabDelay.rs");
include!("InputModeFlag.rs");
include!("InputModeFlagSettings.rs");
include!("LocalModeFlagSettings.rs");
include!("MiscellaneousControlModeFlag.rs");
include!("MiscellaneousControlModeFlagSettings.rs");
include!("MiscellaneousLocalModeFlag.rs");
include!("MiscellaneousLocalModeFlagSettings.rs");
include!("MiscellaneousOutputModeFlag.rs");
include!("MiscellaneousOutputModeFlagSettings.rs");
include!("MultipleBits.rs");
include!("NewLineDelay.rs");
include!("OutputModeFlagSettings.rs");
include!("Parity.rs");
include!("SignalRaising.rs");
include!("StopBits.rs");
include!("TerminalMode.rs");
include!("TerminalSettings.rs");
include!("TerminalSettingsError.rs");
include!("VerticalTabDelay.rs");
include!("WhenToChangeTerminalAttributes.rs");
