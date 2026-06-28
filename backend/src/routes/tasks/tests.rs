use super::*;

#[test]
fn default_board_data_has_zero_version() {
    let d = BoardData::default();
    assert_eq!(d.version, 0);
    assert!(d.boards.is_empty());
}

#[test]
fn board_data_roundtrip_preserves_version() {
    let mut boards = indexmap::IndexMap::new();
    let mut columns = indexmap::IndexMap::new();
    columns.insert(
        "todo".to_string(),
        Column {
            name: "To Do".to_string(),
            tasks: vec!["a".into(), "b".into()],
        },
    );
    boards.insert(
        "work".to_string(),
        Board {
            name: "Work".to_string(),
            columns,
        },
    );
    let data = BoardData {
        version: 42,
        boards,
        active_board: "work".to_string(),
    };
    let s = serde_json::to_string(&data).unwrap();
    let parsed: BoardData = serde_json::from_str(&s).unwrap();
    assert_eq!(parsed.version, 42);
    assert_eq!(parsed.active_board, "work");
    assert_eq!(parsed.boards["work"].columns["todo"].tasks, vec!["a", "b"]);
}

#[test]
fn board_data_deserializes_legacy_without_version() {
    // Older files written before optimistic concurrency was added.
    let legacy = r#"{
        "boards": {
            "work": {
                "name": "Work",
                "columns": {
                    "todo": { "name": "To Do", "tasks": [] }
                }
            }
        },
        "activeBoard": "work"
    }"#;
    let parsed: BoardData = serde_json::from_str(legacy).unwrap();
    assert_eq!(parsed.version, 0, "legacy files default version to 0");
    assert_eq!(parsed.active_board, "work");
}
