fn main() {
    let s:String = "recede".to_string();
    let n1 = s.matches('e').count();
    let n2 = s.matches('r').count();
    let ccc = match n1 {
        1 => '(',
        _ => ')',
    };
    println!("{}, {}, {}", n1, n2, ccc);


   let res: String = s.chars().map(|c| {
       match s.matches(c).count() {
           1 => '(',
           _ => ')',
       }
   }
   ).collect();
    println!("{}", res);

}