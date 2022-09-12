use std::{collections::HashMap, char};

pub fn roman_to_int(rom: String) -> i32 {
    /* 
        Given a roman numeral, convert it to an integer.

        Not the prettiest code, but it works. But how to oranize it better?
    */


    
    let romans = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let rom_vec: Vec<char> = rom.chars().collect();

    let mut i = 0;
    let mut num = 0;

    while i < rom_vec.len() {
        let value = romans.get(&rom_vec[i]);
        let next_value_option = rom_vec.get(i+1);
        let next_value: Option<&i32>;

        match next_value_option {
            Some(char) => {
                next_value = romans.get(char)
            }
            None => {
                next_value = None;
            }
        }

        match value {
            Some(int) => {
                match next_value {
                    Some(next_int) => {
                        if next_int > int {
                            num = num + (next_int - int);
                            i = i + 2;
                        } else {
                            num = num + int;
                            i = i + 1;
                        }
                    },
                    None => {
                        num = num + int;
                        i = i + 1
                    }
                }
            },
            _ => i = i+1
        }
    }
    num
}