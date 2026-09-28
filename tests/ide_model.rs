use dev_cli::models::ide::Ide;

#[test]
fn test_ide_display_name_and_icon() {
    let ides = [
        Ide::Antigravity,
        Ide::AntigravityCli,
        Ide::Cursor,
        Ide::Vscode,
        Ide::Windsurf,
        Ide::Claude,
        Ide::Sublime,
        Ide::Neovim,
        Ide::Zed,
        Ide::Idea,
        Ide::PyCharm,
        Ide::WebStorm,
        Ide::CLion,
        Ide::RustRover,
        Ide::GoLand,
        Ide::Fleet,
        Ide::Rider,
        Ide::AndroidStudio,
        Ide::VisualStudio,
        Ide::Helix,
        Ide::Terminal,
    ];
    for ide in ides {
        assert!(!ide.display_name().is_empty());
        assert!(!ide.icon().is_empty());
        let _ = ide.color();
        let _ = ide.next();
    }
}

#[test]
fn test_ide_next_cycle() {
    assert_eq!(Ide::Antigravity.next(), Ide::AntigravityCli);
    assert_eq!(Ide::AntigravityCli.next(), Ide::Cursor);
    assert_eq!(Ide::Cursor.next(), Ide::Vscode);
    assert_eq!(Ide::Vscode.next(), Ide::Windsurf);
    assert_eq!(Ide::Windsurf.next(), Ide::Claude);
    assert_eq!(Ide::Claude.next(), Ide::Sublime);
    assert_eq!(Ide::Sublime.next(), Ide::Neovim);
    assert_eq!(Ide::Neovim.next(), Ide::Zed);
    assert_eq!(Ide::Zed.next(), Ide::Idea);
    assert_eq!(Ide::Idea.next(), Ide::PyCharm);
    assert_eq!(Ide::PyCharm.next(), Ide::WebStorm);
    assert_eq!(Ide::WebStorm.next(), Ide::CLion);
    assert_eq!(Ide::CLion.next(), Ide::RustRover);
    assert_eq!(Ide::RustRover.next(), Ide::GoLand);
    assert_eq!(Ide::GoLand.next(), Ide::Fleet);
    assert_eq!(Ide::Fleet.next(), Ide::Rider);
    assert_eq!(Ide::Rider.next(), Ide::AndroidStudio);
    assert_eq!(Ide::AndroidStudio.next(), Ide::VisualStudio);
    assert_eq!(Ide::VisualStudio.next(), Ide::Helix);
    assert_eq!(Ide::Helix.next(), Ide::Terminal);
    assert_eq!(Ide::Terminal.next(), Ide::Antigravity);
}
