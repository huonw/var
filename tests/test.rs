#[macro_use] extern crate var;

#[test]
fn zero() {
    var! {};
}

#[test]
fn one() {
    var! {
        a = 1,
    }

    a += 1;
    assert_eq!(a, 2);

    var! {
        b: String = "foo".to_string()
    }
    b.push_str("bar");

    assert_eq!(b, "foobar");
}

#[test]
fn two() {
    var! {
        a: i64 = 1,
        b = 1.0
    }

    a += 1;
    b *= 2.0;
    assert_eq!(a, 2);
    assert_eq!(b, 2.0);
}
