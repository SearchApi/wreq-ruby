use std::collections::BTreeMap;

use ::serde::{Deserialize, Serialize};
use magnus::{Value, value::ReprValue};

use super::{deserialize_json, deserialize_ruby, serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Record {
    count: u64,
    enabled: bool,
    tags: Vec<String>,
    note: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
enum State {
    Ready,
    Progress(u64, bool),
    Failed { message: String },
}

#[test]
fn retains_ruby_serde_conversion_surface() -> Result<(), magnus::Error> {
    // SAFETY: this is the only test that initializes the embedded Ruby VM.
    let ruby = unsafe { magnus::embed::init() };

    let record = Record {
        count: 42,
        enabled: true,
        tags: vec!["ruby".into(), "rust".into()],
        note: None,
    };
    let value: Value = serialize(&ruby, &record)?;
    let output: Record = deserialize_ruby(&ruby, value)?;
    assert_eq!(record, output);

    let map = BTreeMap::from([("first".to_owned(), 1_u64), ("second".to_owned(), 2)]);
    let value: Value = serialize(&ruby, &map)?;
    let output: BTreeMap<String, u64> = deserialize_ruby(&ruby, value)?;
    assert_eq!(map, output);

    for state in [
        State::Ready,
        State::Progress(3, false),
        State::Failed {
            message: "failed".into(),
        },
    ] {
        let value: Value = serialize(&ruby, &state)?;
        let output: State = deserialize_ruby(&ruby, value)?;
        assert_eq!(state, output);
    }

    let value: Value = serialize(&ruby, &i128::MIN)?;
    let decimal: String = value.funcall("to_s", ())?;
    assert_eq!(i128::MIN.to_string(), decimal);
    assert_eq!(i128::MIN, deserialize_ruby::<_, i128>(&ruby, value)?);
    assert_eq!(i128::MIN, deserialize_json::<_, i128>(&ruby, value)?);

    let value: Value = serialize(&ruby, &u128::MAX)?;
    let decimal: String = value.funcall("to_s", ())?;
    assert_eq!(u128::MAX.to_string(), decimal);
    assert_eq!(u128::MAX, deserialize_ruby::<_, u128>(&ruby, value)?);
    assert_eq!(u128::MAX, deserialize_json::<_, u128>(&ruby, value)?);

    let value: Value = serialize(&ruby, &u64::MAX)?;
    assert_eq!(u64::MAX, deserialize_ruby::<_, u64>(&ruby, value)?);
    assert_eq!(u64::MAX, deserialize_json::<_, u64>(&ruby, value)?);

    Ok(())
}
