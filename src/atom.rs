// Copyright (c) 2022 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::emu::Emu;
use crate::path::Item;
use crate::data::Data;

pub type Atom = fn(&mut Emu, usize, usize) -> Data;

pub fn int_add(emu: &mut Emu, ob: usize, bx: usize) -> Data {
    emu.calc(ob, Item::Rho, bx) + emu.calc(ob, Item::Arg(0), bx)
}

pub fn int_sub(emu: &mut Emu, ob: usize, bx: usize) -> Data {
    emu.calc(ob, Item::Rho, bx) - emu.calc(ob, Item::Arg(0), bx)
}

pub fn int_less(emu: &mut Emu, ob: usize, bx: usize) -> Data {
    (emu.calc(ob, Item::Rho, bx) < emu.calc(ob, Item::Arg(0), bx)) as Data
}

pub fn bool_if(emu: &mut Emu, ob: usize, bx: usize) -> Data {
    if emu.calc(ob, Item::Rho, bx) == 1 {
        emu.calc(ob, Item::Arg(0), bx)
    } else {
        emu.calc(ob, Item::Arg(1), bx)
    }
}
