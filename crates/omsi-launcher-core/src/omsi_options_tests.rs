include!("../../../tools/test-support/original_root.rs");

#[test]
#[ignore = "needs the original OMSI 2 install (OMSI_ROOT)"]
fn the_originals_options_are_read() {
    let root = &original_root();
    let o = super::omsi_options(root).expect("options.cfg of the original install");
    assert_eq!(
        o.last_map.as_deref(),
        Some("maps/Berlin-Spandau/global.cfg")
    );
    assert_eq!(o.last_driver.as_deref(), Some("OMSI-Fan"));
    assert_eq!(o.settings["max_fps"], 30);
    assert_eq!(o.settings["mirror_size"], 512);
    assert_eq!(o.settings["language"], "ENG");
    assert_eq!(o.settings["head_movement"], true);
    assert_eq!(o.settings["collision_vehicles"], false);
}

#[test]
fn the_real_time_reflections_are_read_as_omsi_writes_them() {
    let root = std::env::temp_dir().join(format!("omsi-realrefl-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    for (word, mode) in [("none", "off"), ("economy", "eco"), ("full", "full")] {
        std::fs::write(
            root.join("options.cfg"),
            format!(
                "[performance_realreflexions]\r\n{word}\r\n\r\n[performance_reflTexSize]\r\n9\r\n"
            ),
        )
        .unwrap();
        let o = super::omsi_options(&root).unwrap();
        assert_eq!(o.settings["mirror_refresh"], mode, "{word}");
    }
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn ai_pedestrian_awareness_survives_settings_saves() {
    use super::{settings_from_text_on, settings_to_text};
    use serde_json::json;

    let key = "ai_pedestrian_awareness";
    for phone in [false, true] {
        assert_eq!(settings_from_text_on(None, phone)[key], "full");
        let old = format!("{key}=full\n{key}=ignore\n");
        for mode in ["full", "crossings", "ignore"] {
            let mut settings = settings_from_text_on(Some(&old), phone);
            settings[key] = json!(mode);
            let text = settings_to_text(&settings, Some(&old));
            assert_eq!(
                text.lines()
                    .filter(|line| line.starts_with(&format!("{key}=")))
                    .count(),
                1
            );
            assert_eq!(settings_from_text_on(Some(&text), phone)[key], mode);
        }
        assert_eq!(
            settings_from_text_on(Some("ai_pedestrian_awareness=invalid\n"), phone)[key],
            "full"
        );
    }
}
