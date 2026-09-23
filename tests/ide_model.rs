use dev_cli::models::ide::Ide;

#[test]
fn test_ide_display_name_and_icon() {
    let ides =
        [Ide::Cursor, Ide::Vscode, Ide::Claude, Ide::Terminal, Ide::Idea, Ide::Rider, Ide::Zed];
    for ide in ides {
        assert!(!ide.display_name().is_empty());
        assert!(!ide.icon().is_empty());
        let _ = ide.color();
        let _ = ide.next();
    }
}

#[test]
fn test_ide_next_cycle() {
    assert_eq!(Ide::Cursor.next(), Ide::Vscode);
    assert_eq!(Ide::Vscode.next(), Ide::Claude);
    assert_eq!(Ide::Claude.next(), Ide::Terminal);
    assert_eq!(Ide::Terminal.next(), Ide::Idea);
    assert_eq!(Ide::Idea.next(), Ide::Rider);
    assert_eq!(Ide::Rider.next(), Ide::Zed);
    assert_eq!(Ide::Zed.next(), Ide::Cursor);
}
