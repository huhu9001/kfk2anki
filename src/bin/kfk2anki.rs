use shogi::board::Board;
use shogi::board::Move;
use shogi::board::Pos;

use std::io::Write;

const PIECE_NAMES: [&str; 16] = [
    "KI", "GI", "KE", "KY", "FU", "HI", "KA", "OU",
    "KI", "NG", "NK", "NY", "TO", "RY", "UM", "OU",
];

fn main() {
    let mut inputs = Vec::<String>::new();
    let mut output = String::from(".");
    let mut we = std::collections::BTreeSet::<String>::new();
    let mut delta = 0.1;
    let mut arg_type = 0;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-o" | "--output" => arg_type = 1,
            "-n" | "--name" => arg_type = 2,
            "-d" | "--delta" => arg_type = 3,
            _ => {
                match arg_type {
                    1 => output = arg,
                    2 => drop(we.insert(arg)),
                    3 => delta =
                        arg.parse().expect(&format!(r#"Invalid delta "{arg}""#)),
                    _ => inputs.push(arg),
                }

                arg_type = 0;
            }
        }
    }

    if inputs.is_empty() {
        //inputs.push(String::from("C:/Users/admin/Documents/Kifu/piyo_20251115_194055.kfk"));
        //output = String::from("E:/Print/draft/shogi");
        println!("input file:");
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        inputs.push(s);
    }

    let mut deck = std::fs::File::create(format!("{output}/deck.csv")).unwrap();
    'next_file:for finput in inputs {
        println!(r#"Working on file "{finput}"..."#);
        macro_rules! invalid_file {
            ($reason:literal) => {{
                eprintln!("Invalid kifu file \"{finput}\": {}", $reason);
                continue;
            }}
        }
        let Ok(kfk) = std::fs::read_to_string(&finput) else {invalid_file!("cannot read file")};

        let Some(kifu_start) = kfk.find("<kifu>") else {invalid_file!("<kifu> tag not found")};
        let Some(kifu_end) = kfk.find("</kifu>") else {invalid_file!("<kifu> tag not closed")};

        let mut b = Board::from(Board::NEW_STANDARD);
        let mut mvs = Vec::<Move>::new();
        let mut name_black = "";
        let mut name_white = "";

        for (n, line) in kfk[kifu_start + 6 .. kifu_end].lines().enumerate() {
            let l = line.as_bytes();
            if l.len() < 7 {continue}
            match l[0] {
                b'+' | b'-' => {
                    macro_rules! invalid_move {
                        () => {{
                            eprintln!("Illegal move in file \"{finput}\" line {n}: {line}");
                            continue 'next_file;
                        }}
                    }
                    let mv = match l[1] {
                        b'0' => {
                            let Some(kind) = PIECE_NAMES.iter()
                                .position(|&it| *it.as_bytes() == l[5 ..= 6]) else {invalid_move!()};
                            Move{
                                from:Pos{
                                    x:if b.turn {10} else {9},
                                    y:kind as i8,
                                },
                                to:Pos{
                                    x:l[3] as i8 - b'1' as i8,
                                    y:l[4] as i8 - b'1' as i8,
                                },
                                prmt:false,
                            }
                        }
                        b'1' ..= b'9' => {
                            let fx = l[1] as i8 - b'1' as i8;
                            let fy = l[2] as i8 - b'1' as i8;
                            let kind = match b.terra[fy as usize][fx as usize] {
                                0 => invalid_move!(),
                                v => (v - 1 & 0xF) as usize,
                            };
                            Move{
                                from:Pos{x:fx, y:fy},
                                to:Pos{
                                    x:l[3] as i8 - b'1' as i8,
                                    y:l[4] as i8 - b'1' as i8,
                                },
                                prmt:*PIECE_NAMES[kind].as_bytes() != l[5 ..= 6],
                            }
                        }
                        _ => invalid_move!(),
                    };
                    if b.legal(mv) {
                        mvs.push(mv);
                        b.do_move(mv);
                    }
                    else {invalid_move!()}
                }
                b'N' => {
                    match l[1] {
                        b'+' => name_black = &line[2..],
                        b'-' => name_white = &line[2..],
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        
        let player = match (we.contains(name_black), we.contains(name_white)) {
            (true, false) => false,
            (false, true) => false,
            _ => {
                println!("Who am I?");
                println!("  1. Black ${name_black}");
                println!("  2. White ${name_white}");
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).unwrap();
                match s.trim().parse::<i32>() {
                    Ok(1) => {
                        we.remove(name_white);
                        we.insert(String::from(name_black));
                        false
                    }
                    Ok(2) => {
                        we.remove(name_black);
                        we.insert(String::from(name_white));
                        true
                    }
                    _ => continue,
                }
            }
        };

        let mut scores = Vec::<f64>::new();
        let mut pvs = Vec::<&str>::new();
        let mut ana = kfk.as_str();

        loop {
            let Some(score_start) = ana.find("<score>") else {break};
            ana = &ana[score_start + 7..];
            let Some(score_end) = ana.find("</score>") else {break};
            let sc_str = ana[0 .. score_end].trim();
            scores.push(match sc_str.parse::<i32>() {
                Ok(isc) =>
                    1.0 / (1.0 + (if player {isc} else {-isc} as f64 * 0.003).exp()),
                Err(_) =>
                    match sc_str.as_bytes().get(0) {
                        Some(b'+') => if player {0.0} else {1.0},
                        Some(b'-') => if player {1.0} else {0.0},
                        _ => 0.5,
                    },
            });
            ana = &ana[score_end + 8..];

            let Some(pv_start) = ana.find("<pv>") else {break};
            ana = &ana[pv_start + 4..];
            let Some(pv_end) = ana.find("</pv>") else {break};
            pvs.push(&ana[0 .. pv_end]);
            ana = &ana[pv_end + 5..];
        }

        if pvs.len() != scores.len() {
            invalid_file!("mismatched <score> and <pv> tags");
        }
        if mvs.len() + 2 != scores.len() {
            invalid_file!("mismatched number of moves and scores");
        }
        
        b = Board::from(Board::NEW_STANDARD);
        for (n, &mv) in mvs.iter().enumerate() {
            if scores[n + 2] - scores[n + 1] < -delta {
                let mut b_invert = std::mem::MaybeUninit::<Board>::uninit();
                let b = if b.turn {
                    let b = b_invert.write(b.clone());
                    b.invert();
                    b
                } else {&b};

                let mut pv = String::from(pvs[n + 1]);
                if n > 0 {
                    if let Some((i, ch)) = pv.char_indices().nth(1) {
                        if ch == '同' {
                            let to = mvs[n - 1].to;
                            let rep = format!("{}{}", to.x + 1, to.y + 1);
                            pv.replace_range(i .. i + '同'.len_utf8(), &rep);
                        }
                    }
                }

                if player {
                    let mut pv_new = String::new();
                    for ch in pv.chars() {
                        pv_new.push(match ch {
                            '1' ..= '9' => (b'1' + b'9' - ch as u8) as char,
                            '１' ..= '９' =>
                                char::from_u32(8 - (ch as u32 - '１' as u32) + '１' as u32).unwrap(),
                            '一' => '九',
                            '二' => '八',
                            '三' => '七',
                            '四' => '六',
                            '五' => '五',
                            '六' => '四',
                            '七' => '三',
                            '八' => '二',
                            '九' => '一',
                            _ => ch,
                        });
                    }
                    pv = pv_new;
                }

                writeln!(deck, "{}{}\t{pv}", "<style>.nightMode svg{filter:invert(1)}</style>", shogi::board::io::SVG(b)).unwrap();
            }

            b.do_move(mv);
        }
    }
}