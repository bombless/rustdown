// Copyright 2014 The Rustdown Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use types::MarkdownStructure;
use types::MarkdownStructure::{MDH, MDP, MDB};
use types::parse_heading;
use types::parse_paragraph;
use types::parse_bulletlist;

pub fn parse_block(block: &Vec<String>) -> MarkdownStructure {
    match parse_heading(block) {
        Some(heading) => return MDH(heading),
        None => {}
    }
    match parse_bulletlist(block) {
        Some(bullets) => return MDB(bullets),
        None => {}
    }
    MDP(parse_paragraph(block))
}
