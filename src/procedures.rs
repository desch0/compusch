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


fn tear_pow(sequence: &mut Vec<Tag>) -> Result<(), String> {
    let mut pos = sequence.len()-1;
    loop {
        match sequence[pos] {
            Tag::Operator(OperationType::Power) => {
                match &sequence[pos-1] {
                    Tag::Number(value) => {
                        match &sequence[pos+1] {
                            Tag::Number(value1) => {
                                sequence[pos-1] = Tag::Number(OperationType::pow(value.clone(), value1.to_f64().unwrap()));
                                sequence.remove(pos+1);
                                sequence.remove(pos);
                            },
                            Tag::Operator(OperationType::Subtract) => {
                                match &sequence[pos+2] {
                                    Tag::Number(value2) => {
                                        sequence[pos+1] = Tag::Number(-value2);
                                        sequence.remove(pos+2);
                                        pos += 1
                                    },
                                    _ => ()
                                }
                            },
                            _ => return Err("error: wrong syntax".to_string())
                        }

                    },
                    _ => return Err("error: wrong syntax".to_string())
                }

            },
            _ => ()
        }
        if pos == 0 { break; }
        pos -= 1;
    }
    Ok(())
}


fn tear_add_and_sub(sequence: &mut Vec<Tag>) -> Result<(), String> {
    let mut pos = 0;
    loop {
        match &sequence[pos] {
            Tag::Operator(operator) => {
                match operator {
                    OperationType::Add => {
                        sequence.remove(pos);
                    },
                    OperationType::Subtract => {
                        match &sequence[pos+1] {
                            Tag::Number(value) => {
                                    sequence[pos+1] = Tag::Number(-value);
                            },
                            Tag::Operator(OperationType::Subtract) => {
                                sequence.remove(pos+1);
                            }
                            _ => return Err("error: wrong syntax".to_string())
                        }
                        sequence.remove(pos);
                    },
                    _ => ()
                }
            },
            _ => ()
        }
        pos += 1;
        if pos >= sequence.len() { break; }
    }
    Ok(())
}


fn tear_mult_and_div(sequence: &mut Vec<Tag>) -> Result<(), String> {
    let mut pos = sequence.len()-1;
    loop {
        match &sequence[pos] {
            Tag::Operator(operator) => {
                match &sequence[pos-1] {
                    Tag::Number(value) => {
                        match &sequence[pos+1] {
                            Tag::Number(value1) => {
                                match operator {
                                    OperationType::Multiply => {
                                        sequence[pos-1] = Tag::Number(value*value1);
                                        sequence.remove(pos+1);
                                        sequence.remove(pos);
                                    },
                                    OperationType::Divide => {
                                        sequence[pos-1] = Tag::Number(value/value1);
                                        sequence.remove(pos+1);
                                        sequence.remove(pos);
                                    },
                                    _ => ()
                                }
                            },
                            _ => return Err("error: wrong syntax".to_string())
                        }
                    },
                    _ => return Err("error: wrong syntax".to_string())
                }

            },
            _ => ()
        }
        if pos == 0 { break; }
        pos -= 1;
    }
    Ok(())
}


fn take_sum(sequence: &mut Vec<Tag>) -> BigDecimal {
    let mut result = BigDecimal::from_f64(0.0).unwrap();

    for i in 0..sequence.len() {
        match &sequence[i] {
            Tag::Number(value) => {
			result += value;	
		}
            _ => ()
        }
    }

    return result;
}


pub fn calculate_expression(expression: &str) -> Result<BigDecimal, String> {
    let re_tag = Regex::new(r"(\d+(\.\d+)*)|(\(([^)]*)\))|[\^\*//+-]").unwrap();
    let re_number = Regex::new(r"(\d+(\.\d+)*)").unwrap();
    let re_operator = Regex::new(r"[\^\*//+-]").unwrap();

    let mut sequence: Vec<Tag> = Vec::new();

    for tag in re_tag.find_iter(expression) {
        if re_number.is_match(tag.as_str()) {
            sequence.push(Tag::Number(BigDecimal::from_str(tag.as_str()).unwrap()));
        }
        if re_operator.is_match(tag.as_str()) {
            sequence.push(Tag::Operator(OperationType::parse(tag.as_str().chars().nth(0).unwrap())));
        }
    }

    match tear_pow(&mut sequence) {
        Ok(_) => (),
        Err(e) => return Err(e)
    }
    match tear_add_and_sub(&mut sequence) {
        Ok(_) => (),
        Err(e) => return Err(e)
    }
    match tear_mult_and_div(&mut sequence) {
        Ok(_) => (),
        Err(e) => return Err(e)
    }
    Ok(take_sum(&mut sequence))
}
    