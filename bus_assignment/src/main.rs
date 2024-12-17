fn main() {
    println!("Hello, world!");
}

fn find_min_seats(n: u32, a_and_b: Vec<(u32, u32)>) -> u32 {
    let mut current_passengers = 0_u32;
    let mut max_passengers = 0_u32;
    for (a, b) in a_and_b {
        current_passengers -= a;
        current_passengers += b;
        if current_passengers > max_passengers {
            max_passengers = current_passengers;
        }
    }
    max_passengers
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(24, 
            find_min_seats(
                0, vec![
                (0,5),
                (2,1),
                (4,1),
                (1,8),
                (5,15),
                (0,6),
                (24,0)]
            )
        );
    }
}
