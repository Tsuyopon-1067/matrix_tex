extern crate rand;

use rand::Rng;
use std::io::{stdout, stdin, Write, BufRead};
use std::fs::File;

fn main() {
    let selected_problem : usize = input_text("0:足し引き算 1:掛け算".to_string()).parse().unwrap();
    let number_of_problem : usize = input_text("問題数".to_string()).parse().unwrap();

    let m : i32 = input_text("m".to_string()).parse().unwrap();
    let n : i32 = input_text("n".to_string()).parse().unwrap();

    // 問題・解答を生成してvecに入れる
    let mut v: Vec<(String, String)> = Vec::new();
    for _i in 0..number_of_problem {
        let mut tmp: (String, String) = ("".to_string(), "".to_string());
        match selected_problem {
            0 => {
                let mut flag: bool = true;
                let rnd = get_rand(0, 2);
                if rnd == 1 {
                    flag = false;
                }
                tmp = kagen_item(m, n, flag);
            },
            1 => tmp = kakezan_item(m, n),
            _ => {},
        }
        v.push(tmp);
    }

    // 問題・解答を文字列化
    let mut ques: String = String::new();
    let mut ans: String = String::new();
    for i in 0..number_of_problem {
        ques += &format!("{}\n", v[i].0);
        ans += &format!("{}\n", v[i].1);
    }

    // texファイルに書き込み
    write_tex("./q1.tex".to_string(), ques, ans);
}

// 加減問題1つを生成
// flag: tureなら足し算
fn kagen_item(m: i32, n: i32, flag: bool) -> (String, String) {
    let v1 = create_matrix_vec(m, n);
    let v2 = create_matrix_vec(m, n);
    let mut v3: Vec<Vec<i32>> = Vec::new();
    for i in 0 .. m {
        v3.push(Vec::new());
        for j in 0 .. n {
            if flag {
                v3[i as usize].push(v1[i as usize][j as usize]+v2[i as usize][j as usize]);
            } else {
                v3[i as usize].push(v1[i as usize][j as usize]-v2[i as usize][j as usize]);
            }
        }
    }

    let mut v2_text;
    if flag {
        v2_text = String::from("+");
    } else {
        v2_text = String::from("-");
    }
    let v1_text = matrix_to_string(v1.clone(), m, n);
    v2_text += &matrix_to_string(v2.clone(), m, n);
    let v3_text = matrix_to_string(v3.clone(), m, n);
    let res1: String = format!("\\item $\\begin{{aligned}}\n{}{}\\end{{aligned}}$", v1_text, v2_text);
    let res2: String = format!("\\item $\\begin{{aligned}}\n{}\\end{{aligned}}$", v3_text);
    (res1, res2)
}

// 掛け算問題1つを生成
fn kakezan_item(m: i32, n: i32) -> (String, String) {
    let v1 = create_matrix_vec(m, n);
    let v2 = create_matrix_vec(m, n);
    let mut v3: Vec<Vec<i32>> = Vec::new();
    for i in 0 .. m {
        v3.push(Vec::new());
        for j in 0 .. n {
            let mut tmp: i32 = 0;
            for k in 0 .. n {
                tmp += v1[i as usize][k as usize] * v2[k as usize][j as usize];
            }
            v3[i as usize].push(tmp);
        }
    }

    let mut v2_text = String::from("\\times");
    let v1_text = matrix_to_string(v1.clone(), m, n);
    v2_text += &matrix_to_string(v2.clone(), m, n);
    let v3_text = matrix_to_string(v3.clone(), m, n);
    let res1: String = format!("\\item $\\begin{{aligned}}\n{}{}\\end{{aligned}}$", v1_text, v2_text);
    let res2: String = format!("\\item $\\begin{{aligned}}\n{}\\end{{aligned}}$", v3_text);
    (res1, res2)
}

fn create_matrix_vec(m: i32, n: i32) -> Vec<Vec<i32>> {
    let mut v: Vec<Vec<i32>> = Vec::new();
    for i in 0 .. m {
        v.push(Vec::new());
        for _j in 0 .. n {
            v[i as usize].push(get_rand(-9, 9));
        }
    }
    return v;
}

fn matrix_to_string(v: Vec<Vec<i32>>, m: i32, n: i32) -> String {
    let mut res: String = String::new();
    res += "\\begin{bmatrix}\n";
    for i in 0 .. m {
        res += "\t";
        for j in 0 .. n {
            res += &v[i as usize][j as usize].to_string();
            if j != n-1 {
                res += " & ";
            }
        }
        res += "\\\\\n";
    }
    res += "\\end{bmatrix}\n";
    return res;
}

// a以上b未満の乱数を取得
fn get_rand(a: i32, b :i32) -> i32 {
    let res: i32 = rand::thread_rng().gen_range(a, b);
    res
}

// texファイルに書き込み
fn write_tex(path: String, ques: String, ans: String) {
       let mut file = File::create(path)
           .expect("file not found.");
        writeln!(file, "\\subsection*{{問題}}").expect("cannot write.");
        writeln!(file, "\\begin{{enumerate}}[(1)]").expect("cannot write.");
        write!(file, "{}", ques).expect("cannot write.");
        writeln!(file, "\\end{{enumerate}}").expect("cannot write.");

        writeln!(file, "\\newpage").expect("cannot write.");

        writeln!(file, "\\subsection*{{解答}}").expect("cannot write.");
        writeln!(file, "\\begin{{enumerate}}[(1)]").expect("cannot write.");
        write!(file, "{}", ans).expect("cannot write.");
        writeln!(file, "\\end{{enumerate}}").expect("cannot write.");
}

// s:って出して入力文字列を返す
fn input_text(s: String) -> String {
    print!("{}: ", s);
    stdout().flush().unwrap();
    let stdin = stdin();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer).unwrap();  // 標準入力から行を読み取る
    return buffer.trim().to_string();
}