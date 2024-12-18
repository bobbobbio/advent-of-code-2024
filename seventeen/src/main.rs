use advent::prelude::*;

#[derive(HasParser)]
#[parse(sep_by = "\n")]
struct Input {
    #[parse(before = "Register A: ")]
    a: u64,
    #[parse(before = "Register B: ")]
    b: u64,
    #[parse(before = "Register C: ")]
    c: u64,

    #[parse(before = "\nProgram: ")]
    program: List<u64, SepBy<Comma>>,
}

#[derive(Clone)]
struct Machine {
    a: u64,
    b: u64,
    c: u64,
    pc: usize,

    program: Vec<u64>,
    out: Vec<u64>,
}

struct InvalidPc;
type Result<T> = std::result::Result<T, InvalidPc>;

impl Machine {
    fn new(input: Input) -> Self {
        Self {
            a: input.a,
            b: input.b,
            c: input.c,
            pc: 0,
            program: input.program.to_vec(),
            out: vec![],
        }
    }

    fn next(&mut self) -> Result<u64> {
        if self.pc >= self.program.len() {
            return Err(InvalidPc);
        }

        let old_pc = self.pc;
        self.pc += 1;
        Ok(self.program[old_pc])
    }

    fn combo(&mut self) -> Result<u64> {
        Ok(match self.next()? {
            v @ 0..=3 => v,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!(),
        })
    }

    fn run_one(&mut self) -> Result<()> {
        match self.next()? {
            0 => {
                self.a = self.a / 2u64.pow(self.combo()? as u32);
            }
            1 => {
                self.b = self.b ^ self.next()?;
            }
            2 => {
                self.b = self.combo()? % 8;
            }
            3 => {
                if self.a != 0 {
                    self.pc = self.next()? as usize;
                }
            }
            4 => {
                let _ = self.next()?;
                self.b = self.b ^ self.c;
            }
            5 => {
                let v = self.combo()? % 8;
                self.out.push(v);
            }
            6 => {
                self.b = self.a / 2u64.pow(self.combo()? as u32);
            }
            7 => {
                self.c = self.a / 2u64.pow(self.combo()? as u32);
            }
            _ => panic!(),
        };
        Ok(())
    }
}

#[part_one]
fn part_one(input: Input) -> String {
    let mut m = Machine::new(input);
    while m.run_one().is_ok() {}
    Vec::from_iter(m.out.into_iter().map(|v| v.to_string())).join(",")
}

#[part_two]
fn part_two(input: Input) -> u64 {
    let m = Machine::new(input);
    for new_a in 0.. {
        let mut m = m.clone();
        m.a = new_a;
        while m.run_one().is_ok() {
            if m.out != &m.program[..m.out.len()] {
                break;
            }
        }
        if m.out == m.program {
            return new_a;
        }
    }
    panic!("new value for a not found")
}

harness!(part_1: "6,5,7,4,5,7,3,1,0");
