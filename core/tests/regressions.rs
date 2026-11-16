use diary_core::base::date::Date;
use diary_core::model::Day;
use diary_core::storage::DatabaseManager;
use diary_core::utils::io::import::{DuplicateStrategy, Importer};
use diary_core::utils::search::SearchCondition;
use regex::Regex;

fn day(date: &str, text: &str) -> Day {
    Day {
        date: date.parse().unwrap(),
        event: text.to_string().into(),
        weather: Some("晴".into()),
        mood: Some(7.5),
    }
}

fn database() -> DatabaseManager {
    DatabaseManager::from_path(std::path::Path::new(":memory:")).unwrap()
}

fn run(future: impl std::future::Future<Output = ()>) {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(future);
}

#[test]
fn calendar_dates_and_invalid_offsets() {
    assert_eq!(
        "20250101".parse::<Date>().unwrap(),
        Date::new(2025, 1, 1).unwrap()
    );
    assert_eq!(
        "2025-01-01".parse::<Date>().unwrap(),
        Date::new(2025, 1, 1).unwrap()
    );
    for input in [
        "20250230",
        "ye/abc",
        "错误日期",
        "9223372036854775807",
        "-9223372036854775808",
    ] {
        assert!(input.parse::<Date>().is_err(), "{input}");
    }
    assert_eq!(
        "m".parse::<Date>().unwrap(),
        "tomorrow".parse::<Date>().unwrap()
    );
}

#[test]
fn storage_roundtrip_and_null_event() {
    let mut db = database();
    run(async {
        let entry = day("2025-01-01", "测试");
        db.add_day(&entry).await.unwrap();
        let stored = db.read_day(entry.date).await.unwrap().unwrap();
        assert_eq!(stored.mood, Some(7.5));
        assert_eq!(stored.event.instruct, "测试");
        sqlx::query("INSERT INTO day (date) VALUES ('20250102')")
            .execute(&mut *db)
            .await
            .unwrap();
        assert_eq!(db.read_all().await.unwrap()[1].event.instruct, "");
    });
}

#[test]
fn malformed_database_returns_error() {
    let mut db = database();
    run(async {
        sqlx::query("INSERT INTO day (date, event) VALUES ('bad-date', 'text')")
            .execute(&mut *db)
            .await
            .unwrap();
        assert!(db.read_all().await.is_err());
        assert!(
            db.search_in_condition(SearchCondition::default())
                .await
                .is_err()
        );
    });
}

#[test]
fn search_filters_dates_regex_and_sorts() {
    let mut db = database();
    run(async {
        for entry in [
            day("2025-01-03", "日记三"),
            day("2025-01-01", "日记一"),
            day("2025-01-02", "日记二"),
        ] {
            db.add_day(&entry).await.unwrap();
        }
        let results = db
            .search_in_condition(SearchCondition::default())
            .await
            .unwrap();
        assert_eq!(
            results.iter().map(|d| d.date.day()).collect::<Vec<_>>(),
            [3, 2, 1]
        );
        let results = db
            .search_in_condition(
                SearchCondition::default()
                    .with_date_from("2025-01-02".parse().unwrap())
                    .with_date_to("2025-01-03".parse().unwrap())
                    .with_keyword("日记")
                    .with_weather_like("晴")
                    .with_mood_from(7)
                    .with_mood_to(8)
                    .with_regex(Regex::new("[一二]$").unwrap()),
            )
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].date.day(), 2);
    });
}

#[test]
fn imports_handle_duplicates_and_append_metadata() {
    let mut db = database();
    run(async {
        let original = day("2025-01-01", "原文");
        db.add_day(&original).await.unwrap();
        let mut incoming = day("2025-01-01", "追加");
        incoming.weather = None;
        incoming.mood = None;
        Importer::new(&mut db)
            .import_to_db(vec![incoming.clone()], DuplicateStrategy::Ignore)
            .await
            .unwrap();
        assert_eq!(
            db.read_day(original.date)
                .await
                .unwrap()
                .unwrap()
                .event
                .instruct,
            "原文"
        );
        Importer::new(&mut db)
            .import_to_db(vec![incoming.clone()], DuplicateStrategy::Append)
            .await
            .unwrap();
        let appended = db.read_day(original.date).await.unwrap().unwrap();
        assert_eq!(appended.event.instruct, "原文\n追加");
        assert_eq!(appended.mood, Some(7.5));
        assert_eq!(appended.weather.as_deref(), Some("晴"));
        Importer::new(&mut db)
            .import_to_db(vec![incoming], DuplicateStrategy::Replace)
            .await
            .unwrap();
        let replaced = db.read_day(original.date).await.unwrap().unwrap();
        assert_eq!(replaced.event.instruct, "追加");
        assert_eq!(replaced.mood, None);
    });
}

#[test]
fn all_import_modes_roll_back_on_failure() {
    for mode in [
        DuplicateStrategy::Replace,
        DuplicateStrategy::Ignore,
        DuplicateStrategy::Fail,
        DuplicateStrategy::Append,
    ] {
        let mut db = database();
        run(async {
            db.add_day(&day("2025-01-01", "原文")).await.unwrap();
            sqlx::query("CREATE TRIGGER reject_bad BEFORE INSERT ON day WHEN NEW.event = 'bad' BEGIN SELECT RAISE(ABORT, 'rejected'); END")
                .execute(&mut *db).await.unwrap();
            let result = Importer::new(&mut db)
                .import_to_db(
                    vec![day("2025-01-02", "ok"), day("2025-01-03", "bad")],
                    mode,
                )
                .await;
            assert!(result.is_err());
            let remaining = db.read_all().await.unwrap();
            assert_eq!(remaining.len(), 1);
            assert_eq!(remaining[0].event.instruct, "原文");
        });
    }
}

#[test]
fn duplicate_failure_rolls_back_earlier_rows() {
    let mut db = database();
    run(async {
        let original = day("2025-01-01", "原文");
        db.add_day(&original).await.unwrap();
        assert!(
            Importer::new(&mut db)
                .import_to_db(
                    vec![day("2025-01-02", "new"), original],
                    DuplicateStrategy::Fail
                )
                .await
                .is_err()
        );
        assert_eq!(db.read_all().await.unwrap().len(), 1);
    });
}
