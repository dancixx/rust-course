use std::str::FromStr;

struct Equation {
    left: String,
    right: String,
    is_equal: bool,
}

struct ParseEquationError;

impl FromStr for Equation {
    type Err = ParseEquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once("=")
            .and_then(|(l, r)| {
                let left = sum(l);
                let right = sum(r);

                Some(Equation {
                    left: l.to_string(),
                    right: r.to_string(),
                    is_equal: left == right,
                })
            })
            .ok_or(ParseEquationError)
    }
}

fn sum(s: &str) -> usize {
    s.split("+").map(|n| n.parse::<usize>().unwrap_or(0)).sum()
}

fn test_equation(s: &str) {
    let equation = s.parse::<Equation>();

    match equation {
        Ok(Equation {
            left,
            right,
            is_equal,
        }) => {
            if is_equal {
                println!("{} = {}", left, right);
            } else {
                println!("{} != {}", left, right);
            }
        }
        Err(_) => println!("{} is not a valid equation", s),
    }
}

pub fn run() {
    test_equation("1+2=3");
    test_equation("1+2=4");
    test_equation("1+2=3+4");
    test_equation("1+2=3+4+5");
    test_equation("1+2=3+4+5+6");
    test_equation("1+2=3+4+5+6+7");
    test_equation("1+2=3+4+5+6+7+8");
    test_equation("1+2=3+4+5+6+7+8+9");
    test_equation("1+2=3+4+5+6+7+8+9+10");
    test_equation("1+2=3+4+5+6+7+8+9+10+11");

    test_equation("1+2+3=4+5+6");
    test_equation("1+2+3+4=5+6+7+8");
    test_equation("1+2+3+4+5=6+7+8+9+10");
    test_equation("1+2+3+4+5+6=7+8+9+10+11+12");
    test_equation("1+2+3+4+5+6+7=8+9+10+11+12+13+14");
}
