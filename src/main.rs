pub struct Machine {
  pub stack: Vec<i32>,
  pub prog: Vec<i32>,
  pub ip: i32,
}

const PUSH: i32 = 0x01;
const ADD: i32 = 0x02;
const SUB: i32 = 0x03;
const MUL: i32 = 0x04;
const DIV: i32 = 0x05;
const INC: i32 = 0x06;
const DEC: i32 = 0x07;
const JMP: i32 = 0x20;
const CMP: i32 = 0x30;
const JL: i32 = 0x31;
const CALL: i32 = 0x40;
const RET: i32 = 0x41;
const DBG: i32 = 0x70;
const HLT: i32 = 0x80;

impl Machine {
  pub fn new(prog: Vec<i32>) -> Self {
    let stack = Vec::with_capacity(100);
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

    match dbg!(*ins) {
      PUSH => {
        let val = *self.next()?;
        self.stack.push(val)
      }
      ADD => {
        let res = self.arith(core::ops::Add::add)?;
        self.stack.push(res)
      }
      SUB => {
        let res = self.arith(core::ops::Sub::sub)?;
        self.stack.push(res)
      }
      MUL => {
        let res = self.arith(core::ops::Mul::mul)?;
        self.stack.push(res)
      }
      DIV => {
        let res = self.arith(core::ops::Div::div)?;
        self.stack.push(res)
      }
      INC => {
        let last = self.stack.last_mut()?;
        *last += 1;
      }
      DEC => {
        let last = self.stack.last_mut()?;
        *last -= 1;
      }
      JMP => {
        let ip = self.stack.pop()?;
        self.ip = ip;
      }
      CMP => {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        self.stack.push(a - b);
      }
      JL => {
        let ip = *self.next()?;
        let cmp = self.stack.pop()?;
        if cmp < 0 {
          self.ip = ip;
        }
      }
      CALL => {
        let ret_ip = self.ip;
        let jmp_location = *self.next()?;
        let swap_val = self.stack.pop()?;

        self.stack.push(ret_ip);
        self.stack.push(swap_val);
        self.ip = jmp_location;
      }
      RET => {
        let current_val = self.stack.pop()?;
        self.ip = self.stack.pop()? + 1; // is this right?
        self.stack.push(current_val);
      }
      DBG => {
        let last = self.stack.last()?;
        println!("-> {last}");
      }
      HLT => {
        let val = self.stack.pop()?;
        hlt = Some(val);
      }
      _ => unreachable!(),
    }

    hlt
  }

  /// An arithmetic operation.
  fn arith(&mut self, arith: fn(i32, i32) -> i32) -> Option<i32> {
    let a = self.stack.pop()?;
    let b = self.stack.pop()?;
    Some(arith(a, b))
  }

  /// Executes the current program.
  pub fn go(&mut self) -> Option<i32> {
    loop {
      match self.step() {
        None => continue,
        Some(val) => {
          return Some(val);
        }
      }
    }
  }
}

fn main() {
  println!("Hello, world!");

  // let prog = vec![PUSH, 4, PUSH, 4, CMP, DBG, JL, 0, PUSH, 0, HLT];
  let prog = vec![PUSH, 0, CALL, 7, INC, DBG, RET, PUSH, 41, CALL, 4, HLT];

  let mut mach = Machine::new(prog);

  let res = mach.go();

  println!("{res:?}");
}
