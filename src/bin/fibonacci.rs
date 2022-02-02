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

extern crate eoc;

use eoc::atom::{*};
use eoc::emu::Emu;
use eoc::object::Object;
use eoc::path::{Item, Path};
use eoc::ph;
use std::env;
use std::str::FromStr;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args[1].parse().unwrap();
    let cycles = args[2].parse().unwrap();
    let mut emu = Emu::empty();
    emu.put(0, Object::dataic(input));
    emu.put(1, Object::empty().with(Item::Phi, ph!("v0")));
    emu.put(2, Object::empty().with(Item::Phi, ph!("v12")));
    emu.put(4, Object::dataic(2));
    emu.put(5, Object::atomic(int_sub).with(Item::Rho, ph!("$.0")).with(Item::Arg(0), ph!("v4")));
    let mut total = 0;
    let mut f = 0;
    for _ in 0..cycles {
        let bx = emu.new(1, 1);
        f = emu.dataize(bx);
        emu.delete(bx);
        total += f;
    }
    println!("{}-th Fibonacci number is {}", input, f);
    println!("Total is {}", total);
}
