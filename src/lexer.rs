pub struct Lexer {}

impl Lexer {
    pub fn lexer(input: &str) -> i32 {
        let mut line = 1;
        let mut result = 0;

        for c in input.chars() {
            match c {
                '(' => println!("LEFT_PAREN ( null"),
                ')' => println!("RIGHT_PAREN ) null"),
                '{' => println!("LEFT_BRACE {{ null"),
                '}' => println!("RIGHT_BRACE }} null"),
                ',' => println!("COMMA , null"),
                '.' => println!("DOT . null"),
                '-' => println!("MINUS - null"),
                '+' => println!("PLUS + null"),
                ';' => println!("SEMICOLON ; null"),
                '*' => println!("STAR * null"),
                '\n' => line += 1,
                _ => {
                    result = 65;
                    println!("[line {}] Error: Unexpected character: {}", line, c);
                }
            }
        }
        println!("EOF  null");

        result
    }
}
