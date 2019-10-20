use crate::vm::{Cmd, VM};

#[derive(Clone, Debug, PartialEq)]
enum LLangCmd {
    Frame(usize),
    Ret,
    Call(FnIndex),
    LocalLoad(usize),
    LocalStore(usize),
    ArgLoad(usize),
    ArgStore(usize),
    PopR(usize),
    Const(usize),
    Add,
    Mod,
    Entry(FnIndex),
    Eq,
    JumpIf(RelativeFnIndex),
    Jump(RelativeFnIndex),
}

#[derive(Clone, Debug, PartialEq)]
struct FnIndex(usize);

#[derive(Clone, Debug, PartialEq)]
struct RelativeFnIndex(FnIndex, usize);

#[derive(Clone, Debug, PartialEq)]
pub struct LLang {
    entry: usize,
    funcs: Vec<Func>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Func {
    local_count: usize,
    ops: Vec<Op>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Call(usize),
    LocalLoad(usize),
    LocalStore(usize),
    ArgLoad(usize),
    ArgStore(usize),
    Const(usize),
    Add,
    Mod,
    Eq,
    JumpIf(usize),
    Jump(usize),
    PopR(usize),
}

#[derive(Clone, Debug, PartialEq)]
struct CmdGen {
    cmds: Vec<LLangCmd>,
    funcs: Vec<usize>,
}

impl CmdGen {
    fn new() -> CmdGen {
        CmdGen {
            cmds: Vec::new(),
            funcs: Vec::new(),
        }
    }

    fn push(&mut self, cmd: LLangCmd) {
        if let &LLangCmd::Frame(_) = &cmd {
            self.funcs.push(self.cmds.len());
        }
        self.cmds.push(cmd);
    }

    fn to_cmds(self) -> Vec<Cmd> {
        let cmds = self.cmds;
        let funcs = self.funcs;
        cmds.into_iter()
            .map(|cmd| match cmd {
                LLangCmd::Frame(x) => Cmd::Frame(x),
                LLangCmd::Ret => Cmd::Ret,
                LLangCmd::Call(FnIndex(i)) => Cmd::Call(funcs[i]),
                LLangCmd::LocalLoad(x) => Cmd::LocalLoad(x),
                LLangCmd::LocalStore(x) => Cmd::LocalStore(x),
                LLangCmd::ArgLoad(x) => Cmd::ArgLoad(x),
                LLangCmd::ArgStore(x) => Cmd::ArgStore(x),
                LLangCmd::PopR(x) => Cmd::PopR(x),
                LLangCmd::Const(x) => Cmd::Const(x),
                LLangCmd::Add => Cmd::Add,
                LLangCmd::Mod => Cmd::Mod,
                LLangCmd::Entry(FnIndex(i)) => Cmd::Entry(funcs[i]),
                LLangCmd::Eq => Cmd::Eq,
                LLangCmd::JumpIf(RelativeFnIndex(FnIndex(i), x)) => Cmd::JumpIf(funcs[i] + x + 1),
                LLangCmd::Jump(RelativeFnIndex(FnIndex(i), x)) => Cmd::Jump(funcs[i] + x + 1),
            })
            .collect()
    }
}

impl LLang {
    fn convert(&self) -> Vec<Cmd> {
        let mut gen = CmdGen::new();
        let entry = gen.push(LLangCmd::Entry(FnIndex(self.entry)));
        for (i, func) in self.funcs.iter().enumerate() {
            func.convert(i, &mut gen);
        }
        gen.to_cmds()
    }
}

impl Func {
    fn convert(&self, fn_index: usize, gen: &mut CmdGen) {
        gen.push(LLangCmd::Frame(self.local_count));
        for op in &self.ops {
            op.convert(fn_index, gen);
        }
        gen.push(LLangCmd::Ret);
    }
}

impl Op {
    fn convert(&self, fn_index: usize, gen: &mut CmdGen) {
        gen.push(match self {
            Op::Call(x) => LLangCmd::Call(FnIndex(*x)),
            Op::LocalLoad(x) => LLangCmd::LocalLoad(*x),
            Op::LocalStore(x) => LLangCmd::LocalStore(*x),
            Op::ArgLoad(x) => LLangCmd::ArgLoad(*x),
            Op::ArgStore(x) => LLangCmd::ArgStore(*x),
            Op::Const(x) => LLangCmd::Const(*x),
            Op::Add => LLangCmd::Add,
            Op::Mod => LLangCmd::Mod,
            Op::Eq => LLangCmd::Eq,
            Op::JumpIf(x) => LLangCmd::JumpIf(RelativeFnIndex(FnIndex(fn_index), *x)),
            Op::Jump(x) => LLangCmd::Jump(RelativeFnIndex(FnIndex(fn_index), *x)),
            Op::PopR(x) => LLangCmd::PopR(*x),
        });
    }
}

#[test]
fn test() {
    assert_eq!(
        VM::new(
            (LLang {
                entry: 0,
                funcs: vec![
                    Func {
                        local_count: 0,
                        ops: vec![Op::Const(182), Op::Const(1029), Op::Call(1), Op::PopR(2)]
                    },
                    Func {
                        local_count: 0,
                        ops: vec![
                            Op::ArgLoad(0),
                            Op::Const(0),
                            Op::Eq,
                            Op::JumpIf(5),
                            Op::Jump(7),
                            Op::ArgLoad(1),
                            Op::Jump(13),
                            Op::ArgLoad(0),
                            Op::ArgLoad(0),
                            Op::ArgLoad(1),
                            Op::Mod,
                            Op::Call(1),
                            Op::PopR(2),
                        ]
                    }
                ],
            })
            .convert()
        )
        .run(),
        7
    );
}
