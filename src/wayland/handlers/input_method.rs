// SPDX-License-Identifier: GPL-3.0-only

// TODO: Restrict what applications can use protocol? At least don't allow in
// sandboxed app.

use crate::state::State;
use smithay::delegate_input_method_manager;

delegate_input_method_manager!(State);
