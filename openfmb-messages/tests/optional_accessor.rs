use openfmb_messages::{commonmodule::*};

#[test]
fn ensure_optional_or_default() {
    let msg_info = MessageInfo::default();
    msg_info.identified_object();
}
