/*
* This file is part of the uutils coreutils package.
*
* (c) Alex Lyon <arcterus@mail.com>
* (c) Michael Gehring <mg@ebfe.org>
*
* For the full copyright and license information, please view the LICENSE
* file that was distributed with this source code.
*/

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const CRC_TABLE_LEN: usize = 256;

#[path = "../#common/mkmain.rs"]
mod mkmain;

fn main() {
    mkmain::main();

    let out_dir = env::var("OUT_DIR").unwrap();

    let mut table = Vec::with_capacity(CRC_TABLE_LEN);
    for num in 0..CRC_TABLE_LEN {
        table.push(crc_entry(num as u8) as u32);
    }
    let file = File::create(&Path::new(&out_dir).join("crc_table.rs")).unwrap();
    write!(
        &file,
        "#[allow(clippy::unreadable_literal)]\nconst CRC_TABLE: [u32; {}] = {:?};",
        CRC_TABLE_LEN, table
    )
    .unwrap();
}

#[inline]
fn crc_entry(input: u8) -> u32 {
    let mut crc = (input as u32) << 24;

    for _ in 0..8 {
        if crc & 0x8000_0000 != 0 {
            crc <<= 1;
            crc ^= 0x04c1_1db7;
        } else {
            crc <<= 1;
        }
    }

    crc
}
