fn main() {
    loop {
        println!("input:");
        let mut s = String::new();
        loop {
            let n = std::io::stdin().read_line(&mut s).unwrap();
            let line = &s[s.len() - n .. s.len()];
            match line.trim() {
                "exit" => return,
                "" => break,
                _ => {}
            }
        }
        let Ok(mut b) = s.parse::<shogi::board::Board>() else {
            println!("Bad board.");
            continue
        };
        if b.turn {b.invert()}
        println!("{}\n", shogi::board::io::SVG(&b));
    }
}