use std::{collections::HashMap};

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut code_ptr = 0;
    let mut input_ptr = 0;
    let mut tape = HashMap::<usize, u8>::new();
    let mut tape_ptr: usize = 0;
    let mut output = Vec::<u8>::new();
    while let Some(instruction) = code.bytes().nth(code_ptr) {
        let tape_val = match tape.get(&tape_ptr) {
            Some(x) => *x,
            None => 0
        };
        match instruction {
            b'>' => tape_ptr += 1,
            b'<' => tape_ptr -= 1,
            b'+' => {
                tape.insert(tape_ptr, (tape_val).overflowing_add(1).0);
            },
            b'-' => {
                tape.insert(tape_ptr, (tape_val).overflowing_sub(1).0);
            }
            b',' => { tape.insert(tape_ptr, input[input_ptr]); input_ptr += 1 },
            b'.' => { output.push(tape_val) },
            b'[' if tape_val == 0 => { 
                let mut opened = 1;
                while !(code.bytes().nth(code_ptr) == Some(b']') && opened == 0) {
                    code_ptr += 1;
                    if code.bytes().nth(code_ptr) == Some(b'[') { opened += 1 }
                    if code.bytes().nth(code_ptr) == Some(b']') { opened -= 1 }
                }
             },
             b']' if tape_val != 0 => { 
                let mut opened = 1;
                while !(code.bytes().nth(code_ptr) == Some(b'[') && opened == 0) {
                    code_ptr -= 1;
                    if code.bytes().nth(code_ptr) == Some(b']') { opened += 1 }
                    if code.bytes().nth(code_ptr) == Some(b'[') { opened -= 1 }
                }
             },
            _ => {}
        }
        code_ptr += 1;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_tests() {
        // Echo until byte 255 encountered
        assert_eq!(
            String::from_utf8(brain_luck(",+[-.,+]", ez_vec("Codewars", 255))).unwrap(),
            "Codewars"
        );
        // Echo until byte 0 encountered
        assert_eq!(
            String::from_utf8(brain_luck(",[.[-],]", ez_vec("Codewars", 0))).unwrap(),
            "Codewars"
        );
        // Multiply two numbers
        assert_eq!(
            brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]),
            vec![72]
        );
        assert_eq!(
            brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![0, 4]),
            vec![0]
        );
    }

    // Takes a static string and a terminating byte and returns an owned Vec<u8> for convenience
    // Without it, character-based tests are a pain
    fn ez_vec(s: &str, i: u8) -> Vec<u8> {
        let mut v = s.to_string().into_bytes();
        v.push(i);
        v
    }
}
