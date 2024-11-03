use std::io::stdin;

fn main() {
    // let mut memory: f64 = 0.0;
    let mut memories: Vec<f64> = vec![0.0; 10];
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
        let is_memory = tokens[0].starts_with("mem");
        if is_memory && tokens[0].ends_with('+') {
            add_and_print_memory(&mut memories, tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with('-') {
            add_and_print_memory(&mut memories, tokens[0], -prev_result);
            continue;
        }

        // 式の計算
        let left = eval_token(tokens[0], &memories);
        let right = eval_token(tokens[2], &memories);
        let result = eval_expression(left, tokens[1], right);
        print_output(result);
        prev_result = result;
    }
}

fn add_and_print_memory(memories: &mut [f64], token: &str, prev_result: f64) {
    let slot_index: usize = token[3..token.len() - 1].parse().unwrap();
    memories[slot_index] += prev_result; // 自動的に参照外しが行われる
                                         // (*memories)[slot_index] += prev_result; と同じ
    print_output(memories[slot_index]);
}

fn print_output(value: f64) {
    println!(" => {}", value);
}

fn eval_token(token: &str, memories: &[f64]) -> f64 {
    if token.starts_with("mem") {
        let slot_index: usize = token[3..].parse().unwrap();
        memories[slot_index]
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
