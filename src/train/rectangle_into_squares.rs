// fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
//     if lng == wdth { return None; }
//
//     let large = cmp::max(lng, wdth);
//     let small = cmp::min(lng, wdth);
//
//     match sq_in_rect(small, large - small) {
//         None => Some(vec![small, small]),
//         Some(x) => Some([vec![small], x].concat())
//     }
// }

pub fn sq_in_rect(length: i32, width: i32) -> Option<Vec<i32>> {
    if length == width {
        return None;
    }

    let mut v: Vec<i32> = Vec::new();

    let mut l: i32 = length;
    let mut w: i32 = width;

    while l > 0 || w > 0 {
        if l > w {
            l -= w;
            v.push(w)
        }
        if w > l {
            w -= l;
            v.push(l);
        }
        if l == w {
            v.push(l);
            return Some(v);
        }
    }

    None
}

#[test]
fn tests_sq_in_rect() {
    fn testing(length: i32, width: i32, exp: Option<Vec<i32>>) -> () {
        assert_eq!(sq_in_rect(length, width), exp)
    }

    testing(5, 3, Some(vec![3, 2, 1, 1]));
    testing(3, 5, Some(vec![3, 2, 1, 1]));
    testing(5, 5, None);
}
