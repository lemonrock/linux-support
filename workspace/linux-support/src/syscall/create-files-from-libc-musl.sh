#!/usr/bin/env sh
# This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


print_header()
{
	cat <<EOF
// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


EOF
}

prepare()
{
	rm -rf *.tmp
}

create_constant_files()
{
	local arch
	for arch in 'aarch64' 'mips64' 'powerpc64' 'riscv64' 's390x' 'x86_64'
	do
		local define
		local name
		local value
		while read -r define name value
		do
			if [ "$define" != '#define' ]; then
				continue
			fi

			local refinedName="$(printf '%s' "$name" | sed -e 's/^__NR_//g')"
			if [ "$refinedName" = 'break' ]; then
				refinedName='break_'
			fi

			local file="$refinedName".tmp
			printf '\t#[cfg(target_arch = "%s")] #[allow(missing_docs)] %s = %s,\n' "$arch" "$refinedName" "$value" >>"$file"

		done <"$LOCATION_OF_MUSL"/arch/"$arch"/bits/syscall.h.in
	done
}

create_include_file()
{
	{
		print_header

		cat <<EOF
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SystemCalls
{
EOF

		local file
		for file in *.tmp
		do
			cat "$file"
			printf '\n'
			rm "$file"
		done

		cat <<EOF
}
EOF
	} >SystemCalls.constants.rs
}

LOCATION_OF_MUSL=./musl
prepare
create_constant_files
create_include_file
