use steamicon::shortcut;

#[test]
fn test_shortcut_new() {
    let shortcut = shortcut::Shortcut::new("tests/samples/Warframe.url".into());
}

#[test]
fn test_shortcut_read() {
    let mut shortcut = shortcut::Shortcut::new("tests/samples/Warframe.url".into());
    shortcut.read();

    assert_eq!(shortcut.appid, 230410);
    assert_eq!(shortcut.name, "Warframe".to_string());
    assert_eq!(
        shortcut.icon.hash,
        Some("3b8ddd969a289cdf233fafe0a77ca70655b9b2e1".to_string())
    );
}
