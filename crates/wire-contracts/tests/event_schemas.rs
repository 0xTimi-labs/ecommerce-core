use std::fs;
use std::path::Path;

#[test]
fn test_cloudevents_json_schemas_and_fixtures() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let events_dir = manifest_dir.join("../../contracts/events");
    let fixtures_dir = events_dir.join("fixtures");

    // 1. 读取基础 envelope schema
    let envelope_content = fs::read_to_string(events_dir.join("event-envelope.v1.schema.json"))?;
    let envelope_json: serde_json::Value = serde_json::from_str(&envelope_content)?;

    // 2. 遍历并验证所有事件 Schema 及其 Fixtures
    let fixture_entries = fs::read_dir(&fixtures_dir)?;
    let mut verified_count = 0;

    for entry in fixture_entries {
        let entry = entry?;
        let path = entry.path();
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("无法获取文件名")?;

        if !file_name.ends_with(".json") {
            continue;
        }

        let is_valid_fixture = file_name.ends_with(".valid.json");
        let is_invalid_fixture = file_name.ends_with(".invalid.json");

        if !is_valid_fixture && !is_invalid_fixture {
            return Err(format!("Fixture 文件名不符合约定: {file_name}").into());
        }

        let schema_stem = if is_valid_fixture {
            file_name.trim_end_matches(".valid.json")
        } else {
            file_name.trim_end_matches(".invalid.json")
        };

        let schema_path = events_dir.join(format!("{schema_stem}.schema.json"));
        let schema_content = fs::read_to_string(&schema_path)
            .map_err(|e| format!("未找到对应的 Schema 文件 [{schema_path:?}]: {e}"))?;
        let schema_json: serde_json::Value = serde_json::from_str(&schema_content)?;

        // 若不是基础 envelope，将基础 envelope 的属性与特定事件 schema 合并为完整单体验证 schema
        let resolved_schema = if schema_stem == "event-envelope.v1" {
            envelope_json.clone()
        } else {
            let mut resolved = envelope_json.clone();
            if let (Some(resolved_props), Some(schema_props)) = (
                resolved
                    .get_mut("properties")
                    .and_then(|p| p.as_object_mut()),
                schema_json.get("properties").and_then(|p| p.as_object()),
            ) {
                for (k, v) in schema_props {
                    resolved_props.insert(k.clone(), v.clone());
                }
            }
            resolved
        };

        let validator = jsonschema::validator_for(&resolved_schema)
            .map_err(|e| format!("Schema [{file_name}] 语法编译失败: {e}"))?;

        let fixture_content = fs::read_to_string(&path)?;
        let fixture_json: serde_json::Value = serde_json::from_str(&fixture_content)?;

        let is_valid = validator.is_valid(&fixture_json);

        if is_valid_fixture {
            assert!(
                is_valid,
                "Fixture [{file_name}] 预期校验通过 (valid)，但实际校验失败"
            );
        } else {
            assert!(
                !is_valid,
                "Fixture [{file_name}] 预期校验失败 (invalid)，但实际校验通过"
            );
        }

        verified_count += 1;
    }

    assert!(
        verified_count >= 16,
        "预期至少校验 16 个 Fixture，实际校验了: {verified_count}"
    );

    Ok(())
}
