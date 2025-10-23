use steamicon::shortcut;

#[test]
fn test_shortcut_new() {
    let _ = shortcut::Shortcut::new("tests/samples/Warframe.url".into());
}

#[test]
fn test_shortcut_read() {
    let shortcut = shortcut::Shortcut::new("tests/samples/Warframe.url".into())
        .read()
        .unwrap();

    assert_eq!(shortcut.appid, 230410);
    assert_eq!(shortcut.name, "Warframe".to_string());
    assert_eq!(
        shortcut.icon.hash,
        Some("3b8ddd969a289cdf233fafe0a77ca70655b9b2e1".to_string())
    );
}

#[test]
fn test_shortcut_url() {
    let shortcut = shortcut::Shortcut::new("tests/samples/Warframe.url".into())
        .read()
        .unwrap();
    let target = "https://media.steampowered.com/steamcommunity/public/images/apps/230410/3b8ddd969a289cdf233fafe0a77ca70655b9b2e1.ico";

    assert_eq!(shortcut.icon_url(), target);
}
