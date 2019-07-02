macro_rules! get {
      ($t:ty) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              line.trim().parse::<$t>().unwrap()
          }
      };
      ($($t:ty),*) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              let mut iter = line.split_whitespace();
              (
                  $(iter.next().unwrap().parse::<$t>().unwrap(),)*
              )
          }
      };
      ($t:ty; $n:expr) => {
          (0..$n).map(|_|
              get!($t)
          ).collect::<Vec<_>>()
      };
      ($($t:ty),*; $n:expr) => {
          (0..$n).map(|_|
              get!($($t),*)
          ).collect::<Vec<_>>()
      };
      ($t:ty ;;) => {
          {
              let mut line: String = String::new();
              std::io::stdin().read_line(&mut line).unwrap();
              line.split_whitespace()
                  .map(|t| t.parse::<$t>().unwrap())
                  .collect::<Vec<_>>()
          }
      };
      ($t:ty ;; $n:expr) => {
          (0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()
      };
}

fn main() {
    let mut s = get!(String);

    if s.chars().last().unwrap() == 'a' {
        s.pop();
        if s.is_empty() {
            println!("-1");
        } else {
            for (i, c) in s.chars().into_iter().enumerate() {
                if i < s.len() {
                    print!("{}", c);
                    continue;
                }
                println!();
            }
        }
        return;
    }

    for (i, c) in s.chars().into_iter().enumerate() {
        if i < s.len() - 1 {
            print!("{}", c);
        } else {
            println!("{}", ((c as u8) - 1) as char);
        }
    }
}
