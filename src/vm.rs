#[derive(Clone, Debug, PartialEq)]
pub struct VM {
    // 現在実行中の関数のフレームポインタ(旧フレームポインタが入ってるスタックのアドレス。最初のローカル変数の一個前のアドレス)
    fp: usize,
    // 現在積んであるスタックの一個上のアドレス
    sp: usize,
    // 次に実行する命令のアドレス
    pc: usize,
    stack: Vec<usize>,
    program: Vec<Cmd>,
}

impl VM {
    fn new(program: Vec<Cmd>) -> VM {
        VM {
            fp: 0,
            stack: {
                let mut v = Vec::with_capacity(10);
                v.resize(10, 0);
                v
            },
            sp: 0,
            program,
            pc: 0,
        }
    }

    fn run(&mut self) -> usize {
        self.run_cmd();
        while self.pc != 0 {
            self.run_cmd();
        }
        self.peak()
    }

    fn push(&mut self, x: usize) {
        self.stack[self.sp] = x;
        self.sp += 1;
    }

    fn peak(&self) -> usize {
        self.stack[self.sp - 1]
    }

    fn pop(&mut self) -> usize {
        let x = self.peak();
        self.sp -= 1;
        x
    }

    fn debug_state(&self) -> String {
        format!(
            "pc:{} fp:{} stack:{:?}",
            self.pc,
            self.fp,
            self.stack
                .clone()
                .into_iter()
                .take(self.sp)
                .collect::<Vec<_>>()
        )
    }

    fn run_cmd(&mut self) {
        println!("[run]{:?}", self.program[self.pc]);
        println!("[state] {}", self.debug_state());
        let cmd = self.program[self.pc].clone();
        match cmd {
            Cmd::Entry(i) => {
                self.pc = i;
                self.push(0);
            }
            Cmd::Frame(local_count) => {
                self.push(self.fp);
                self.fp = self.sp - 1;
                self.sp += local_count;

                self.pc += 1;
            }
            Cmd::Ret => {
                let res = self.peak();
                self.sp = self.fp;
                self.pc = self.stack[self.fp - 1];
                self.fp = self.stack[self.fp];
                self.push(res);
            }
            Cmd::Call(i) => {
                self.push(self.pc + 1);

                self.pc = i;
            }
            Cmd::LocalLoad(i) => {
                self.push(self.stack[self.fp + i + 1]);

                self.pc += 1;
            }
            Cmd::LocalStore(i) => {
                self.stack[self.fp + i + 1] = self.pop();

                self.pc += 1;
            }
            Cmd::ArgLoad(i) => {
                self.push(self.stack[self.fp - i - 2]);
                self.pc += 1;
            }
            Cmd::ArgStore(i) => {
                self.stack[self.fp - i - 2] = self.pop();

                self.pc += 1;
            }
            Cmd::PopR(i) => {
                let res = self.pop();
                self.sp -= i - 1;
                self.push(res);

                self.pc += 1;
            }
            Cmd::Const(x) => {
                self.push(x);

                self.pc += 1;
            }
            Cmd::Add => {
                let x = self.pop();
                let y = self.pop();
                self.push(x + y);

                self.pc += 1;
            }
        }
        println!("[result]{}", self.debug_state());
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum Cmd {
    Frame(usize),
    Ret,
    Call(usize),
    LocalLoad(usize),
    LocalStore(usize),
    ArgLoad(usize),
    ArgStore(usize),
    PopR(usize),
    Const(usize),
    Add,
    Entry(usize),
}

#[test]
fn test() {
    assert_eq!(
        VM::new(vec![
            Cmd::Entry(1),
            Cmd::Frame(0),
            Cmd::Const(1),
            Cmd::Const(2),
            Cmd::Call(7),
            Cmd::PopR(2),
            Cmd::Ret,
            Cmd::Frame(0),
            Cmd::ArgLoad(0),
            Cmd::ArgLoad(1),
            Cmd::Add,
            Cmd::Ret
        ])
        .run(),
        3
    );
}
