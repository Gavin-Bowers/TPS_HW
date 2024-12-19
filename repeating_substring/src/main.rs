use repeating_substring::prefix_tree;

fn main() {
}

fn repeating_substring(input: &str) -> &str {
    dp_repeating_substring(input)
}

//This has a bug that only shows up on one test case
fn dp_repeating_substring(input: &str) -> &str {
    // Edge case checking (prevents out of bounds)
    let len = input.len(); 
    if len <= 1 {
        return input;
    };

    // Setting up variables
    let mut best: &str = "";
    // let mut rows: Vec<Vec<u8>> = vec![vec![0; len-1]; len-1];
    let mut previous_row: Vec<u8>;
    let mut current_row: Vec<u8> = vec![0; len-1];

    // Iterate over a triangular shape in the matrix
    for r_i in 0..len-1 {

        previous_row = current_row.clone();
        current_row = vec![0; len-1];

        let row_char = &input[r_i..r_i+1];
        let best_len = best.len() as u8;

        for c_i in r_i..len-1 {

            // Bounds check: if either index is zero, their product will be too
            let carried_value: u8 = match r_i * c_i { 
                0 => 0,
                _ => previous_row[c_i-1],
            };

            if &input[c_i+1..c_i+2] == row_char {
                // Add one to the value from top left to represent an existing substring being extended
                current_row[c_i] += 1 + carried_value;
            }
            
            if current_row[c_i] == best_len {
                let substring: &str = &input[c_i+2-current_row[c_i] as usize..c_i+2];
                let mut temp = vec![substring, best];
                temp.sort();
                best = temp[0];
            } else if current_row[c_i] > best_len {
                let substring: &str = &input[c_i+2-current_row[c_i] as usize..c_i+2];
                best = substring;
            }
        }
    }
    return best;
}

fn naive_improved_repeating_substring(input: &str) -> &str {
    // This version is the naive solution with a couple of optimizations that save a lot of time for most test cases, but still aren't enough for Kattis
    let mut best: &str = input;
    for i in 0..input.len() { //each start
        for j in i+1..input.len() { //each end
            if (j - i) >= best.len() { // If the substring is shorter than our current best, it's not worth checking
                let candidate = &input[i..j];
                let mut stop = true;
                for k in i+1..input.len() {
                    if k+candidate.len() > input.len() { // If we go out of bounds looking for repeat, break
                        break;
                    }
                    let other = &input[k..k+candidate.len()];
                    if candidate == other {
                        stop = false;
                        if candidate.len() > best.len() { 
                            best = candidate;
                        }
                        if candidate.len() == best.len() { // Break ties alphanumerically
                            let mut temp = vec![candidate, &best];
                            temp.sort();
                            best = temp[0];
                        }
                    }
                }
                if stop { // This breaks the loop that checks longer and longer substrings starting at a specific index
                    break; // If the substring has no repetition, longer substrings with the same start also won't
                }
            }
        }
    }
    best
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(repeating_substring("abcabc"), "abc");
    }

    #[test]
    fn test_2() {
        assert_eq!(repeating_substring("dabcabcaf"), "abca");
    }

    #[test]
    fn test_3() {
        assert_eq!(repeating_substring(""), "");
    }

    #[test]
    fn test_4() {
        assert_eq!(repeating_substring("aaaa"), "aaa");
    }

    #[test]
    fn test_5() {
        assert_eq!(repeating_substring("bbcaadbbeaa"), "aa");
    }

    #[test]
    fn test_6() {
        assert_eq!(repeating_substring("weoifhsduifghireyhgoiedrgiuwetuiwseyhgiuefhiuwejffffgvffffffffwoeijfwpoejfowehtg9wiouretguiretguyguiytedygiuygjdvnkreyvhnuiterty 7irethgiuefhtgvyjetrthuyrtvhyetuctntjehtg rutg yhureyhgure bytuiyerghjyuregfhrejyvtu utrgyjhretygureyguyi5rehtiuredhygi8vdfti afroweh tgiourehgiurwehiugreuigreu reigfherigireugiergiergiewfhireuh reig hreuighreiughiregiergh reighireghireughiuregh iregh ireghireghiurerhgiershgirehgirehurehgireigrehgiregihreigherighreiughreiugerigreiuguiryeghuyrehgierghuyrehguirehihgirehgiureahgi ygiehgirehgiuraehgierhgiahgirehgierhgiregiurehgiabgierhgirehgierhgirehgireghiuaerht8retyibnre"), "hgirehgierhgire");
    }
}
