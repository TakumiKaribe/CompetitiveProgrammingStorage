use std::collections::HashMap;

fn main() {
    let n = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim_right().to_owned();
        let n: i32 = s.parse().unwrap();
        n
    };

    let mut map = HashMap::new();

    let mut ans = 0;

    for _ in 0..n {
        let a = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s = s.trim_right().to_owned();
            let a: i32 = s.parse().unwrap();
            a
        };

        if map.contains_key(&a) {
            ans += 1;
        } else {
            map.insert(a, true);
        }

    }

    println!("{}", ans);
}
