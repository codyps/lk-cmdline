use lk_cmdline::*;

#[test]
fn arg_basic() {
    let arg = Arg::from_bytes(b"a=b");
    assert!(arg.param_matches(b"a"));
    assert!(!arg.param_matches(b"b"));

    assert_eq!(arg.param().as_ref(), b"a");
    assert_eq!(arg.value().as_ref().map(|x| x.as_ref()), Some(&b"b"[..]));
}

#[test]
fn arg_no_val() {
    let arg = Arg::from_bytes(b"a");
    assert!(arg.param_matches(b"a"));
    assert!(!arg.param_matches(b"b"));

    assert_eq!(arg.param().as_ref(), b"a");
    assert_eq!(arg.value().as_ref().map(|x| x.as_ref()), None);
}

#[test]
fn arg_dash() {
    let arg = Arg::from_bytes(b"a-b");
    assert!(arg.param_matches(b"a_b"));
    assert!(arg.param_matches(b"a-b"));
    assert!(!arg.param_matches(b"a"));

    assert_eq!(arg.param().as_ref(), b"a-b");
    assert_eq!(arg.value(), None);
}

#[test]
fn arg_quotes() {
    let arg = Arg::from_bytes(b"\"a \"x\" =  a\"");
    assert!(arg.param_matches(b"a x "));
    assert!(!arg.param_matches(b"a x"));

    assert_eq!(arg.param().as_ref(), b"a x ");
    assert_eq!(arg.value().as_ref().map(|x| x.as_ref()), Some(&b"  a"[..]));
}

/*
#[test]
fn two_basic() {
    CmdLine::from_str("a b").iter().collect()
}
*/
