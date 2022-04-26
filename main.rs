#![feature(int_log)]

/// A quick tool to generate a matrix of floating point features from a given bit magnitude (input 32 = 32 bits), Sign bit or naw, exponent magnitude (8 = 8 bits = 64 values). It will return a string of values and details of the floating point number.

//Rules:
// a Base/Radix(usually binary or decimal) will be specified
// a number of exponent bits will be specified
// A sign bit may be specified (IEEE 754 Requires signed Floating point types)

// the Exponent Max and Min will be calculated
// a Precision must be calculated
// a number for mantissa bits will be calcualted
// The maximum value and precisions at each exponent will be calculated
// a "zero" minimum value will be calculated (zero can't be hit so how close can we get?)
// 
//(...) A sign bit, followed by `w` exponent bits that describe the exponent offset by a bias, and `p − 1` bits that describe the mantissa. The width of the exponent field for a k-bit format is computed as `w = round(4 log2(k)) − 13`. The existing 64 and 128-bit formats follow this rule, but the 16 and 32-bit formats have more exponent bits (5 and 8 respectively) than this formula would provide (3 and 7 respectively). (https://en.wikipedia.org/wiki/IEEE_754#Binary)

const description: &str = "A quick tool to generate a matrix of floating point features from a given bit magnitude (input 32 = 32 bits), Sign bit or naw, exponent magnitude (8 = 8 bits = 64 values). It will return a string of values and details of the floating point number.";


fn main() 
{
    //Input Variables
    let mut base: u64 = 0;
    let mut size: u64 = 0;
    let mut exponent: u64 = 0;
    let mut sign: Option<bool> = None;

    //Also Input Variables but not used for calculation
    let mut input_validated = false;

    //Input & validation stage
    while input_validated == false {
        while base == 0 {
            println!("Base/Radix, Binary (2) or Decimal (10) ");
            
        }

        if sign.is_none() {
            println!("Would you like this Float to be signed? ");
        }

        while size == 0 {
            println!("Size in bits of the Float: ");
        }
        
        if size > 0 {
            println!("Bit layout");
            print!("\n(1) - Explicit Exponent Count\n(2) - Range Generated\n(3) - Precision Generated");
            1 => {
                println!("Explicit Exponent Selected: How many Bits do you wish to use? ");
            }
            2 => {
                println!("Range Generation Selected: How large does the range need to be? ");
                    let range_width = buf_string.parse::<u32>().unwrap();
                    exponent = range_width.log2() as u64; //TODO:Make a log2 that can be used for u64
            }
            3 => {
                println!("Precision Generation Selected: How many digits of precision do you need? ");
                let requested_precision: u64;
                let mut remainder: u128 = 2^requested_precision as u128; // makes me wish for long longs
                let mut mantissa: u64 = 0;
                while remainder > 10 {
                    remainder = remainder / 10; // Precision is measured in decimal, fight me.
                    mantissa += 1;
                exponent = size - mantissa;
                }
            }
            _ => { println!("Invalid Input, Pick one of the above stated options"); }
        }
    }



        //Validation
        if size > 0 && base > 0 && exponent > 0 && sign.is_some() { //Checks for valid inputs
            if (sign.is_some() && sign.unwrap() == true) && (size > exponent + 1) {
                input_validated = true;
            }
            else if size > exponent { //All calculations are still valid if its just exponent bits, it's just not very useful.
                input_validated = true;
            }
            else {
                let mut switcher: bool = false;
                while switcher {
                    println!("Size validation failed, Exponent bitfield was larger than total size for the type");
                    println!("Would you like to try a new Type Size or a new Exponent?(0 for Size, 1 for Exponent");
                    switcher = true;
                }
            }
        }
    //Calculate float definition

    //Calculate float characteristics

    //Return Float Def and Characteristics

}

struct Range(i128, u128); // Lower, Upper
fn total_range(exponent: u64, base: u64) -> Range { Range(-((base as i128)^((exponent-1) as i128) ), (base^exponent) as u128) }

fn range(exponent: u64, base: u64, sign: bool) -> Vec<Range>
{
    let mut result: Vec<Range> = Vec::new();
    for work_exp in (0..exponent) {
        result.push(Range(-((base as i128)^((work_exp-1) as i128) ), (base^work_exp) as u128));
    }
    result
}

fn precision(exponent: u64, base: u64) -> Vec<u32>
{
    let mut result: Vec<u32> = Vec::new();
    for work_exp in (0..exponent) {
        result.push((base ^ work_exp).log10());
    }
    result
}

