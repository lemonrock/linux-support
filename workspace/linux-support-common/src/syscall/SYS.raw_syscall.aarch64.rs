// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


impl SYS
{
    /// Raw syscall for 0 arguments.
    #[inline(always)]
    pub unsafe fn raw_syscall0(self) -> usize
    {
        let x8 = self as usize;
        let x0: usize;
        asm!
        (
            "svc $$0"
            :
                "={x0}"(x0)
            :
                "{x8}"(x8)
            :
                "cc",
                "memory"
            :
                "volatile"
        );
        x0
    }

    /// Raw syscall for 1 argument.
    #[inline(always)]
    pub unsafe fn raw_syscall1(self, a: usize) -> usize
    {
        let x8 = self as usize;
        let mut x0 = a;
        asm!
        (
            "svc $$0"
            :
                "+{x0}"(x0)
            :
                "{x8}"(x8)
            :
                "cc",
                "memory"
            :
                "volatile"
        );
        x0
    }

    /// Raw syscall for 2 arguments.
    #[inline(always)]
    pub unsafe fn raw_syscall2(self, mut a: usize, mut b: usize) -> usize
    {
        let x8 = self as usize;
        let mut x0 = a;
        asm!
        (
            "svc $$0"
            :
                "+{x0}"(x0)
            :
                "{x8}"(x8),
                "{x1}"(b)
            :
                "cc",
                "memory"
            :
                "volatile"
        );
        x0
    }

    /// Raw syscall for 3 arguments.
    #[inline(always)]
    pub unsafe fn raw_syscall3(self, mut a: usize, b: usize, c: usize) -> usize
    {
        let x8 = self as usize;
        let mut x0 = a;
        asm!
        (
            "svc $$0"
            :
                "+{x0}"(x0)
            :
                "{x8}"(x8),
                "{x1}"(b),
                "{x2}"(c)
            :
                "cc",
                "memory"
            :
                "volatile"
        );
        x0
    }

    /// Raw syscall for 4 arguments.
    #[inline(always)]
    pub unsafe fn raw_syscall4(self, a: usize, b: usize, c: usize, d: usize) -> usize
    {
        let x8 = self as usize;
        let mut x0 = a;
        asm!
        (
            "svc $$0"
            :
                "+{x0}"(x0)
            :
                "{x8}"(x8),
                "{x1}"(b),
                "{x2}"(c)
                "{x3}"(d)
            :
                "cc",
                "memory"
            :
                "volatile"
        );
        x0
    }

    /// Raw syscall for 5 arguments.
    #[inline(always)]
    pub unsafe fn raw_syscall5(self, a: usize, b: usize, c: usize, d: usize, e: usize) -> usize
    {
        let x8 = self as usize;
        let mut x0 = a;
        asm!
        (
            "svc $$0"
            :
                "+{x0}"(x0)
            :
                "{x8}"(x8),
                "{x1}"(b),
                "{x2}"(c)
                "{x3}"(d)
                "{x4}"(e)
            :
                "cc",
                "memory"
            :
                "volatile"
        );
        x0
    }

    /// Raw syscall for 6 arguments.
    #[inline(always)]
    pub unsafe fn raw_syscall6(self, a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> usize
    {
        let x8 = self as usize;
        let mut x0 = a;
        asm!
        (
            "svc $$0"
            :
                "+{x0}"(x0)
            :
                "{x8}"(x8),
                "{x1}"(b),
                "{x2}"(c)
                "{x3}"(d)
                "{x4}"(e)
                "{x5}"(f)
            :
                "cc",
                "memory"
            :
                "volatile"
        );
        x0
    }
}
