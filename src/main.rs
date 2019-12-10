use std::fmt;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum ParseInputError {
    ParseError(ParseIntError),
    InputTooLong,
}

#[derive(Copy, Clone)]
struct D2 {
    value: u8,
}

impl D2 {
    fn new(v: u8) -> D2 {
        D2 { value: v }
    }
}

impl fmt::Display for D2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}", self.value)
    }
}

#[derive(Copy, Clone)]
struct D3 {
    value: i16,
}

impl D3 {
    fn new(v: i16) -> D3 {
        D3 { value: v }
    }
}

impl fmt::Display for D3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:+04}", self.value)
    }
}

enum LmcResult {
    Success,
    Error(D2, &'static str),
}

struct Lmc {
    pc: D2,
    acc: D3,
    mem: [D3; 100],
    out: String,
}

impl Lmc {
    fn new(pc: D2, acc: D3, mem: [D3; 100]) -> Lmc {
        Lmc {
            pc,
            acc,
            mem,
            out: String::new(),
        }
    }
    /*
        fn new_empty() -> Lmc {
            let m = [D3::new(0); 100];
            Lmc::new(D2::new(0), D3::new(0), m)
        }
    */
    fn from(s: &str) -> Result<Lmc, ParseInputError> {
        let pc = D2::new(0);
        let acc = D3::new(0);

        let mem: [D3; 100] = unsafe {
            let mut array: [D3; 100] = std::mem::uninitialized();
            let mut lines = s.split_whitespace();

            for v in array.iter_mut() {
                if let Some(line) = lines.next() {
                    match line.parse::<i16>() {
                        Ok(i) => *v = D3::new(i),
                        Err(e) => return Err(ParseInputError::ParseError(e)),
                    }
                } else {
                    *v = D3::new(0);
                }
            }
            // If there are still lines left in the input but memory is full,
            if lines.peekable().peek().is_some() {
                return Err(ParseInputError::InputTooLong);
            }
            array
        };

        Result::Ok(Lmc::new(pc, acc, mem))
    }

    fn cycle(&mut self) -> LmcResult {
        // =====Fetch=====
        let instruction = self.mem[self.pc.value as usize].value;

        // Increment the PC
        self.pc.value += 1;

        // =====Decode====
        if instruction < 0 {
            return LmcResult::Error(self.pc, "Negative instruction...?");
        }
        let opcode = (instruction / 100) as u8;
        let operand = (instruction % 100) as u8;

        // ====Execute====
        match (opcode, operand) {
            // HLT
            (0, _) => return LmcResult::Error(self.pc, "Halt command executed."),
            // ADD
            (1, _) => {
                self.acc.value += self.mem[operand as usize].value;
                if self.acc.value > 999 {
                    self.acc.value = 999;
                }
            }
            // SUB
            (2, _) => {
                self.acc.value -= self.mem[operand as usize].value;
                if self.acc.value < -999 {
                    self.acc.value = -999;
                }
            }
            // LDA
            (3, _) => self.mem[operand as usize] = self.acc,
            // STO
            (5, _) => self.acc = self.mem[operand as usize],
            // BRA
            (6, _) => self.pc = D2 { value: operand },
            // BRZ
            (7, _) => {
                if self.acc.value == 0 {
                    self.pc = D2 { value: operand }
                }
            }
            // BRP
            (8, _) => {
                if self.acc.value >= 0 {
                    self.pc = D2 { value: operand }
                }
            }
            // INP
            (9, 1) => {
                let mut invalid = true;
                while invalid {
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Error reading line.");

                    match input.trim().parse::<i16>() {
                        Err(_) => invalid = true,
                        Ok(i) => {
                            self.acc = D3::new(i);
                            invalid = false;
                        }
                    }
                }
            }
            // OUT
            (9, 2) => {
                let s = self.acc.value.to_string();
                self.out.push_str(&s[..]);
                self.out.push(' ');
            }
            // OTC
            (9, 22) => {
                let c = self.acc.value as u8;
                self.out.push(char::from(c));
            }
            _ => return LmcResult::Error(self.pc, "Bad instruction."),
        }

        // Checks for more instructions.
        if self.pc.value > 99 {
            return LmcResult::Error(self.pc, "No more instructions.");
        }

        LmcResult::Success
    }
}

impl fmt::Display for Lmc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = write!(f, "PC:{}, ACC:{}, mem:\n", self.pc, self.acc);
        for (i, v) in self.mem.iter().enumerate() {
            if i % 10 == 9 {
                writeln!(f, "{}", v)?;
            } else {
                write!(f, "{} ", v)?;
            }
        }
        write!(f, "Out: {}", self.out)?;
        
        result
    }
}

fn main() {
<<<<<<< HEAD
    // Read file.
    use std::io::Read;
    let mut file = std::fs::File::open("program.txt").expect("couldn't open program.txt");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("couldn't read program.txt");

    // Parse file.
    let mut lmc = Lmc::from(&contents).unwrap_or_else(|e| panic!("Error in input {:#?}!", e));

    // Run emulator.
=======
    let mut lmc = Lmc::from("514\n317\n517\n902\n514\n922\n517\n922\n115\n317\n216\n713\n602\n000\n032\n001\n097\n000\n")
        .unwrap_or_else(|e| panic!("Error in input {:#?}", e));

>>>>>>> 2e705f3d6105f58d72329bfb377dfc1ca365299c
    loop {
        match lmc.cycle() {
            LmcResult::Success => (),
            LmcResult::Error(pc, msg) => {
                println!("Exception at {}:", pc);
                println!("\t{}", msg);
                break;
            }
        }
    }
    println!("{}", lmc);
}
