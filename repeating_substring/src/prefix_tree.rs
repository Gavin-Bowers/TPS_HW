pub mod prefix_tree {
    use std::collections::HashMap;

    pub struct Trie {
        root: Node,
    }

    struct Node {
        children: HashMap<char, Node>,
        indices: Vec<usize>,
    }

    impl Node {
        fn insert_child(&mut self, suffix: &str, start_index: usize) {

            if suffix.len() == 0 { return; }
            let c = suffix.chars().next().unwrap();
            
            if !self.children.contains_key(&c) {
                self.children.insert(c, Node {children: HashMap::new(), indices: Vec::new()});
            }
            let n = self.children.get_mut(&c).unwrap();
            n.insert_child(&suffix[1..], start_index);
            n.indices.push(start_index);
        }
    }

    impl Trie {

        pub fn find_longest_repeating_substring(input: &str) -> String {
            
            let trie = Trie::construct(input);
            return Trie::dfs(&trie.root, "".to_string());
        }

        fn construct(input: &str) -> Trie {
            let mut prefix_tree = Trie{root: Node{children: HashMap::new(), indices: Vec::new()}};

            for i in 0..input.len() {
                prefix_tree.root.insert_child(&input[i..], i);
            }

            return prefix_tree;
        }

        // Recursive depth first search to find LRS
        fn dfs(node: &Node, prefix: String) -> String {
            let mut longest =  if node.indices.len() >= 2 { prefix.clone() } else { "".to_string() };
            for (char, child) in node.children.iter() {
                if child.indices.len() >= 2 {
                    let mut extended_prefix = prefix.clone();
                    extended_prefix.push(*char);

                    let candidate = Self::dfs(child, extended_prefix);

                    if candidate.len() == longest.len() && candidate < longest {
                        longest = candidate;
                    } else if candidate.len() > longest.len() {
                        longest = candidate;
                    }
                }
            }
            return longest;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prefix_tree::prefix_tree::Trie;

    fn repeating_substring(input: &str) -> String {
        Trie::find_longest_repeating_substring(input)
    }

    #[test]
    fn test_1() {
        assert_eq!(repeating_substring("cccaaa"), "aa");
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