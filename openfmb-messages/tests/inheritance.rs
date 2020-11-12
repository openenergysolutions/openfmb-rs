use openfmb_messages::{commonmodule::*, switchmodule::*};

#[test]
fn test_inheritance_accessor() {
    let msg_info = MessageInfo::default();
    assert_eq!(msg_info.m_rid(), msg_info.identified_object().m_rid());
}

#[test]
fn test_nested_inheritance_accessor() {
    let ctl_profile = SwitchDiscreteControlProfile::default();
    assert_eq!(ctl_profile.m_rid(), ctl_profile.message_info().m_rid());
}
