// https://www.codewars.com/kata/58e61f3d8ff24f774400002c/train/rust
// https://schweigi.github.io/assembler-simulator/
// https://github.com/Schweigi/assembler-simulator
// https://docs.rs/register/latest/register/
// http://ref.x86asm.net/coder32.html

/*
We want to create an interpreter of assembler which will support the following instructions:
mov x, y - copy y (either an integer or the value of a register) into register x.
inc x - increase the content of register x by one.
dec x - decrease the content of register x by one.
add x, y - add the content of the register x with y (either an integer or the value of a register) and stores the result in x (i.e. register[x] += y).
sub x, y - subtract y (either an integer or the value of a register) from the register x and stores the result in x (i.e. register[x] -= y).
mul x, y - same with multiply (i.e. register[x] *= y).
div x, y - same with integer division (i.e. register[x] /= y).
label: - define a label position (label = identifier + ":", an identifier being a string that does not match any other command). Jump commands and call are aimed to these labels positions in the program.
jmp lbl - jumps to to the label lbl.

cmp x, y - compares x (either an integer or the value of a register) and y (either an integer or the value of a register). The result is used in the conditional jumps (jne, je, jge, jg, jle and jl)
jne lbl - jump to the label lbl if the values of the previous cmp command were not equal.
je lbl - jump to the label lbl if the values of the previous cmp command were equal.
jge lbl - jump to the label lbl if x was greater or equal than y in the previous cmp command.
jg lbl - jump to the label lbl if x was greater than y in the previous cmp command.
jle lbl - jump to the label lbl if x was less or equal than y in the previous cmp command.
jl lbl - jump to the label lbl if x was less than y in the previous cmp command.

call lbl - call to the subroutine identified by lbl. When a ret is found in a subroutine, the instruction pointer should return to the instruction next to this call command.
ret - when a ret is found in a subroutine, the instruction pointer should return to the instruction that called the current function.
msg 'Register: ', x - this instruction stores the output of the program. It may contain text strings (delimited by single quotes) and registers. The number of arguments isn't limited and will vary, depending on the program.
end - this instruction indicates that the program ends correctly, so the stored output is returned (if the program terminates without this instruction it should return the default output: see below).
; comment - comments should not be taken in consideration during the execution of the program.
 */

use std::cmp::Ordering;
use std::collections::HashMap;

type MemoryAddr = usize;
type Num = i32;
type JmpLabel = String;
type FunctionLabel = String;

// mod opcodes
const COMMA: char = ',';
const COMMENT_INDICATOR: char = ';';
const COLON: char = ':';
const SINGLE_QUOTE: char = '\'';
const CMD_END: &'static str = "end";
const CMD_MOV: &'static str = "mov";
const CMD_INC: &'static str = "inc";
const CMD_DEC: &'static str = "dec";
const CMD_ADD: &'static str = "add";
const CMD_SUB: &'static str = "sub";
const CMD_MUL: &'static str = "mul";
const CMD_DIV: &'static str = "div";
const CMD_JMP: &'static str = "jmp";
const CMD_CMP: &'static str = "cmp";
const CMD_JNE: &'static str = "jne";
const CMD_JE: &'static str = "je";
const CMD_JGE: &'static str = "jge";
const CMD_JG: &'static str = "jg";
const CMD_JLE: &'static str = "jle";
const CMD_JL: &'static str = "jl";
const CMD_CALL: &'static str = "call";
const CMD_RET: &'static str = "ret";
const CMD_MSG: &'static str = "msg";

// integer, register, fixed-string
#[derive(Debug)]
enum OpNum {
    Memory(MemoryAddr),
    NUM(Num),
    FIXEDString(String),
}

impl OpNum {
    pub fn new(input: &str, mem: &mut StackMemory) -> Self {
        if input.starts_with(SINGLE_QUOTE) {
            let end_index = input.rfind(SINGLE_QUOTE);
            let unquote_str = &input[1..end_index.unwrap()];
            Self::FIXEDString(unquote_str.to_string())
        } else if input.chars().all(char::is_numeric) {
            Self::NUM(input.parse::<Num>().unwrap())
        } else {
            let addr = mem.add_var(input);
            Self::Memory(addr)
        }
    }

    pub fn value(&self, mem: &StackMemory) -> Num {
        match self {
            Self::NUM(num) => *num,
            Self::Memory(addr) => mem.get_value(*addr),
            _ => 0,
        }
    }
}

#[derive(Debug)]
enum OpCode {
    UNKNOWN,
    // NOP,
    // Arithmetic Operators
    MOV(MemoryAddr, OpNum),
    INC(MemoryAddr),
    DEC(MemoryAddr),
    ADD(MemoryAddr, OpNum),
    SUB(MemoryAddr, OpNum),
    MUL(MemoryAddr, OpNum),
    DIV(MemoryAddr, OpNum),
    // Branch Prediction
    JMP(JmpLabel),
    CMP(OpNum, OpNum),
    JNE(JmpLabel),
    JE(JmpLabel),
    JGE(JmpLabel),
    JG(JmpLabel),
    JLE(JmpLabel),
    JL(JmpLabel),
    // Function
    CALL(FunctionLabel),
    RET,
    MSG(Vec<OpNum>),
    END,
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect::<String>()
}

fn parse_msg_args(input: &str, mem: &mut StackMemory) -> Vec<OpNum> {
    let mut v: Vec<OpNum> = Vec::new();
    let length = input.len();
    let mut index: usize = 0;
    let mut word_start_index: usize = 0;
    let mut hit_word: bool = false;
    let mut hit_quote: bool = false;

    while index < length {
        // find first char
        // match 3 types
        // find next
        if !hit_word && !hit_quote {
            if input.chars().nth(index).unwrap().is_whitespace() || input.chars().nth(index).unwrap() == COMMA {
                index += 1;
                continue;
            } else if input.chars().nth(index).unwrap() == SINGLE_QUOTE {
                word_start_index = index;
                index += 1;
                hit_quote = true;
                continue;
            } else if input.chars().nth(index).unwrap().is_alphanumeric() {
                word_start_index = index;
                index += 1;
                hit_word = true;
                continue;
            }
        } else if hit_word {
            if input.chars().nth(index).unwrap().is_whitespace() || input.chars().nth(index).unwrap() == COMMA {
                let w = OpNum::new(&input[word_start_index..index], mem);
                v.push(w);
                index += 1;
                hit_word = false;
                continue;
            } else {
                index += 1;
            }
        } else if hit_quote {
            if input.chars().nth(index).unwrap() == SINGLE_QUOTE {
                let w = OpNum::new(&input[word_start_index..index+1], mem);
                v.push(w);
                index += 1;
                hit_quote = false;
                continue;
            } else {
                index += 1;
            }
        }
    }

    if hit_word {
        let w = OpNum::new(&input[word_start_index..index], mem);
        v.push(w);
    }

    v
}

impl OpCode {
    // TODO: jmp label -> jmp address

    pub fn from_string(input: &str, mem: &mut StackMemory) -> Self {

        let cmd: &str;
        let mut arg_str: &str = "";

        // A mnemonic is single key, that activates a menu command in an open menu
        if let Some(mnemonic_index) = input.find(' ') {
            cmd = &input[0..mnemonic_index];
            arg_str = &input[mnemonic_index..input.len()];
        } else {
            cmd = &input[..];
        }

        if cmd == CMD_MSG {
            let op_nums = parse_msg_args(arg_str, mem);
            OpCode::MSG(op_nums)
        } else {
            // trim first, then split
            let arg_string: String = remove_whitespace(arg_str);
            let args : Vec<&str> = arg_string.split(COMMA).collect::<Vec<&str>>();
            match cmd {
                CMD_MOV => {
                    let a_addr = mem.add_var(args[0]);
                    let b = OpNum::new(args[1], mem);
                    OpCode::MOV(a_addr, b)
                }
                CMD_INC => {
                    let a_addr = mem.add_var(args[0]);
                    OpCode::INC(a_addr)
                }
                CMD_DEC => {
                    let a_addr = mem.add_var(args[0]);
                    OpCode::DEC(a_addr)
                }
                CMD_ADD => {
                    let a_addr = mem.add_var(args[0]);
                    let b = OpNum::new(args[1], mem);
                    OpCode::ADD(a_addr, b)
                }
                CMD_SUB => {
                    let a_addr = mem.add_var(args[0]);
                    let b = OpNum::new(args[1], mem);
                    OpCode::SUB(a_addr, b)
                }
                CMD_MUL => {
                    let a_addr = mem.add_var(args[0]);
                    let b = OpNum::new(args[1], mem);
                    OpCode::MUL(a_addr, b)
                }
                CMD_DIV => {
                    let a_addr = mem.add_var(args[0]);
                    let b = OpNum::new(args[1], mem);
                    OpCode::DIV(a_addr, b)
                }
                CMD_JMP => {
                    let a = args[0].to_string();
                    OpCode::JMP(a)
                }
                CMD_CMP => {
                    let a = OpNum::new(args[0], mem);
                    let b = OpNum::new(args[1], mem);
                    OpCode::CMP(a, b)
                }
                CMD_JNE => {
                    let a = args[0].to_string();
                    OpCode::JNE(a)
                }
                CMD_JE => {
                    let a = args[0].to_string();
                    OpCode::JE(a)
                }
                CMD_JGE => {
                    let a = args[0].to_string();
                    OpCode::JGE(a)
                }
                CMD_JG => {
                    let a = args[0].to_string();
                    OpCode::JG(a)
                }
                CMD_JLE => {
                    let a = args[0].to_string();
                    OpCode::JLE(a)
                }
                CMD_JL => {
                    let a = args[0].to_string();
                    OpCode::JL(a)
                }
                CMD_CALL => {
                    let a = args[0].to_string();
                    OpCode::CALL(a)
                }
                _ => OpCode::UNKNOWN
            } // end match
        } // end else
    } // end new
} // end OpCode

// allocate a stack, store variable value
// allocate a HashMap, store variable name and offset on the stack
type SPStack = Vec<usize>;
type LabelMap = HashMap<String, usize>;

trait InterpreterMemory {
    fn new() -> Self;
    fn get_value(&self, addr: MemoryAddr) -> Num;
    fn set_value(&mut self, addr: MemoryAddr, num: Num);
    fn add_var(&mut self, var_name: &str) -> MemoryAddr;
    fn get_address(&self, var_name: &str) -> MemoryAddr;
}
// struct HeapMemory(HashMap<String, Num>);

#[derive(Debug)]
struct StackMemory {
    sp: usize,
    var_map: HashMap<String, usize>,
    mem_stack: Vec<Num>,
}

impl InterpreterMemory for StackMemory {
    fn new() -> StackMemory {
        StackMemory {
            sp: 0,
            var_map: HashMap::new(),
            mem_stack: Vec::new(),
        }
    }
    fn get_value(&self, addr: MemoryAddr) -> Num {
        self.mem_stack[addr]
    }
    fn set_value(&mut self, addr: MemoryAddr, num: Num) {
        self.mem_stack[addr] = num;
    }
    fn add_var(&mut self, var_name: &str) -> MemoryAddr {
        let addr: MemoryAddr;
        let v = self.var_map.get(var_name);
        match v {
            Some(value) => addr = *value,
            None => {
                self.var_map.insert(var_name.to_string(), self.sp);
                self.mem_stack.push(0);
                addr = self.sp;
                self.sp += 1;
            }
        };
        addr
    }
    fn get_address(&self, var_name: &str) -> MemoryAddr {
        *self.var_map.get(var_name).unwrap()
    }
}

#[derive(Debug)]
struct InnerAssemblerInterpreter {
    // stack pointer register
    stack_pointer: usize,
    // flag register
    cmp_register: Ordering,
    // stack, push pop from end
    func_stack: SPStack,
    // label & function
    labels: LabelMap,
    // variable to address
    stack_mem: StackMemory,
    opcodes: Vec<OpCode>,
}

impl InnerAssemblerInterpreter {
    pub fn new() -> Self {
        Self {
            stack_pointer: 0,
            cmp_register: Ordering::Equal,
            func_stack: Vec::new(),
            labels: HashMap::new(),
            stack_mem: StackMemory::new(),
            opcodes: Vec::new(),
        }
    }

    fn lexical_analysis(&mut self, input: &str) {
        let mut stack_index: usize = 0;

        for _line in input.lines() {
            let line = _line.trim();

            if line.is_empty() || line.starts_with(COMMENT_INDICATOR) {
                continue;
            }
            // Label & Function
            if line.ends_with(COLON) {
                let label = line[0..line.len() - 1].to_string();
                self.labels.insert(label, stack_index);
                continue;
            }
            // Return
            if line == CMD_RET {
                self.opcodes.push(OpCode::RET);
                stack_index += 1;
                // self.stack_pointer = self.func_stack.pop().unwrap();
                continue;
            }
            // END
            if line == CMD_END {
                self.opcodes.push(OpCode::END);
                stack_index += 1;
                continue;
            }

            let opcode_str: &str;
            let comment_index = line.find(COMMENT_INDICATOR);
            match comment_index {
                Some(index) => opcode_str = &line[0..index],
                None => opcode_str = &line[..],
            }

            let opcode = OpCode::from_string(opcode_str.trim(), &mut self.stack_mem);
            self.opcodes.push(opcode);
            stack_index += 1;
        }
    }

    fn execute_opcode(&mut self) -> Option<String> {
        let mut output: Option<String> = None;
        // let mut ends_correctly = false;

        loop {
            let t = &self.opcodes.get(self.stack_pointer);
            if t.is_none() {
                output = None;
                break;
            }
            match t.unwrap() {
                OpCode::MOV(a, b) => {
                    let b_value = b.value(&self.stack_mem);
                    self.stack_mem.set_value(*a, b_value);
                    self.stack_pointer += 1;
                }
                OpCode::INC(a) => {
                    let a_value = self.stack_mem.get_value(*a);
                    self.stack_mem.set_value(*a, a_value + 1);
                    self.stack_pointer += 1;
                }
                OpCode::DEC(a) => {
                    let a_value = self.stack_mem.get_value(*a);
                    self.stack_mem.set_value(*a, a_value - 1);
                    self.stack_pointer += 1;
                }
                OpCode::ADD(a, b) => {
                    let a_value = self.stack_mem.get_value(*a);
                    let b_value = b.value(&self.stack_mem);
                    self.stack_mem.set_value(*a, a_value + b_value);
                    self.stack_pointer += 1;
                }
                OpCode::SUB(a, b) => {
                    let a_value = self.stack_mem.get_value(*a);
                    let b_value = b.value(&self.stack_mem);
                    self.stack_mem.set_value(*a, a_value - b_value);
                    self.stack_pointer += 1;
                }
                OpCode::MUL(a, b) => {
                    let a_value = self.stack_mem.get_value(*a);
                    let b_value = b.value(&self.stack_mem);
                    self.stack_mem.set_value(*a, a_value * b_value);
                    self.stack_pointer += 1;
                }
                OpCode::DIV(a, b) => {
                    let a_value = self.stack_mem.get_value(*a);
                    let b_value = b.value(&self.stack_mem);
                    self.stack_mem.set_value(*a, a_value / b_value);
                    self.stack_pointer += 1;
                }
                OpCode::CMP(a, b) => {
                    let a_value = a.value(&self.stack_mem);
                    let b_value = b.value(&self.stack_mem);
                    self.cmp_register = a_value.cmp(&b_value);
                    self.stack_pointer += 1;
                }
                OpCode::JMP(jmp_label) => {
                    self.stack_pointer = *self.labels.get(jmp_label).unwrap();
                    continue;
                }
                OpCode::JNE(jmp_label) => {
                    if self.cmp_register != Ordering::Equal {
                        self.stack_pointer = *self.labels.get(jmp_label).unwrap();
                    } else {
                        self.stack_pointer += 1;
                    }
                }
                OpCode::JE(jmp_label) => {
                    if self.cmp_register == Ordering::Equal {
                        self.stack_pointer = *self.labels.get(jmp_label).unwrap();
                    } else {
                        self.stack_pointer += 1;
                    }
                }
                OpCode::JGE(jmp_label) => {
                    if self.cmp_register == Ordering::Equal || self.cmp_register == Ordering::Greater {
                        self.stack_pointer = *self.labels.get(jmp_label).unwrap();
                    } else {
                        self.stack_pointer += 1;
                    }
                }
                OpCode::JG(jmp_label) => {
                    if self.cmp_register == Ordering::Greater {
                        self.stack_pointer = *self.labels.get(jmp_label).unwrap();
                    } else {
                        self.stack_pointer += 1;
                    }
                }
                OpCode::JLE(jmp_label) => {
                    if self.cmp_register == Ordering::Equal || self.cmp_register == Ordering::Less {
                        self.stack_pointer = *self.labels.get(jmp_label).unwrap();
                    } else {
                        self.stack_pointer += 1;
                    }
                }
                OpCode::JL(jmp_label) => {
                    if self.cmp_register == Ordering::Less {
                        self.stack_pointer = *self.labels.get(jmp_label).unwrap();
                    } else {
                        self.stack_pointer += 1;
                    }
                }
                OpCode::CALL(func_label) => {
                    self.func_stack.push(self.stack_pointer + 1);
                    self.stack_pointer = *self.labels.get(func_label).unwrap();
                }
                OpCode::END => {
                    break;
                }
                OpCode::RET => {
                    self.stack_pointer = self.func_stack.pop().unwrap();
                }
                OpCode::MSG(args) => {
                    let mut s = String::new();
                    for arg in args {
                        match arg {
                            OpNum::NUM(_) | OpNum::Memory(_) => {
                                let v = format!("{}", arg.value(&self.stack_mem));
                                // t.value(&self.stack_mem));
                                s.push_str(&v);
                            }
                            OpNum::FIXEDString(fixed_string) => {
                                s.push_str(fixed_string);
                            }
                        }
                    }
                    output = Some(s);
                    self.stack_pointer += 1;
                }
                _ => {
                    self.stack_pointer += 1;
                }
            }
        }
        output
    }


}

pub struct AssemblerInterpreter {
}

impl AssemblerInterpreter {
    pub fn interpret(input: &str) -> Option<String> {
        let mut inner_interpreter = InnerAssemblerInterpreter::new();
        inner_interpreter.lexical_analysis(input);
        inner_interpreter.execute_opcode()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let simple_programs = &[
            "\n; My first program\nmov  a, 5\ninc  a\ncall function\nmsg  '(5+1)/2 = ', a    ; output message\nend\n\nfunction:\n    div  a, 2\n    ret\n",
            "\nmov   a, 5\nmov   b, a\nmov   c, a\ncall  proc_fact\ncall  print\nend\n\nproc_fact:\n    dec   b\n    mul   c, b\n    cmp   b, 1\n    jne   proc_fact\n    ret\n\nprint:\n    msg   a, '! = ', c ; output text\n    ret\n",
            "\nmov   a, 8            ; value\nmov   b, 0            ; next\nmov   c, 0            ; counter\nmov   d, 0            ; first\nmov   e, 1            ; second\ncall  proc_fib\ncall  print\nend\n\nproc_fib:\n    cmp   c, 2\n    jl    func_0\n    mov   b, d\n    add   b, e\n    mov   d, e\n    mov   e, b\n    inc   c\n    cmp   c, a\n    jle   proc_fib\n    ret\n\nfunc_0:\n    mov   b, c\n    inc   c\n    jmp   proc_fib\n\nprint:\n    msg   'Term ', a, ' of Fibonacci series is: ', b        ; output text\n    ret\n",
            "\nmov   a, 11           ; value1\nmov   b, 3            ; value2\ncall  mod_func\nmsg   'mod(', a, ', ', b, ') = ', d        ; output\nend\n\n; Mod function\nmod_func:\n    mov   c, a        ; temp1\n    div   c, b\n    mul   c, b\n    mov   d, a        ; temp2\n    sub   d, c\n    ret\n",
            "\nmov   a, 81         ; value1\nmov   b, 153        ; value2\ncall  init\ncall  proc_gcd\ncall  print\nend\n\nproc_gcd:\n    cmp   c, d\n    jne   loop\n    ret\n\nloop:\n    cmp   c, d\n    jg    a_bigger\n    jmp   b_bigger\n\na_bigger:\n    sub   c, d\n    jmp   proc_gcd\n\nb_bigger:\n    sub   d, c\n    jmp   proc_gcd\n\ninit:\n    cmp   a, 0\n    jl    a_abs\n    cmp   b, 0\n    jl    b_abs\n    mov   c, a            ; temp1\n    mov   d, b            ; temp2\n    ret\n\na_abs:\n    mul   a, -1\n    jmp   init\n\nb_abs:\n    mul   b, -1\n    jmp   init\n\nprint:\n    msg   'gcd(', a, ', ', b, ') = ', c\n    ret\n",
            "\ncall  func1\ncall  print\nend\n\nfunc1:\n    call  func2\n    ret\n\nfunc2:\n    ret\n\nprint:\n    msg 'This program should return null'\n",
            "\nmov   a, 2            ; value1\nmov   b, 10           ; value2\nmov   c, a            ; temp1\nmov   d, b            ; temp2\ncall  proc_func\ncall  print\nend\n\nproc_func:\n    cmp   d, 1\n    je    continue\n    mul   c, a\n    dec   d\n    call  proc_func\n\ncontinue:\n    ret\n\nprint:\n    msg a, '^', b, ' = ', c\n    ret\n"];

        let expected = &[
            Some(String::from("(5+1)/2 = 3")),
            Some(String::from("5! = 120")),
            Some(String::from("Term 8 of Fibonacci series is: 21")),
            Some(String::from("mod(11, 3) = 2")),
            Some(String::from("gcd(81, 153) = 9")),
            None,
            Some(String::from("2^10 = 1024"))];

        for (prg, exp) in simple_programs.iter().zip(expected) {
            let actual = AssemblerInterpreter::interpret(*prg);
            assert_eq!(actual, *exp);
        }
        assert!(true);
    }
}
