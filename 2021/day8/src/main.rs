use bitvec::prelude::*;

fn main() {
    let input = include_str!("../input.txt");
    let entries: Vec<([BitArray; 10], [BitArray; 4])> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (digits, output) = line.split_once(" | ").unwrap();
            let digits: Vec<BitArray> = digits
                .split(' ')
                .map(|word| {
                    let mut char_map = bitarr![0; 7];
                    for ch in word.chars() {
                        char_map.set(ch as usize - 'a' as usize, true);
                    }
                    char_map
                })
                .collect();

            let output: Vec<BitArray> = output
                .split(' ')
                .map(|word| {
                    let mut char_map = bitarr![0; 7];
                    for ch in word.chars() {
                        char_map.set(ch as usize - 'a' as usize, true);
                    }
                    char_map
                })
                .collect();

            (digits.try_into().unwrap(), output.try_into().unwrap())
        })
        .collect();

    // Part 1
    let sum: usize = entries
        .iter()
        .map(|(_, outputs)| {
            outputs
                .iter()
                .map(|output| output.count_ones())
                .filter(|count| match *count {
                    2 => true,
                    3 => true,
                    4 => true,
                    7 => true,
                    _ => false,
                })
                .count()
        })
        .sum();

    println!("Sum: {}", sum);

    // Part 2
    let result: usize = entries.iter().map(|entry| process_entry(entry)).sum();
    println!("Result: {}", result);
}

fn process_entry(entry: &([BitArray; 10], [BitArray; 4])) -> usize {
    let (digits, inputs) = entry;
    let one = find_with_count(digits, digits, 2);
    let four = find_with_count(digits, digits, 4);
    let seven = find_with_count(digits, digits, 3);
    let eight = find_with_count(digits, digits, 7);

    let a = one ^ seven;
    assert!(a.count_ones() == 1);

    let nine = find_with_count(
        digits,
        &digits.map(|x| {
            if x == four {
                bitarr![0; 7]
            } else {
                x ^ (four | a)
            }
        }),
        1,
    );
    assert!(nine.count_ones() == 6);

    let g = nine ^ (four | a);
    assert!(g.count_ones() == 1);

    let e = eight ^ nine;
    assert!(e.count_ones() == 1);

    let zero = find_with_count(digits, &digits.map(|x| x ^ (seven | e | g)), 1);
    assert!(zero.count_ones() == 6);

    let b = zero ^ seven ^ e ^ g;
    assert!(b.count_ones() == 1);

    let three = eight ^ b ^ e;
    assert!(three.count_ones() == 5);

    let d = three ^ (one | a | g);
    assert!(d.count_ones() == 1);

    let five = find_with_count(digits, &digits.map(|x| x ^ (nine ^ one)), 1);
    assert!(five.count_ones() == 5);

    let c = five ^ nine;
    assert!(c.count_ones() == 1);
    let f = one ^ c;
    assert!(f.count_ones() == 1);

    let two = a ^ c ^ d ^ e ^ g;
    let six = five ^ e;
    let zero = eight ^ d;

    let numbers: [BitArray; 10] = [zero, one, two, three, four, five, six, seven, eight, nine];
    inputs
        .map(|input| find_match(&numbers, input))
        .iter()
        .fold(0, |acc, val| acc * 10 + val)
}

fn find_match(numbers: &[BitArray; 10], input: BitArray) -> usize {
    for (value, bit_repr) in numbers.iter().enumerate() {
        if input == *bit_repr {
            return value;
        }
    }
    println!("input: {:#?}", input);
    println!("numbers: {:#?}", numbers);
    unreachable!("should have returned one");
}

fn find_with_count(
    original_src: &[BitArray; 10],
    digits: &[BitArray; 10],
    count: usize,
) -> BitArray {
    for (index, digit) in digits.iter().enumerate() {
        if digit.count_ones() == count {
            return original_src[index];
        }
    }
    unreachable!("should have returned one");
}
