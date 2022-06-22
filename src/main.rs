use std::io;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use regex::Regex;

enum OperationType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

impl OperationType{

    fn reckon(&self, x: &BigDecimal, y: &BigDecimal) -> BigDecimal {
        match self {
            OperationType::Power => BigDecimal::from_str("1").unwrap(),
            OperationType::Multiply => x*y,
            OperationType::Divide => x/y,
            OperationType::Add => x+y,
            OperationType::Subtract => x-y
        }

    }
}

fn main() {

    println!("\n         {}   {}   {}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));

    let mut cmd = String::new();
    loop {
        print!("   &>   ");
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin().read_line(&mut cmd).unwrap();
        println!("    {}", calcute_expression(&cmd[0..cmd.len()-2]));
        cmd.clear();
    }

}

fn calcute_expression(expression: &str) -> BigDecimal {

    let re = Regex::new(r"^(\s*)\d+(\.\d+)*((\s*)(?:[+\*-/^](\s*)\d+(\.\d+)*))+$").unwrap();


    if !re.is_match(expression) {
        println!("Error, not right syntax");
    };

    let re_number = Regex::new(r"\d+(\.\d+)*").unwrap();
    let re_operator = Regex::new(r"[+\*-/^]").unwrap();

    let mut number_sequence: Vec<BigDecimal> = Vec::new();
    let mut operation_sequence: Vec<OperationType> = Vec::new();


    for mat in re_number.find_iter(expression) {
        number_sequence.push(BigDecimal::from_str(&expression[mat.start()..mat.end()]).unwrap());
    }

    for mat in re_operator.find_iter(expression) {
        operation_sequence.push(match expression.chars().nth(mat.start()).unwrap() {
            '+' => OperationType::Add,
            '-' => OperationType::Subtract,
            '*' => OperationType::Multiply,
            '/' => OperationType::Divide,
            '^' => OperationType::Power,
            _ => todo!()
        });
    }

    let mut result: BigDecimal;

    let mut i: u8;
    let mut current_op_pos: u8;

    loop {

        i = 255;
        current_op_pos = 0;

        for (pos, op) in operation_sequence.iter().enumerate() {
            match op {
                OperationType::Power => {
                    current_op_pos = pos as u8;
                    i=0;
                    continue;
                },
                OperationType::Multiply | OperationType::Divide => {
                    if i>0 {
                        current_op_pos = pos as u8;
                        i=1;
                    }
                    continue;
                },
                OperationType::Add | OperationType::Subtract => {
                    if i>1 {
                        current_op_pos = pos as u8;
                        i=2;
                    }
                    continue;
                }
            }
        }

        result = operation_sequence[current_op_pos as usize].reckon(&number_sequence[current_op_pos as usize], &number_sequence[(current_op_pos+1) as usize]);
        number_sequence.remove((current_op_pos+1) as usize);
        number_sequence[current_op_pos as usize] = result;
        operation_sequence.remove(current_op_pos as usize);


        if operation_sequence.len() == 0 {
            break number_sequence[0].clone();
        }

    }
}
