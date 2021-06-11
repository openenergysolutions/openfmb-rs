// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use openfmb_messages::commonmodule::*;

#[test]
fn ensure_optional_or_default() {
    let msg_info = MessageInfo::default();
    msg_info.identified_object();
}
