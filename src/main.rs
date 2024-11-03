use std::io::stdin;

fn main() {
    let mut memory: f64 = 0.0;
    let mut prev_result: f64 = 0.0;
    for line in stdin().lines() {
        // 1行読み取って空行なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        // 空行で分割
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        // memoryへの書き込み
        if tokens[0] == "mem+" || tokens[0] == "mem-" {
            // &mut（可変参照）を付けないと、値を変更しても実引数のmemoryは変わらない
            // &mutを付けない場合、memoryの値が関数の仮引数にCopyされる動きとなる(f64はCopyトレイトを実装しているため)
            // 結果、両者は別のスタック領域に格納される
            add_and_print_memory(&mut memory, prev_result);
            continue;
        }

        // 式の計算
        let left = eval_token(tokens[0], memory);
        let right = eval_token(tokens[2], memory);
        let result = eval_expression(left, tokens[1], right);
        print_output(result);
        prev_result = result;
    }
}

fn add_and_print_memory(memory: &mut f64, prev_result: f64) {
    *memory += prev_result;
    print_output(*memory);
}

fn print_output(value: f64) {
    println!(" => {}", value);
}

fn eval_token(token: &str, memory: f64) -> f64 {
    if token == "mem" {
        memory
    } else {
        // 参照のtoken(&str)をもとに新しいf64の値を生成して返す
        token.parse().unwrap() // 型を指定しなくていいのは、memoryがf64なので型推論されている
    }
}

fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => {
            // 不明な演算子
            unreachable!();
        }
    }
}
