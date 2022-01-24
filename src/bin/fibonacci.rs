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

extern crate phi_emu;

use phi_emu::atoms::Atom;
use phi_emu::emu::Emu;
use phi_emu::obs::Obs;
use phi_emu::path::Path;
use phi_emu::ph;
use std::env;
use std::str::FromStr;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args[1].parse().unwrap();
    let cycles = args[2].parse().unwrap();
    let emu = Emu {
        obses: vec![
            Obs::Data(ph!("v3"), input),                           // v0
            Obs::Copy(ph!("v2"), vec![ph!("v0")]),                 // v1
            Obs::Abstract(ph!("v12"), vec![]),                     // v2
            Obs::Empty,                                            // v3
            Obs::Data(ph!("v3"), 0x02),                            // v4
            Obs::Atom(1, ph!("$.0"), vec![ph!("v4")]),             // v5
            Obs::Data(ph!("v3"), 0x01),                            // v6
            Obs::Atom(1, ph!("$.0"), vec![ph!("v6")]),             // v7
            Obs::Copy(ph!("v2"), vec![ph!("v7")]),                 // v8
            Obs::Copy(ph!("v2"), vec![ph!("v5")]),                 // v9
            Obs::Atom(2, ph!("v8"), vec![ph!("v9")]),              // v10
            Obs::Atom(0, ph!("$.0"), vec![ph!("v4")]),             // v11
            Obs::Atom(3, ph!("v11"), vec![ph!("v6"), ph!("v10")]), // v12
            Obs::Empty,                                            // v13
        ],
        atoms: vec![
            Atom::from_str(
                // int.less
                "
                DATAIZE $.^
                DATAIZE $.0
                SUB $.0 FROM $.^ TO #0
                JUMP less IF #0 GT
                WRITE 0x00 TO #1
                RETURN #1
                less:
                WRITE 0x01 TO #1
                RETURN #1
                ",
            )
            .unwrap(),
            Atom::from_str(
                // int.sub
                "
                DATAIZE $.^
                DATAIZE $.0
                SUB $.0 FROM $.^ TO r#0
                RETURN #0
                ",
            )
            .unwrap(),
            Atom::from_str(
                // int.add
                "
                DATAIZE $.^
                DATAIZE $.0
                ADD $.^ AND $.0 TO #0
                RETURN #0
                ",
            )
            .unwrap(),
            Atom::from_str(
                // bool.if
                "
                DATAIZE $.^
                SUB 0x01 FROM $.^ TO #0
                JUMP yes IF #0 EQ
                DATAIZE $.1
                READ $.1 TO #0
                RETURN #0
                yes:
                DATAIZE $.0
                READ $.0 TO #0
                RETURN #0
                ",
            )
            .unwrap(),
        ],
        ..Default::default()
    };
    let mut total = 0;
    let mut f = 0;
    for _ in 0..cycles {
        f = emu.dataize(0);
        total += f;
    }
    println!("{}-th Fibonacci number is {}", input, f);
    println!("Total is {}", total);
}
