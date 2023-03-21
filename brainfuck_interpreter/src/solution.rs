pub fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let (mut buffer, mut buf_pointer): (Vec<u8>, usize) = (vec![0; 5_000], 0);
    let (mut input_idx, mut code_idx): (usize, usize) = (0, 0);

    let mut loops: Vec<usize> = vec![];
    let mut output: Vec<u8> = vec![];
    loop {
        if code_idx >= code.len() {
            break;
        }

        let ch = code.chars().nth(code_idx).unwrap();
        match ch {
            '>' => buf_pointer += 1,
            '<' => buf_pointer -= 1,
            '+' => {
                if buffer[buf_pointer] == 255 {
                    buffer[buf_pointer] = 0
                } else {
                    buffer[buf_pointer] += 1
                }
            }
            '-' => {
                if buffer[buf_pointer] == 0 {
                    buffer[buf_pointer] = 255
                } else {
                    buffer[buf_pointer] -= 1
                }
            }
            '.' => output.push(buffer[buf_pointer]),
            ',' => {
                if input_idx >= input.len() {
                    break;
                }

                buffer[buf_pointer] = input[input_idx];
                input_idx += 1
            }
            '[' => {
                if buffer[buf_pointer] == 0 {
                    code_idx += 1;

                    let mut bracket_counter: usize = 1;
                    loop {
                        if bracket_counter == 0 {
                            break;
                        }

                        let next_ch = code.chars().nth(code_idx).unwrap();
                        match next_ch {
                            '[' => bracket_counter += 1,
                            ']' => bracket_counter -= 1,
                            _ => {}
                        }
                        code_idx += 1;
                    }

                    continue;
                }
                loops.push(code_idx);
            }
            ']' => {
                if buffer[buf_pointer] == 0 {
                    loops.remove(loops.len() - 1);
                    code_idx += 1;
                    continue;
                }

                let corresponding_opening = loops.last().unwrap();
                code_idx = *corresponding_opening;
            }
            _ => {}
        }
        code_idx += 1
    }

    output
}
