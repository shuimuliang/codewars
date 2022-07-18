// http://ref.x86asm.net/coder32.html
// enum OpCode {
//     MOV,
//     INC,
//     DEC,
//     ADD,
//     SUB,
//     MUL,
//     DIV,
//     JMP,
//     CMP,
//     JNE,
//     JE,
//     JGE,
//     JG,
//     JLE,
//     JL,
//     CALL,
//     RET,
//     END,
//     COMMENT
// // label:
// }

// memory.store(self.sp--, value);
// var value = memory.load(++self.sp);

use itertools::Itertools;

fn main() {

    let perms = (5..8).permutations(2);
    itertools::assert_equal(perms, vec![
        vec![5, 6],
        vec![5, 7],
        vec![6, 5],
        vec![6, 7],
        vec![7, 5],
        vec![7, 6],
    ]);
}