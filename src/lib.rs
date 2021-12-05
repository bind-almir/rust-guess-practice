use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    let secret = generage_random_number();
    loop {
        let input = get_input(false, "");
        println!("You guessed {}", input);
        match input.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn get_input(is_test: bool, fixture: &str) -> u32 {
    let mut input = String::new();
    loop {
        if is_test {
            input = String::from(fixture);
        } else {
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
        }
        let res = handle_input(&input);
        if res == 0 {
            continue;
        }
        return res;
    }
}

fn handle_input(input: &str) -> u32 {
    match input.trim().parse::<u32>() {
        Ok(num) => {
            if !(1..=100).contains(&num){
                println!("Number must be between 1 and 100");
                return 0
            }
            num
        },
        Err(_) => 0,
    }
}

fn generage_random_number() -> u32 {
    rand::thread_rng().gen_range(1..101)
}

#[cfg(test)]

mod test {
    use super::get_input;
    use super::handle_input;
    #[test]
    fn test_get_input() {
        assert_eq!(get_input(true, "5"), 5);
        assert_eq!(get_input(true, "10"), 10);
        assert_eq!(get_input(true, "20"), 20);
    }

    #[test]
    fn test_handle_input() {
        assert_eq!(handle_input("asd5"), 0);
        assert_eq!(handle_input("10"), 10);
        assert_eq!(handle_input("20"), 20);
        assert_eq!(handle_input("30"), 30);
        assert_eq!(handle_input(""), 0);
        assert_eq!(handle_input("-1"), 0);
        assert_eq!(handle_input("103"), 0);
        assert_eq!(handle_input("-103"), 0);
        assert_eq!(handle_input("-103"), 0);
        assert_eq!(handle_input("!!!"), 0);
    }


}