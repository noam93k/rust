fn a() {
    let x = [1];
    match x {
        [_, _, _, _, _, .._] => ::core::util::unreachable(),
        [.._, _, _, _, _] => ::core::util::unreachable(),
        [_, .._, _, _] => ::core::util::unreachable(),
        [_, _] => ::core::util::unreachable(),
        [a] => {
            assert_eq!(a, 1);
        }
        [] => ::core::util::unreachable()
    }
}

fn b() {
    let x = [1, 2, 3];
    match x {
        [a, b, ..c] => {
            assert_eq!(a, 1);
            assert_eq!(b, 2);
            assert_eq!(c, &[3]);
        }
        _ => fail!()
    }
    match x {
        [..a, b, c] => {
            assert_eq!(a, &[1]);
            assert_eq!(b, 2);
            assert_eq!(c, 3);
        }
        _ => fail!()
    }
    match x {
        [a, ..b, c] => {
            assert_eq!(a, 1);
            assert_eq!(b, &[2]);
            assert_eq!(c, 3);
        }
        _ => fail!()
    }
    match x {
        [a, b, c] => {
            assert_eq!(a, 1);
            assert_eq!(b, 2);
            assert_eq!(c, 3);
        }
        _ => fail!()
    }
}

pub fn main() {
    a();
    b();
}
