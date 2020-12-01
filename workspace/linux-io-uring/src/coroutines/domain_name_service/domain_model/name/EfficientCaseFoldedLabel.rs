// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A label.
#[derive(Copy, Clone)]
pub struct EfficientCaseFoldedLabel<'a>(&'a [u8]);

impl<'a> Deref for EfficientCaseFoldedLabel<'a>
{
	type Target = [u8];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0
	}
}

impl Default for EfficientCaseFoldedLabel<'static>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Root
	}
}

impl<'a> Debug for EfficientCaseFoldedLabel<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.display(f)
	}
}

impl<'a> Display for EfficientCaseFoldedLabel<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.display(f)
	}
}

impl<'a> PartialEq for EfficientCaseFoldedLabel<'a>
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		self.equals(rhs)
	}
}

impl<'a> Eq for EfficientCaseFoldedLabel<'a>
{
}

impl<'a> PartialOrd for EfficientCaseFoldedLabel<'a>
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		self.partial_compare(rhs)
	}
}

impl<'a> Ord for EfficientCaseFoldedLabel<'a>
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		self.compare(rhs)
	}
}

impl<'a> Hash for EfficientCaseFoldedLabel<'a>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.hash_slice(state)
	}
}

impl<'a, 'message> PartialEq<ParsedLabel<'message>> for EfficientCaseFoldedLabel<'a>
{
	#[inline(always)]
	fn eq(&self, rhs: &ParsedLabel<'message>) -> bool
	{
		self.equals(rhs)
	}
}

impl<'a, 'message> PartialOrd<ParsedLabel<'message>> for EfficientCaseFoldedLabel<'a>
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &ParsedLabel<'message>) -> Option<Ordering>
	{
		self.partial_compare(rhs)
	}
}

impl HasTypeEquality for EfficientCaseFoldedLabel
{
	type TypeEquality = OwnedTypeEquality;
}

impl<'a> Label<'a> for EfficientCaseFoldedLabel<'a>
{
	#[inline(always)]
	fn bytes_pointer(&self) -> *const u8
	{
		self.0.as_ptr()
	}
	
	#[inline(always)]
	fn len(&self) -> u8
	{
		self.0.len() as u8
	}
	
	#[inline(always)]
	fn get_unchecked_safe_case_folded_byte(&self, index: u8) -> u8
	{
		self.get_unchecked_value_safe(index)
	}
	
	#[inline(always)]
	fn get_unchecked_safe(&self, index: u8) -> &u8
	{
		self.0.get_unchecked_safe(index)
	}
}

impl<'a> TryFrom<&'a [u8]> for EfficientCaseFoldedLabel
{
	type Error = EfficientCaseFoldedLabelParseError;
	
	#[inline(always)]
	fn try_from(value: &'a [u8]) -> Result<Self, Self::Error>
	{
		Self::validate_bytes(value)?;
		Ok(Self(value))
	}
}

impl<'a> EfficientCaseFoldedLabel<'a>
{
	#[inline(always)]
	fn validate_bytes(slice: &[u8]) -> Result<(), EfficientCaseFoldedLabelParseError>
	{
		let length = slice.len();
		
		if unlikely!(length > Self::MaximumSize)
		{
			return Err(EfficientCaseFoldedLabelParseError::LabelExceeded63Bytes)
		}
		
		for index in 0 .. length
		{
			Self::validate_byte(slice.get_unchecked_safe(index))?
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn validate_byte(byte: &u8) -> Result<(), EfficientCaseFoldedLabelParseError>
	{
		use self::EfficientCaseFoldedLabelParseError::*;
		
		if unlikely!(Self::is_upper_case(byte))
		{
			Err(LabelContainedUppercaseBytes)
		}
		else if unlikely!(*byte == b'.')
		{
			Err(LabelContainedPeriod)
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn is_upper_case(byte: &u8) -> bool
	{
		let byte = *byte;
		byte >= b'A' || byte <= b'Z'
	}
}

impl EfficientCaseFoldedLabel<'static>
{
	#[inline(always)]
	const fn new(label: &'static [u8]) -> Self
	{
		Self(label)
	}
	
	pub(crate) const Root: Self = Self(b"");
	
	pub(crate) const _6tisch: Self = Self::new(b"6tisch");
	
	pub(crate) const in_addr: Self = Self::new(b"in-addr");
	
	pub(crate) const ip6: Self = Self::new(b"ip6");
	
	pub(crate) const example: Self = Self::new(b"example");
	
	pub(crate) const invalid: Self = Self::new(b"invalid");
	
	pub(crate) const ipv4only: Self = Self::new(b"ipv4only");
	
	pub(crate) const local: Self = Self::new(b"local");
	
	pub(crate) const localdomain: Self = Self::new(b"localdomain");
	
	pub(crate) const localhost: Self = Self::new(b"localhost");
	
	pub(crate) const onion: Self = Self::new(b"onion");
	
	pub(crate) const test: Self = Self::new(b"test");
	
	pub(crate) const home: Self = Self::new(b"home");
	
	pub(crate) const intranet: Self = Self::new(b"intranet");
	
	pub(crate) const internal: Self = Self::new(b"internal");
	
	pub(crate) const private: Self = Self::new(b"private");
	
	pub(crate) const corp: Self = Self::new(b"corp");
	
	pub(crate) const lan: Self = Self::new(b"lan");
	
	pub(crate) const a: Self = Self::new(b"a");
	
	pub(crate) const b: Self = Self::new(b"b");
	
	pub(crate) const c: Self = Self::new(b"c");
	
	pub(crate) const d: Self = Self::new(b"d");
	
	pub(crate) const e: Self = Self::new(b"e");
	
	pub(crate) const f: Self = Self::new(b"f");
	
	#[inline(always)]
	const fn nibble(label: u8) -> Self
	{
		match label
		{
			0 => Self::_0,
			
			1 => Self::_1,
			
			2 => Self::_2,
			
			3 => Self::_3,
			
			4 => Self::_4,
			
			5 => Self::_5,
			
			6 => Self::_6,
			
			7 => Self::_7,
			
			8 => Self::_8,
			
			9 => Self::_9,
			
			10 => Self::a,
			
			11 => Self::b,
			
			12 => Self::c,
			
			13 => Self::d,
			
			14 => Self::e,
			
			15 => Self::f,
			
			_ => unreachable_code_const("Nibble can not exceed 15"),
		}
	}
	
	#[inline(always)]
	const fn byte(label: u8) -> Self
	{
		match label
		{
			0 => Self::_0,

			1 => Self::_1,

			2 => Self::_2,

			3 => Self::_3,

			4 => Self::_4,

			5 => Self::_5,

			6 => Self::_6,

			7 => Self::_7,

			8 => Self::_8,

			9 => Self::_9,

			10 => Self::_10,
			
			11 => Self::_11,

			12 => Self::_12,

			13 => Self::_13,

			14 => Self::_14,

			15 => Self::_15,

			16 => Self::_16,

			17 => Self::_17,

			18 => Self::_18,

			19 => Self::_19,

			20 => Self::_20,

			21 => Self::_21,

			22 => Self::_22,

			23 => Self::_23,

			24 => Self::_24,

			25 => Self::_25,

			26 => Self::_26,

			27 => Self::_27,

			28 => Self::_28,

			29 => Self::_29,

			30 => Self::_30,

			31 => Self::_31,

			32 => Self::_32,

			33 => Self::_33,

			34 => Self::_34,

			35 => Self::_35,

			36 => Self::_36,

			37 => Self::_37,

			38 => Self::_38,

			39 => Self::_39,

			40 => Self::_40,

			41 => Self::_41,

			42 => Self::_42,

			43 => Self::_43,

			44 => Self::_44,

			45 => Self::_45,

			46 => Self::_46,

			47 => Self::_47,

			48 => Self::_48,

			49 => Self::_49,

			50 => Self::_50,

			51 => Self::_51,

			52 => Self::_52,

			53 => Self::_53,

			54 => Self::_54,

			55 => Self::_55,

			56 => Self::_56,

			57 => Self::_57,

			58 => Self::_58,

			59 => Self::_59,

			60 => Self::_60,

			61 => Self::_61,

			62 => Self::_62,

			63 => Self::_63,

			64 => Self::_64,

			65 => Self::_65,

			66 => Self::_66,

			67 => Self::_67,

			68 => Self::_68,

			69 => Self::_69,

			70 => Self::_70,

			71 => Self::_71,

			72 => Self::_72,

			73 => Self::_73,

			74 => Self::_74,

			75 => Self::_75,

			76 => Self::_76,

			77 => Self::_77,

			78 => Self::_78,

			79 => Self::_79,

			80 => Self::_80,

			81 => Self::_81,

			82 => Self::_82,

			83 => Self::_83,

			84 => Self::_84,

			85 => Self::_85,

			86 => Self::_86,

			87 => Self::_87,

			88 => Self::_88,

			89 => Self::_89,

			90 => Self::_90,

			91 => Self::_91,

			92 => Self::_92,

			93 => Self::_93,

			94 => Self::_94,

			95 => Self::_95,

			96 => Self::_96,

			97 => Self::_97,

			98 => Self::_98,

			99 => Self::_99,

			100 => Self::_100,

			101 => Self::_101,

			102 => Self::_102,

			103 => Self::_103,

			104 => Self::_104,

			105 => Self::_105,

			106 => Self::_106,

			107 => Self::_107,

			108 => Self::_108,

			109 => Self::_109,

			110 => Self::_110,

			111 => Self::_111,

			112 => Self::_112,

			113 => Self::_113,

			114 => Self::_114,

			115 => Self::_115,

			116 => Self::_116,

			117 => Self::_117,

			118 => Self::_118,

			119 => Self::_119,

			120 => Self::_120,

			121 => Self::_121,

			122 => Self::_122,

			123 => Self::_123,

			124 => Self::_124,

			125 => Self::_125,

			126 => Self::_126,

			127 => Self::_127,

			128 => Self::_128,

			129 => Self::_129,

			130 => Self::_130,

			131 => Self::_131,

			132 => Self::_132,

			133 => Self::_133,

			134 => Self::_134,

			135 => Self::_135,

			136 => Self::_136,

			137 => Self::_137,

			138 => Self::_138,

			139 => Self::_139,

			140 => Self::_140,

			141 => Self::_141,

			142 => Self::_142,

			143 => Self::_143,

			144 => Self::_144,

			145 => Self::_145,

			146 => Self::_146,

			147 => Self::_147,

			148 => Self::_148,

			149 => Self::_149,

			150 => Self::_150,

			151 => Self::_151,

			152 => Self::_152,

			153 => Self::_153,

			154 => Self::_154,

			155 => Self::_155,

			156 => Self::_156,

			157 => Self::_157,

			158 => Self::_158,

			159 => Self::_159,

			160 => Self::_160,

			161 => Self::_161,

			162 => Self::_162,

			163 => Self::_163,

			164 => Self::_164,

			165 => Self::_165,

			166 => Self::_166,

			167 => Self::_167,

			168 => Self::_168,

			169 => Self::_169,

			170 => Self::_170,

			171 => Self::_171,

			172 => Self::_172,

			173 => Self::_173,

			174 => Self::_174,

			175 => Self::_175,

			176 => Self::_176,

			177 => Self::_177,

			178 => Self::_178,

			179 => Self::_179,

			180 => Self::_180,

			181 => Self::_181,

			182 => Self::_182,

			183 => Self::_183,

			184 => Self::_184,

			185 => Self::_185,

			186 => Self::_186,

			187 => Self::_187,

			188 => Self::_188,

			189 => Self::_189,

			190 => Self::_190,

			191 => Self::_191,

			192 => Self::_192,

			193 => Self::_193,

			194 => Self::_194,

			195 => Self::_195,

			196 => Self::_196,

			197 => Self::_197,

			198 => Self::_198,

			199 => Self::_199,

			200 => Self::_200,

			201 => Self::_201,

			202 => Self::_202,

			203 => Self::_203,

			204 => Self::_204,

			205 => Self::_205,

			206 => Self::_206,

			207 => Self::_207,

			208 => Self::_208,

			209 => Self::_209,

			210 => Self::_210,

			211 => Self::_211,

			212 => Self::_212,

			213 => Self::_213,

			214 => Self::_214,

			215 => Self::_215,

			216 => Self::_216,

			217 => Self::_217,

			218 => Self::_218,

			219 => Self::_219,

			220 => Self::_220,

			221 => Self::_221,

			222 => Self::_222,

			223 => Self::_223,

			224 => Self::_224,

			225 => Self::_225,

			226 => Self::_226,

			227 => Self::_227,

			228 => Self::_228,

			229 => Self::_229,

			230 => Self::_230,

			231 => Self::_231,

			232 => Self::_232,

			233 => Self::_233,

			234 => Self::_234,

			235 => Self::_235,

			236 => Self::_236,

			237 => Self::_237,

			238 => Self::_238,

			239 => Self::_239,

			240 => Self::_240,

			241 => Self::_241,

			242 => Self::_242,

			243 => Self::_243,

			244 => Self::_244,

			245 => Self::_245,

			246 => Self::_246,

			247 => Self::_247,

			248 => Self::_248,

			249 => Self::_249,

			250 => Self::_250,

			251 => Self::_251,

			252 => Self::_252,

			253 => Self::_253,

			254 => Self::_254,

			255 => Self::_255,
		}
	}
	
	pub(crate) const _0: Self = Self::new(b"0");
	
	pub(crate) const _1: Self = Self::new(b"1");
	
	pub(crate) const _2: Self = Self::new(b"2");
	
	pub(crate) const _3: Self = Self::new(b"3");
	
	pub(crate) const _4: Self = Self::new(b"4");
	
	pub(crate) const _5: Self = Self::new(b"5");
	
	pub(crate) const _6: Self = Self::new(b"6");
	
	pub(crate) const _7: Self = Self::new(b"7");
	
	pub(crate) const _8: Self = Self::new(b"8");
	
	pub(crate) const _9: Self = Self::new(b"9");
	
	pub(crate) const _10: Self = Self::new(b"10");
	
	pub(crate) const _11: Self = Self::new(b"11");
	
	pub(crate) const _12: Self = Self::new(b"12");
	
	pub(crate) const _13: Self = Self::new(b"13");
	
	pub(crate) const _14: Self = Self::new(b"14");
	
	pub(crate) const _15: Self = Self::new(b"15");
	
	pub(crate) const _16: Self = Self::new(b"16");
	
	pub(crate) const _17: Self = Self::new(b"17");
	
	pub(crate) const _18: Self = Self::new(b"18");
	
	pub(crate) const _19: Self = Self::new(b"19");
	
	pub(crate) const _20: Self = Self::new(b"20");
	
	pub(crate) const _21: Self = Self::new(b"21");
	
	pub(crate) const _22: Self = Self::new(b"22");
	
	pub(crate) const _23: Self = Self::new(b"23");
	
	pub(crate) const _24: Self = Self::new(b"24");
	
	pub(crate) const _25: Self = Self::new(b"25");
	
	pub(crate) const _26: Self = Self::new(b"26");
	
	pub(crate) const _27: Self = Self::new(b"27");
	
	pub(crate) const _28: Self = Self::new(b"28");
	
	pub(crate) const _29: Self = Self::new(b"29");
	
	pub(crate) const _30: Self = Self::new(b"30");
	
	pub(crate) const _31: Self = Self::new(b"31");
	
	pub(crate) const _32: Self = Self::new(b"32");
	
	pub(crate) const _33: Self = Self::new(b"33");
	
	pub(crate) const _34: Self = Self::new(b"34");
	
	pub(crate) const _35: Self = Self::new(b"35");
	
	pub(crate) const _36: Self = Self::new(b"36");
	
	pub(crate) const _37: Self = Self::new(b"37");
	
	pub(crate) const _38: Self = Self::new(b"38");
	
	pub(crate) const _39: Self = Self::new(b"39");
	
	pub(crate) const _40: Self = Self::new(b"40");
	
	pub(crate) const _41: Self = Self::new(b"41");
	
	pub(crate) const _42: Self = Self::new(b"42");
	
	pub(crate) const _43: Self = Self::new(b"43");
	
	pub(crate) const _44: Self = Self::new(b"44");
	
	pub(crate) const _45: Self = Self::new(b"45");
	
	pub(crate) const _46: Self = Self::new(b"46");
	
	pub(crate) const _47: Self = Self::new(b"47");
	
	pub(crate) const _48: Self = Self::new(b"48");
	
	pub(crate) const _49: Self = Self::new(b"49");
	
	pub(crate) const _50: Self = Self::new(b"50");
	
	pub(crate) const _51: Self = Self::new(b"51");
	
	pub(crate) const _52: Self = Self::new(b"52");
	
	pub(crate) const _53: Self = Self::new(b"53");
	
	pub(crate) const _54: Self = Self::new(b"54");
	
	pub(crate) const _55: Self = Self::new(b"55");
	
	pub(crate) const _56: Self = Self::new(b"56");
	
	pub(crate) const _57: Self = Self::new(b"57");
	
	pub(crate) const _58: Self = Self::new(b"58");
	
	pub(crate) const _59: Self = Self::new(b"59");
	
	pub(crate) const _60: Self = Self::new(b"60");
	
	pub(crate) const _61: Self = Self::new(b"61");
	
	pub(crate) const _62: Self = Self::new(b"62");
	
	pub(crate) const _63: Self = Self::new(b"63");
	
	pub(crate) const _64: Self = Self::new(b"64");
	
	pub(crate) const _65: Self = Self::new(b"65");
	
	pub(crate) const _66: Self = Self::new(b"66");
	
	pub(crate) const _67: Self = Self::new(b"67");
	
	pub(crate) const _68: Self = Self::new(b"68");
	
	pub(crate) const _69: Self = Self::new(b"69");
	
	pub(crate) const _70: Self = Self::new(b"70");
	
	pub(crate) const _71: Self = Self::new(b"71");
	
	pub(crate) const _72: Self = Self::new(b"72");
	
	pub(crate) const _73: Self = Self::new(b"73");
	
	pub(crate) const _74: Self = Self::new(b"74");
	
	pub(crate) const _75: Self = Self::new(b"75");
	
	pub(crate) const _76: Self = Self::new(b"76");
	
	pub(crate) const _77: Self = Self::new(b"77");
	
	pub(crate) const _78: Self = Self::new(b"78");
	
	pub(crate) const _79: Self = Self::new(b"79");
	
	pub(crate) const _80: Self = Self::new(b"80");
	
	pub(crate) const _81: Self = Self::new(b"81");
	
	pub(crate) const _82: Self = Self::new(b"82");
	
	pub(crate) const _83: Self = Self::new(b"83");
	
	pub(crate) const _84: Self = Self::new(b"84");
	
	pub(crate) const _85: Self = Self::new(b"85");
	
	pub(crate) const _86: Self = Self::new(b"86");
	
	pub(crate) const _87: Self = Self::new(b"87");
	
	pub(crate) const _88: Self = Self::new(b"88");
	
	pub(crate) const _89: Self = Self::new(b"89");
	
	pub(crate) const _90: Self = Self::new(b"90");
	
	pub(crate) const _91: Self = Self::new(b"91");
	
	pub(crate) const _92: Self = Self::new(b"92");
	
	pub(crate) const _93: Self = Self::new(b"93");
	
	pub(crate) const _94: Self = Self::new(b"94");
	
	pub(crate) const _95: Self = Self::new(b"95");
	
	pub(crate) const _96: Self = Self::new(b"96");
	
	pub(crate) const _97: Self = Self::new(b"97");
	
	pub(crate) const _98: Self = Self::new(b"98");
	
	pub(crate) const _99: Self = Self::new(b"99");
	
	pub(crate) const _100: Self = Self::new(b"100");
	
	pub(crate) const _101: Self = Self::new(b"101");
	
	pub(crate) const _102: Self = Self::new(b"102");
	
	pub(crate) const _103: Self = Self::new(b"103");
	
	pub(crate) const _104: Self = Self::new(b"104");
	
	pub(crate) const _105: Self = Self::new(b"105");
	
	pub(crate) const _106: Self = Self::new(b"106");
	
	pub(crate) const _107: Self = Self::new(b"107");
	
	pub(crate) const _108: Self = Self::new(b"108");
	
	pub(crate) const _109: Self = Self::new(b"109");
	
	pub(crate) const _110: Self = Self::new(b"110");
	
	pub(crate) const _111: Self = Self::new(b"111");
	
	pub(crate) const _112: Self = Self::new(b"112");
	
	pub(crate) const _113: Self = Self::new(b"113");
	
	pub(crate) const _114: Self = Self::new(b"114");
	
	pub(crate) const _115: Self = Self::new(b"115");
	
	pub(crate) const _116: Self = Self::new(b"116");
	
	pub(crate) const _117: Self = Self::new(b"117");
	
	pub(crate) const _118: Self = Self::new(b"118");
	
	pub(crate) const _119: Self = Self::new(b"119");
	
	pub(crate) const _120: Self = Self::new(b"120");
	
	pub(crate) const _121: Self = Self::new(b"121");
	
	pub(crate) const _122: Self = Self::new(b"122");
	
	pub(crate) const _123: Self = Self::new(b"123");
	
	pub(crate) const _124: Self = Self::new(b"124");
	
	pub(crate) const _125: Self = Self::new(b"125");
	
	pub(crate) const _126: Self = Self::new(b"126");
	
	pub(crate) const _127: Self = Self::new(b"127");
	
	pub(crate) const _128: Self = Self::new(b"128");
	
	pub(crate) const _129: Self = Self::new(b"129");
	
	pub(crate) const _130: Self = Self::new(b"130");
	
	pub(crate) const _131: Self = Self::new(b"131");
	
	pub(crate) const _132: Self = Self::new(b"132");
	
	pub(crate) const _133: Self = Self::new(b"133");
	
	pub(crate) const _134: Self = Self::new(b"134");
	
	pub(crate) const _135: Self = Self::new(b"135");
	
	pub(crate) const _136: Self = Self::new(b"136");
	
	pub(crate) const _137: Self = Self::new(b"137");
	
	pub(crate) const _138: Self = Self::new(b"138");
	
	pub(crate) const _139: Self = Self::new(b"139");
	
	pub(crate) const _140: Self = Self::new(b"140");
	
	pub(crate) const _141: Self = Self::new(b"141");
	
	pub(crate) const _142: Self = Self::new(b"142");
	
	pub(crate) const _143: Self = Self::new(b"143");
	
	pub(crate) const _144: Self = Self::new(b"144");
	
	pub(crate) const _145: Self = Self::new(b"145");
	
	pub(crate) const _146: Self = Self::new(b"146");
	
	pub(crate) const _147: Self = Self::new(b"147");
	
	pub(crate) const _148: Self = Self::new(b"148");
	
	pub(crate) const _149: Self = Self::new(b"149");
	
	pub(crate) const _150: Self = Self::new(b"150");
	
	pub(crate) const _151: Self = Self::new(b"151");
	
	pub(crate) const _152: Self = Self::new(b"152");
	
	pub(crate) const _153: Self = Self::new(b"153");
	
	pub(crate) const _154: Self = Self::new(b"154");
	
	pub(crate) const _155: Self = Self::new(b"155");
	
	pub(crate) const _156: Self = Self::new(b"156");
	
	pub(crate) const _157: Self = Self::new(b"157");
	
	pub(crate) const _158: Self = Self::new(b"158");
	
	pub(crate) const _159: Self = Self::new(b"159");
	
	pub(crate) const _160: Self = Self::new(b"160");
	
	pub(crate) const _161: Self = Self::new(b"161");
	
	pub(crate) const _162: Self = Self::new(b"162");
	
	pub(crate) const _163: Self = Self::new(b"163");
	
	pub(crate) const _164: Self = Self::new(b"164");
	
	pub(crate) const _165: Self = Self::new(b"165");
	
	pub(crate) const _166: Self = Self::new(b"166");
	
	pub(crate) const _167: Self = Self::new(b"167");
	
	pub(crate) const _168: Self = Self::new(b"168");
	
	pub(crate) const _169: Self = Self::new(b"169");
	
	pub(crate) const _170: Self = Self::new(b"170");
	
	pub(crate) const _171: Self = Self::new(b"171");
	
	pub(crate) const _172: Self = Self::new(b"172");
	
	pub(crate) const _173: Self = Self::new(b"173");
	
	pub(crate) const _174: Self = Self::new(b"174");
	
	pub(crate) const _175: Self = Self::new(b"175");
	
	pub(crate) const _176: Self = Self::new(b"176");
	
	pub(crate) const _177: Self = Self::new(b"177");
	
	pub(crate) const _178: Self = Self::new(b"178");
	
	pub(crate) const _179: Self = Self::new(b"179");
	
	pub(crate) const _180: Self = Self::new(b"180");
	
	pub(crate) const _181: Self = Self::new(b"181");
	
	pub(crate) const _182: Self = Self::new(b"182");
	
	pub(crate) const _183: Self = Self::new(b"183");
	
	pub(crate) const _184: Self = Self::new(b"184");
	
	pub(crate) const _185: Self = Self::new(b"185");
	
	pub(crate) const _186: Self = Self::new(b"186");
	
	pub(crate) const _187: Self = Self::new(b"187");
	
	pub(crate) const _188: Self = Self::new(b"188");
	
	pub(crate) const _189: Self = Self::new(b"189");
	
	pub(crate) const _190: Self = Self::new(b"190");
	
	pub(crate) const _191: Self = Self::new(b"191");
	
	pub(crate) const _192: Self = Self::new(b"192");
	
	pub(crate) const _193: Self = Self::new(b"193");
	
	pub(crate) const _194: Self = Self::new(b"194");
	
	pub(crate) const _195: Self = Self::new(b"195");
	
	pub(crate) const _196: Self = Self::new(b"196");
	
	pub(crate) const _197: Self = Self::new(b"197");
	
	pub(crate) const _198: Self = Self::new(b"198");
	
	pub(crate) const _199: Self = Self::new(b"199");
	
	pub(crate) const _200: Self = Self::new(b"200");
	
	pub(crate) const _201: Self = Self::new(b"201");
	
	pub(crate) const _202: Self = Self::new(b"202");
	
	pub(crate) const _203: Self = Self::new(b"203");
	
	pub(crate) const _204: Self = Self::new(b"204");
	
	pub(crate) const _205: Self = Self::new(b"205");
	
	pub(crate) const _206: Self = Self::new(b"206");
	
	pub(crate) const _207: Self = Self::new(b"207");
	
	pub(crate) const _208: Self = Self::new(b"208");
	
	pub(crate) const _209: Self = Self::new(b"209");
	
	pub(crate) const _210: Self = Self::new(b"210");
	
	pub(crate) const _211: Self = Self::new(b"211");
	
	pub(crate) const _212: Self = Self::new(b"212");
	
	pub(crate) const _213: Self = Self::new(b"213");
	
	pub(crate) const _214: Self = Self::new(b"214");
	
	pub(crate) const _215: Self = Self::new(b"215");
	
	pub(crate) const _216: Self = Self::new(b"216");
	
	pub(crate) const _217: Self = Self::new(b"217");
	
	pub(crate) const _218: Self = Self::new(b"218");
	
	pub(crate) const _219: Self = Self::new(b"219");
	
	pub(crate) const _220: Self = Self::new(b"220");
	
	pub(crate) const _221: Self = Self::new(b"221");
	
	pub(crate) const _222: Self = Self::new(b"222");
	
	pub(crate) const _223: Self = Self::new(b"223");
	
	pub(crate) const _224: Self = Self::new(b"224");
	
	pub(crate) const _225: Self = Self::new(b"225");
	
	pub(crate) const _226: Self = Self::new(b"226");
	
	pub(crate) const _227: Self = Self::new(b"227");
	
	pub(crate) const _228: Self = Self::new(b"228");
	
	pub(crate) const _229: Self = Self::new(b"229");
	
	pub(crate) const _230: Self = Self::new(b"230");
	
	pub(crate) const _231: Self = Self::new(b"231");
	
	pub(crate) const _232: Self = Self::new(b"232");
	
	pub(crate) const _233: Self = Self::new(b"233");
	
	pub(crate) const _234: Self = Self::new(b"234");
	
	pub(crate) const _235: Self = Self::new(b"235");
	
	pub(crate) const _236: Self = Self::new(b"236");
	
	pub(crate) const _237: Self = Self::new(b"237");
	
	pub(crate) const _238: Self = Self::new(b"238");
	
	pub(crate) const _239: Self = Self::new(b"239");
	
	pub(crate) const _240: Self = Self::new(b"240");
	
	pub(crate) const _241: Self = Self::new(b"241");
	
	pub(crate) const _242: Self = Self::new(b"242");
	
	pub(crate) const _243: Self = Self::new(b"243");
	
	pub(crate) const _244: Self = Self::new(b"244");
	
	pub(crate) const _245: Self = Self::new(b"245");
	
	pub(crate) const _246: Self = Self::new(b"246");
	
	pub(crate) const _247: Self = Self::new(b"247");
	
	pub(crate) const _248: Self = Self::new(b"248");
	
	pub(crate) const _249: Self = Self::new(b"249");
	
	pub(crate) const _250: Self = Self::new(b"250");
	
	pub(crate) const _251: Self = Self::new(b"251");
	
	pub(crate) const _252: Self = Self::new(b"252");
	
	pub(crate) const _253: Self = Self::new(b"253");
	
	pub(crate) const _254: Self = Self::new(b"254");
	
	pub(crate) const _255: Self = Self::new(b"255");
	
	/// Language: Arabic.
	/// Script: Arabic
	/// Non-punycode name `.إختبار.`
	/// Listed as a domain of type `test` in <https://www.iana.org/domains/root/db>.
	pub(crate) const Arabic_Arabic: Self = Self::new(b"xn--kgbechtv");
	
	/// Language: Persian.
	/// Script: Arabic.
	/// Non-punycode name `.آزمایشی.`.
	/// Listed as a domain of type `test` in <https://www.iana.org/domains/root/db>.
	pub(crate) const Persian_Arabic: Self = Self::new(b"xn--hgbk6aj7f53bba");
	
	/// Language: Chinese.
	/// Script: Han (Simplified variant).
	/// Non-punycode name `.测试.`.
	/// Listed as a domain of type `test` in <https://www.iana.org/domains/root/db>.
	pub(crate) const Chinese_Han_Simplified: Self = Self::new(b"xn--0zwm56d");
	
	/// Language: Chinese.
	/// Script: Han (Traditional variant).
	/// Non-punycode name `.測試.`.
	/// Listed as a domain of type `test` in <https://www.iana.org/domains/root/db>.
	pub(crate) const Chinese_Han_Traditional: Self = Self::new(b"xn--g6w251d");
	
	/// Language: Russian.
	/// Script: Cyrillic.
	/// Non-punycode name `.испытание.`.
	/// Listed as a domain of type `test` in <https://www.iana.org/domains/root/db>.
	pub(crate) const Russion_Cyrillic: Self = Self::new(b"xn--80akhbyknj4f");
	
	/// Language: Hindi.
	/// Script: Devanagari (Nagari).
	/// Non-punycode name `.परीक्षा.`.
	/// Listed as a domain of type `test` in <https://www.iana.org/domains/root/db>.
	pub(crate) const Hindi_Devangari: Self = Self::new(b"xn--11b5bs3a9aj6g");
	
	/// Language: Greek, Modern (1453-).
	/// Script: Greek.
	/// Non-punycode name `.δοκιμή.`.
	/// Listed as a domain of type `test` in <https://www.iana.org/domains/root/db>.
	pub(crate) const Greek_Greek: Self = Self::new(b"xn--jxalpdlp");
	
	/// Language: Korean.
	/// Script: Hangul (Hangŭl, Hangeul).
	/// Non-punycode name `.테스트.`.
	/// Listed as a domain of type `test` in <https://www.iana.org/domains/root/db>.
	pub(crate) const Korean_Hangul: Self = Self::new(b"xn--9t4b11yi5a");
	
	/// Language: Yiddish
	/// Script: Hebrew
	/// Non-punycode name `.טעסט.`
	/// Listed as a domain of type `test` in <https://www.iana.org/domains/root/db>.
	pub(crate) const Yiddish_Hebrew: Self = Self::new(b"xn--deba0ad");
	
	/// Language: Japanese.
	/// Script: Katakana.
	/// Non-punycode name `.テスト.`.
	/// Listed as a domain of type `test` in <https://www.iana.org/domains/root/db>.
	pub(crate) const Japanese_Katakana: Self = Self::new(b"xn--zckzah");
	
	/// Language: Tamil.
	/// Script: Tamil.
	/// Non-punycode name `.பரிட்சை.`.
	/// Listed as a domain of type `test` in <https://www.iana.org/domains/root/db>.
	pub(crate) const Tamil_Tamil: Self = Self::new(b"xn--hlcj6aya9esc7a");
}

include!(concat!(env!("OUT_DIR"), "/EfficientCaseFoldedLabel.top_level_domains.rs"));
