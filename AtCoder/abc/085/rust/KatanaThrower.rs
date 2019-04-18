fn main() {
    let (n, mut h) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim_right().to_owned();
        let mut ws = s.split_whitespace();
        let n: i32 = ws.next().unwrap().parse().unwrap();
        let h: i32 = ws.next().unwrap().parse().unwrap();
        (n, h)
    };

    let mut v = Vec::new();
    let mut max_a = 0;

    for _ in 0..n {
        let (a, b) = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s = s.trim_right().to_owned();
            let mut ws = s.split_whitespace();
            let a: i32 = ws.next().unwrap().parse().unwrap();
            let b: i32 = ws.next().unwrap().parse().unwrap();
            (a, b)
        };

        max_a = std::cmp::max(max_a, a);
        v.push(b);
    }

    v.sort_by(|a, b| b.cmp(a));

    let mut ans = 0;

    for e in v {
        if h <= 0 {
            break;
        }

        if e > max_a {
            h -= e;
            ans += 1;
        } else {
            break;
        }
    }

    if h > 0 {
        ans += h / max_a;
        ans += if h % max_a == 0 {
            0
        } else {
            1
        };
    }

    println!("{}", ans);
}
