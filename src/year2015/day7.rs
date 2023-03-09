use std::{collections::HashMap, str::FromStr, vec};

use crate::utils::ParseError;

#[derive(Debug)]
enum Op {
    Assign,
    Not,
    And,
    Or,
    LShift,
    RShift,
}

impl Op {
    fn exec(&self, inputs: Vec<u16>) -> u16 {
        match self {
            Op::Assign => inputs[0],
            Op::Not => !inputs[0],
            Op::And => inputs[0] & inputs[1],
            Op::Or => inputs[0] | inputs[1],
            Op::LShift => inputs[0] << inputs[1],
            Op::RShift => inputs[0] >> inputs[1],
        }
    }
}

#[derive(Debug)]
enum Input {
    Value(u16),
    Gate(String),
}

impl Input {
    fn gate(id: &str) -> Self {
        Input::Gate(id.to_owned())
    }
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_digit(10)) {
            Ok(Input::Value(s.parse()?))
        } else {
            Ok(Input::Gate(s.to_owned()))
        }
    }
}

#[derive(Debug)]
struct Gate {
    inputs: Vec<Input>,
    op: Op,
}

fn _to_input(s: &str) -> Input {
    Input::from_str(s).unwrap()
}

impl Gate {
    fn assign(input: &str) -> Self {
        Gate {
            inputs: vec![_to_input(input)],
            op: Op::Assign,
        }
    }

    fn not(input: &str) -> Self {
        Gate {
            inputs: vec![_to_input(input)],
            op: Op::Not,
        }
    }

    fn create(op: &str, first: &str, second: &str) -> Self {
        Gate {
            inputs: vec![_to_input(first), _to_input(second)],
            op: match op {
                "AND" => Op::And,
                "OR" => Op::Or,
                "LSHIFT" => Op::LShift,
                "RSHIFT" => Op::RShift,
                _ => panic!("Invalid operation: {}!!!", op),
            },
        }
    }
}

#[derive(Debug)]
struct Wires {
    gates: HashMap<String, Gate>,
    values: HashMap<String, u16>,
}

impl Wires {
    fn parse(input: &str) -> Self {
        let mut gates: HashMap<String, Gate> = HashMap::new();

        input.lines().for_each(|line| {
            let tokens = line.split(" ").collect::<Vec<_>>();
            if tokens.len() == 3 {
                assert_eq!(tokens[1], "->");
                gates.insert(tokens[2].to_owned(), Gate::assign(tokens[0]));
            } else if tokens.len() == 4 {
                assert_eq!(tokens[0], "NOT");
                assert_eq!(tokens[2], "->");
                gates.insert(tokens[3].to_owned(), Gate::not(tokens[1]));
            } else if tokens.len() == 5 {
                assert_eq!(tokens[3], "->");
                gates.insert(
                    tokens[4].to_owned(),
                    Gate::create(tokens[1], tokens[0], tokens[2]),
                );
            }
        });
        Wires {
            gates,
            values: HashMap::new(),
        }
    }

    fn _get_gate_val_memo(&self, gate: &Input, memo: &mut HashMap<String, u16>) -> u16 {
        let res = match gate {
            Input::Value(x) => *x,
            Input::Gate(ref id) => {
                if memo.contains_key(id) {
                    memo[id]
                } else {
                    let res = self
                        .gates
                        .get(id)
                        .and_then(|gate_info| {
                            let inputs: Vec<u16> = gate_info
                                .inputs
                                .iter()
                                .map(|x| self._get_gate_val_memo(x, memo))
                                .collect();
                            Some(gate_info.op.exec(inputs))
                        })
                        .unwrap();
                    memo.insert(id.to_owned(), res);
                    //eprintln!("calculated wire {:?} = {}", gate, res);
                    res
                }
            }
        };
        res
    }

    fn get_gate_val(&mut self, gate: &str) -> u16 {
        let mut memo = self.values.clone();
        let res = self._get_gate_val_memo(&Input::gate(gate), &mut memo);
        self.values = memo;
        res
    }

    fn reset(&mut self) {
        self.values = HashMap::new();
    }

    fn set_gate(&mut self, gate: &str, val: u16) {
        self.values.insert(gate.to_owned(), val);
    }
}

pub fn part1(input: &str) -> u16 {
    let mut wires = Wires::parse(input);
    wires.get_gate_val("a")
}

pub fn part2(input: &str) -> u16 {
    let mut wires = Wires::parse(input);
    let a = wires.get_gate_val("a");
    wires.reset();
    wires.set_gate("b", a);
    wires.get_gate_val("a")
}

#[test]
fn test() {
    fn get(wires: &mut Wires, gate: &str) -> u16 {
        wires.get_gate_val(gate)
    }
    let wires = &mut Wires::parse(&crate::utils::load_test_file(2015, 7));
    assert_eq!(get(wires, "d"), 72);
    assert_eq!(get(wires, "e"), 507);
    assert_eq!(get(wires, "f"), 492);
    assert_eq!(get(wires, "g"), 114);
    assert_eq!(get(wires, "h"), 65412);
    assert_eq!(get(wires, "i"), 65079);
    assert_eq!(get(wires, "x"), 123);
    assert_eq!(get(wires, "y"), 456);
}
