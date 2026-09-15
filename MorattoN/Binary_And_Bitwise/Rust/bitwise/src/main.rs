use std::fmt::{self, Display, Formatter};
use std::io::{self, Write};

enum Operation {
    And,
    Or,
    Xor,
    Lshift,
    Rshift,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Operation::And => write!(f, "and"),
            Operation::Or => write!(f, "or"),
            Operation::Xor => write!(f, "xor"),
            Operation::Lshift => write!(f, "lshift"),
            Operation::Rshift => write!(f, "rshift"),
        }
    }
}

fn get_valid_isize(value_number: i8) -> io::Result<i8> {
    loop {
        let mut raw_value = String::new();
        print!("Enter a number between -255 and 255 for value {value_number}: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut raw_value)?;
        match raw_value.trim().parse::<i8>() {
            Ok(v) => return Ok(v),
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

fn get_valid_operation() -> io::Result<Operation> {
    use Operation::*;

    loop {
        let mut raw_value = String::new();
        print!("Enter one of [and, &, or, |, xor, ^, lshift, <<, rshift, >>]: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut raw_value)?;
        match raw_value.to_lowercase().trim() {
            "and" | "&" => return Ok(And),
            "or" | "|" => return Ok(Or),
            "xor" | "^" => return Ok(Xor),
            "lshift" | "<<" => return Ok(Lshift),
            "rshift" | ">>" => return Ok(Rshift),
            _ => eprintln!("Please enter a valid operation!"),
        }
    }
}

fn do_operation(value1: i8, value2: i8, operation: Operation) {
    use Operation::*;

    let result: i8 = match operation {
        And => value1 & value2,
        Or => value1 | value2,
        Xor => value1 ^ value2,
        Lshift => value1 << value2,
        Rshift => value1 >> value2,
    };

    println!("***********************************");
    println!("{value1} in binary:\t\t{value1:08b}");
    println!("{value2} in binary:\t\t{value2:08b}");
    println!("{value1} {operation} {value2}: \t\t{result:08b}");
}

fn main() -> io::Result<()> {
    loop {
        let value1 = get_valid_isize(1)?;
        let value2 = get_valid_isize(2)?;
        let operation = get_valid_operation()?;

        do_operation(value1, value2, operation);
    }
}
