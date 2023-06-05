fn main() {
    println!("hello");

    let mut money = 10000;
    let mut pop_level = 1;
    let mut ecology_level = 5;

    loop {
        println!("money:{}, pop_level:{}, ecology_level:{}", money, pop_level, ecology_level);
        if has_won(money, pop_level, ecology_level) {
            println!("Bravo !");
        return;
        }
        if has_lost(money, pop_level, ecology_level){
            println!("Looser! Not bravo");
            return;
        }
        next_step(&mut money, &mut pop_level, &mut ecology_level);
    }
}

fn next_step(money: &mut i32, pop_level: &mut i32, ecology_level: &mut i32) {
    if *pop_level != 0 && *pop_level < 12 {
        *pop_level = *pop_level + 1;
    }

    if *pop_level == 12 && *ecology_level > 0 {
        *ecology_level = *ecology_level - 1;
    }

    if *ecology_level == 0 && *pop_level > 0 {
        *pop_level = *pop_level - 1;
    }
}

fn has_lost(money: i32, pop_level: i32, ecology_level:  i32) -> bool {
    return money <= 0 ||
        pop_level == 0;
}


fn has_won(money: i32, pop_level: i32, ecology_level: i32) -> bool {
    pop_level == 8
}