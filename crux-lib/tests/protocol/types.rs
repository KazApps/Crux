use crux_lib::protocol::types::{EngineOption, Overtime, SearchLimits, TimeControl};

#[test]
fn bool_option_set_from_str() {
    let mut opt = EngineOption::bool(false);

    opt.set_from_str("true").unwrap();
    match opt {
        EngineOption::Bool { value, .. } => assert!(value),
        _ => panic!("wrong variant"),
    }

    opt.set_from_str("false").unwrap();
    match opt {
        EngineOption::Bool { value, .. } => assert!(!value),
        _ => panic!("wrong variant"),
    }
}

#[test]
fn bool_option_invalid_value() {
    let mut opt = EngineOption::bool(false);

    assert!(opt.set_from_str("not_bool").is_err());
}

#[test]
fn int_option_set_from_str() {
    let mut opt = EngineOption::int_range(1, 1, 1024);

    opt.set_from_str("16").unwrap();

    match opt {
        EngineOption::IntRange { value, .. } => assert_eq!(value, 16),
        _ => panic!("wrong variant"),
    }
}

#[test]
fn int_option_invalid_value() {
    let mut opt = EngineOption::int_range(1, 1, 1024);

    assert!(opt.set_from_str("abc").is_err());
}

#[test]
fn int_option_out_of_range() {
    let mut opt = EngineOption::int_range(1, 1, 1024);

    assert!(opt.set_from_str("2048").is_err());
}

#[test]
fn overtime_default_is_increment_zero() {
    match Overtime::default() {
        Overtime::Increment(d) => assert_eq!(d.as_millis(), 0),
        _ => panic!("default should be Increment"),
    }
}

#[test]
fn time_control_default() {
    let tc = TimeControl::default();

    assert_eq!(tc.black.base.as_millis(), 0);
    assert_eq!(tc.white.base.as_millis(), 0);
}

#[test]
fn search_limits_default() {
    let limits = SearchLimits::default();

    assert!(limits.time.is_none());
    assert!(limits.nodes.is_none());
    assert!(limits.depth.is_none());
    assert!(limits.moves.is_none());
}
