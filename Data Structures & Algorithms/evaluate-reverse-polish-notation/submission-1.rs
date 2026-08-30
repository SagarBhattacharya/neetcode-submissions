impl Solution {
  pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];
    for token in tokens {
      match token.as_str() {
        "+" => {
          let b = stack.pop().unwrap();
          *stack.last_mut().unwrap() += b;
        },
        "-" => {
          let b = stack.pop().unwrap();
          *stack.last_mut().unwrap() -= b;
        },
        "*" => {
          let b = stack.pop().unwrap();
          *stack.last_mut().unwrap() *= b;
        },
        "/" => {
          let b = stack.pop().unwrap();
          *stack.last_mut().unwrap() /= b;
        },
        num => {
          if let Ok(num) = num.parse::<i32>() {
            stack.push(num);
          }
        }
      }
    }

    stack[0]
  }
}