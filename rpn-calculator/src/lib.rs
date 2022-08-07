#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn do_op(mut stack: Vec<i32>, f: fn(i32, i32) -> i32) -> Option<Vec<i32>> {
    let arg2 = stack.pop()?;
    let arg1 = stack.pop()?;
    stack.push(f(arg1, arg2));
    Some(stack)

}
fn operate(mut stack: Vec<i32>, op: &CalculatorInput) -> Option<Vec<i32>>{
    match *op {
       CalculatorInput::Add =>  do_op(stack, |a, b| a + b),
       CalculatorInput::Divide =>  do_op(stack, |a, b| a / b),
       CalculatorInput::Multiply => do_op(stack, |a, b| a * b),
       CalculatorInput::Subtract => do_op(stack, |a, b| a - b),
       CalculatorInput::Value(val) => { 
        stack.push(val);
        Some(stack)
       },
    }

}
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
   let result = inputs.iter().try_fold(vec![], |stack, op| operate(stack, op))?;
   if result.len() != 1 {
    return None
   }
   Some(result[0])
}
