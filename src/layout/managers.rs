// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::*;

pub struct Stack<Window: PartialEq + 'static> {
	layout: TilingLayout<Window>,
}

// TODO: implement TilingWindowManager
// TODO: spiral layout
