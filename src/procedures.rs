use std::str::FromStr;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use bigdecimal::ToPrimitive;
use regex::Regex;


#[derive(Debug)]
enum Tag {
    Number(BigDecimal),
    Operator(OperationType)
}

#[derive(Debug)]
enum OperationType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}


impl OperationType {
    fn parse(operator: char) -> OperationType {
        match operator {
            '+' => OperationType::Add,
            '-' => OperationType::Subtract,
            '*' => OperationType::Multiply,
            '/' => OperationType::Divide,
            '^' => OperationType::Power,
            _ => todo!()
        }
    }


    fn count(&self, x: &BigDecimal, y: &BigDecimal) -> BigDecimal {
        match self {
            OperationType::Power => OperationType::pow(x.clone(), y.to_f64().unwrap()),
            OperationType::Multiply => x*y,
            OperationType::Divide => x/y,
            OperationType::Add => x+y,
            OperationType::Subtract => x-y
        }

    }


    fn pow(num: BigDecimal, p: f64) -> BigDecimal {
        if p==0.0 {
            return BigDecimal::from_str("1").unwrap();
        }
        if p<0.0 {
            return BigDecimal::from_str("1").unwrap()/OperationType::pow(num, -p);
        }

        const MAX_F64: f64 = 17976931348623156f64;

        let integer_power_part = p.floor() as u32;
        let mut result: BigDecimal = BigDecimal::from_str("1").unwrap();
        for _ in 0..integer_power_part {
            result *= &num;
        }
        let divider = &num/BigDecimal::from_f64(MAX_F64).unwrap();
        let dividend = &num/divider.to_f64().unwrap();
        let float_power_part = BigDecimal::from_f64(dividend.to_f64().unwrap().powf(p-p.floor())).unwrap()
                                                    *BigDecimal::from_f64(divider.to_f64().unwrap().powf(p-p.floor())).unwrap();
        result *= float_power_part;
        result.clone()
    }
}


pub fn calculate_expression(expression: &str) -> f64 {
    let re = Regex::new(r"^(\s*)(-)*(\s*)\d+(\.\d+)*((\s*)(?:[\^\*//+-](\s*)(-)*(\s*)\d+(\.\d+)*))+$").unwrap();

    if !re.is_match(expression) {
        //println!("Error, wrong syntax");
    };

    let re_tag = Regex::new(r"(\d+(\.\d+)*)|(\(([^)]*)\))|[\^\*//+-]").unwrap();
    let re_number = Regex::new(r"(\d+(\.\d+)*)").unwrap();
    let re_operator = Regex::new(r"[\^\*//+-]").unwrap();
    let re_p = Regex::new(r"(\(([^)]*)\))").unwrap();
    let re_minus_before_number = Regex::new(r"[\^\*//+-](\s*)[-]").unwrap();


    let mut sequence: Vec<Tag> = Vec::new();

    for tag in re_tag.find_iter(expression) {
        if re_number.is_match(tag.as_str()) {
            sequence.push(Tag::Number(BigDecimal::from_str(tag.as_str()).unwrap()));
        }
        if re_operator.is_match(tag.as_str()) {
            sequence.push(Tag::Operator(OperationType::parse(tag.as_str().chars().nth(0).unwrap())));
        }
    }

    println!("sequence before: {:?}", sequence);

    let mut to_delete: Vec<usize> = Vec::new();
    let mut pos = 0;
    let mut deleted = 0;
    loop {
        match &sequence[pos] {
            Tag::Number(number) => {

            },
            Tag::Operator(operator) => {
                match operator {
                    OperationType::Add => {
                        sequence.remove(pos);
                        deleted += 1;
                    },
                    OperationType::Subtract => {
                        match &sequence[pos+1] {
                            Tag::Number(value) => {
                                    sequence[pos+1] = Tag::Number(-value);
                            },
                            Tag::Operator(OperationType::Subtract) => {
                                sequence.remove(pos+1);
                                deleted += 1;
                            }
                            _ => ()
                        }
                        sequence.remove(pos);
                        deleted += 1;
                    },
                    _ => ()
                }
            }
        }
        pos += 1;
        if pos >= sequence.len()-deleted { break; }
    }

    /*
    println!("{:?}", to_delete);
    for (pos, i) in to_delete.iter().enumerate() {
        sequence.remove(*i-pos);
    }*/

    println!("sequence after: {:?}", sequence);


    return 1.0;


}
