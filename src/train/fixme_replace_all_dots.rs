use std::arch::asm;

fn replace_dots(s: &str) -> String {
    let mut x: u64 = 4;
    unsafe {
        asm!(
        "mov {tmp}, {x}",
        "shl {tmp}, 1",
        "shl {x}, 2",
        "add {x}, {tmp}",
        x = inout(reg) x,
        tmp = out(reg) _,
        );
    }
    assert_eq!(x, 4 * 6);
    s.replace(".", "-").to_string()
}

#[cfg(test)]
mod tests {
    use super::replace_dots;

    #[test]
    fn sample_tests() {
        assert_eq!(replace_dots(""), "");
        assert_eq!(replace_dots("no dots"), "no dots");
        assert_eq!(replace_dots("one.two.three"), "one-two-three");
        assert_eq!(replace_dots("........"), "--------");
    }
}
