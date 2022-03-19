use regex::Regex;
use std::cmp;
use math::round;

use std::io::{stdin,stdout,Write};

fn main() {

    fn add(x: Vec<i32>, y: Vec<i32>, base: i32) -> Vec<i32> {
        let mut z = vec![];
        let n = cmp::max(x.len(), y.len());
        //println!("{:?}", x);
        let mut carry = 0;
        let mut i = 0;

        let mut xi;
        let mut yi;

        while i < n || carry != 0{
            if i < x.len() {
                xi = x[i];
            }
            else {
                xi = 0;
            }

            if i < y.len() {
                yi = y[i];
            }
            else {
                yi = 0;
            }

            let zi = carry + xi + yi;
            z.push(zi % base);
            carry = round::floor(zi as f64 / base as f64, 0) as i32;
            i += 1;

        }
        return z;
    }

    fn multiply_by_number(mut num: i32, x: Vec<i32>, base: i32) -> Vec<i32> {
        if num < 0 { return vec![]; }
        if num == 0 { return vec![]; }
        let mut result= vec![];

        let mut power = x;
        loop {
            if (num & 1) != 0 {
                result = add(result, power.to_vec(), base);

            }

            num = num >> 1;
            if num == 0 { break }

            power = add(power.to_vec(), power.to_vec(), base);
        }
        return result;
    }


    fn parse_to_digits(value: &str, base: i32) -> Vec<i32> {
        let digits = value.split("");
        let digits_vec: Vec<&str> = digits.collect();

        let mut ary: Vec<i32> = vec![];

        for i in (1..digits_vec.len() - 1).rev() {
            ary.push(digits_vec[i].parse::<i32>().unwrap());
        }

        return ary;
    }


    fn convert_base(value: &str, from: i32, to: i32) -> String {
        let digits: Vec<i32> = parse_to_digits(value, from);
        if digits.is_empty() {
            return "".to_owned();
        }
        else {

            //println!("{:?}", digits);
            let mut out_array: Vec<i32> = vec![];
            let mut power = vec![1];


            for i in 0..digits.len() {

                if (digits[i]) != 0 {
                    out_array = add(out_array, multiply_by_number(digits[i], power.to_vec(), to), to);
                }

                power = multiply_by_number(from, power.to_vec(), to);
            }

            let mut out = "".to_owned();
            for i in (0..out_array.len()).rev() {
                let converted = format!("{:x}", out_array[i]);
                out.push_str(&*converted.to_string());
            }

            return out;
        }
    }


    fn int_to_card(value: &str) -> String {

        let value = convert_base(value, 10, 16);
        let mut my_captures: Vec<&str> = Regex::new(r"\S{1,2}") //  \{\S+\}"
            .unwrap()
            .find_iter(&value)
            .map(|x| x.as_str())
            .collect();

        my_captures.reverse();
        return my_captures.join("");
    }

    let mut s=String::new();

    print!("Please enter the card id: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    println!("output: {:?}", int_to_card(&s));
}
