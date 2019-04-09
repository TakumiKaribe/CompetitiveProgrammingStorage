fn main() {
    let (n, s, t) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim_right().to_owned();
        let mut ws = s.split_whitespace();
        let n: i32 = ws.next().unwrap().parse().unwrap();
        let s: i32 = ws.next().unwrap().parse().unwrap();
        let t: i32 = ws.next().unwrap().parse().unwrap();
        (n, s, t)
    };

    let mut w = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim_right().to_owned();
        let w: i32 = s.parse().unwrap();
        w
    };

    let mut ans: i32 = if w >= s && w <= t {
        1
    } else {
        0
    };

    for _ in 1..n {
        let mut a = String::new();
        std::io::stdin().read_line(&mut a).unwrap();
        let a: i32 = a.trim_right().to_owned().parse().unwrap();
        w += a;
        ans += if w >= s && w <= t {
            1
        } else {
            0
        };
    }

    println!("{}", ans);
}
