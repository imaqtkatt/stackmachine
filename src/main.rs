pub struct Machine {
  pub stack: Vec<i32>,
  pub prog: Vec<i32>,
  pub ip: i32,
}

const PUSH: i32 = 0x01;
const ADD: i32 = 0x02;
const INC: i32 = 0x03;
const DEC: i32 = 0x04;
const HLT: i32 = 0x80;

impl Machine {
  pub fn new(prog: Vec<i32>) -> Self {
    let stack = Vec::with_capacity(10);
    let ip = 0;

    Machine { stack, prog, ip }
  }
}

impl Machine {
  pub fn next(&mut self) -> Option<&i32> {
    let ins = self.prog.get(self.ip as usize);
    self.ip += 1;
    ins
  }

  pub fn step(&mut self) -> Option<i32> {
    let ins = self.next()?;

    let mut hlt = None;

    match *ins {
      PUSH => {
        let val = *self.next()?;
        self.stack.push(val)
      }
      ADD => {
        let l = self.stack.pop()?;
        let r = self.stack.pop()?;
        self.stack.push(l + r);
      }
      INC => {
        let lst = self.stack.last_mut()?;
        *lst += 1;
      }
      DEC => {
        let lst = self.stack.last_mut()?;
        *lst -= 1;
      }
      HLT => {
        let val = self.stack.pop()?;
        hlt = Some(val);
      }
      _ => unreachable!(),
    }

    hlt
  }
}

fn main() {
  println!("Hello, world!");

  let prog = vec![PUSH, 1, INC, PUSH, 2, DEC, ADD, HLT];

  let mut mach = Machine::new(prog);

  mach.step();
  mach.step();
  mach.step();
  mach.step();
  mach.step();
  let res = mach.step();

  println!("{res:?}");
}
