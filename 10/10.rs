// Advent of code Day 10
// Trying out Rust for the 4th time


fn check_if_overlap(cycle: i32, register_x: i32) -> bool {
    let pos = cycle % 40;
    // println!("Cycle: {}, X: {}, pos: {}", cycle, register_x, pos);

    return pos == register_x || pos+1 == register_x || pos-1 == register_x; 
}

fn main() {
    let input: &str = 
"addx 2
addx 3
addx 1
noop
addx 4
noop
noop
noop
addx 5
noop
addx 1
addx 4
addx -2
addx 3
addx 5
addx -1
addx 5
addx 3
addx -2
addx 4
noop
noop
noop
addx -27
addx -5
addx 2
addx -7
addx 3
addx 7
addx 5
addx 2
addx 5
noop
noop
addx -2
noop
addx 3
addx 2
addx 5
addx 2
addx 3
noop
addx 2
addx -29
addx 30
addx -26
addx -10
noop
addx 5
noop
addx 18
addx -13
noop
noop
addx 5
noop
noop
addx 5
noop
noop
noop
addx 1
addx 2
addx 7
noop
noop
addx 3
noop
addx 2
addx 3
noop
addx -37
noop
addx 16
addx -12
addx 29
addx -16
addx -10
addx 5
addx 2
addx -11
addx 11
addx 3
addx 5
addx 2
addx 2
addx -1
addx 2
addx 5
addx 2
noop
noop
noop
addx -37
noop
addx 17
addx -10
addx -2
noop
addx 7
addx 3
noop
addx 2
addx -10
addx 22
addx -9
addx 5
addx 2
addx -5
addx 6
addx 2
addx 5
addx 2
addx -28
addx -7
noop
noop
addx 1
addx 4
addx 17
addx -12
noop
noop
noop
noop
addx 5
addx 6
noop
addx -1
addx -17
addx 18
noop
addx 5
noop
noop
noop
addx 5
addx 4
addx -2
noop
noop
noop
noop
noop";

    let mut cycle = 0;
    let mut register_x = 1;
    // let mut total_strength = 0;
    // let cycles = [20, 60, 100, 140, 180, 220];
    let mut state = vec![0; 40 * 6];


    for instruction in input.lines(){
        let mut seperated = instruction.split_whitespace();

        match seperated.next() {
            Some("addx") => {
                let number = seperated.next().unwrap().parse::<i32>().unwrap();
                // println!("Add x number: {}, cycle count: {}", number, cycle);
                cycle = cycle + 1;
                
                // if cycles.contains(&cycle) {
                //     println!("Cycle: {}, register: {}", cycle, register_x);
                //     total_strength = total_strength + cycle * register_x;
                // }
                if check_if_overlap(cycle - 1, register_x){
                    state[(cycle - 1) as usize] = 1;
                } else{
                    state[(cycle - 1) as usize] = 0;
                }


                cycle = cycle + 1;

                // if cycles.contains(&cycle) {
                //     println!("Cycle: {}, register: {}", cycle, register_x);
                //     total_strength = total_strength + (cycle * register_x);
                // }

                if check_if_overlap(cycle - 1, register_x){
                    state[(cycle - 1) as usize] = 1;
                } else{
                    state[(cycle - 1) as usize] = 0;
                }

                register_x = register_x + number;

    
            },
            Some("noop") => {
                cycle = cycle + 1;
                // if cycles.contains(&cycle) {
                //     println!("Cycle: {}, register: {}", cycle, register_x);
                //     total_strength = total_strength + (cycle * register_x);
                // }
                if check_if_overlap(cycle - 1, register_x){
                    state[(cycle - 1) as usize] = 1;
                } else{
                    state[(cycle - 1) as usize] = 0;
                }
            },
            _ => {}
        };
    }
    for i in 0..40*6 {
        if i % 40 == 0 {
            println!("");
        }

        if state[i] == 1 {
            print!("#");
        }else{
            print!(".");
        }
    }
    // println!("{}", total_strength);

}

