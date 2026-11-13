//! `LOG_FORMAT=json`: one JSON object per line, for log collectors (Loki, Elasticsearch...).
//!
//! `{"time":"2026-11-13T06:46:39.316714Z","level":"info","target":"hbbs","file":"main.rs","line":42,"msg":"..."}`
use flexi_logger::{DeferredNow, FormatFunction};
use hbb_common::log::Record;

/// `text` unless `LOG_FORMAT` is `json`.
pub fn format_or(text: FormatFunction) -> FormatFunction {
    match std::env::var("LOG_FORMAT") {
        Ok(v) if v.eq_ignore_ascii_case("json") => format,
        _ => text,
    }
}

pub fn format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let file = record.file().unwrap_or("unknown");
    let line = serde_json::json!({
        "time": now
            .now()
            .with_timezone(&chrono::Utc)
            .to_rfc3339_opts(chrono::SecondsFormat::Micros, true),
        "level": record.level().as_str().to_lowercase(),
        "target": record.target(),
        "file": file.rsplit('/').next().unwrap_or(file),
        "line": record.line(),
        "msg": record.args().to_string(),
    });
    write!(w, "{line}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn one_json_object_per_record() {
        let mut out = Vec::new();
        super::format(
            &mut out,
            &mut flexi_logger::DeferredNow::new(),
            &hbb_common::log::Record::builder()
                .args(format_args!("a \"quoted\"\nline"))
                .level(hbb_common::log::Level::Warn)
                .target("rocket::server")
                .file(Some("/src/server.rs"))
                .line(Some(132))
                .build(),
        )
        .unwrap();
        let out = String::from_utf8(out).unwrap();
        assert!(!out.contains('\n'), "{out}");
        let v: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(v["level"], "warn");
        assert_eq!(v["target"], "rocket::server");
        assert_eq!(v["file"], "server.rs");
        assert_eq!(v["line"], 132);
        assert_eq!(v["msg"], "a \"quoted\"\nline");
        assert!(v["time"].as_str().unwrap().ends_with('Z'), "{v}");
    }
}
