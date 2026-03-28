
struct Solution;



struct  TrieNode{ 
    children : [Option<Box<TrieNode>>; 26],
}

impl TrieNode {
    fn new() -> Self { 
        TrieNode { children: std::array::from_fn(|_| None) }
    }
}

struct Trie { 
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie { root: TrieNode::new() }
    }

    fn insert(&mut self , word :&str) { 
        let mut node = &mut self.root;
        for c in word.bytes() { 
            let i = (c - b'a') as usize;
            node = node.children[i].get_or_insert_with(||
                Box::new(TrieNode::new()));
        }
    }

       fn max_prefix_len(&self, s: &[u8], start: usize) -> usize {

          let mut node = &self.root;
          let mut len = 0;
          for &c in &s[start..] {
              let i = (c - b'a') as usize;
              match &node.children[i] {
                  Some(next) => { node = next; len += 1; }
                  None => break,
              }
          }
          len
        }
  }



impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {

        let mut trie = Trie::new();
    
        for w in &words { 
            trie.insert(w);
        } 
        
        let n =  target.len();
        let mut dp = vec![i32::MAX; n+1];

        dp[0] = 0; 

        let  target_bytes = target.as_bytes();

        for i in 0..n { 
            if dp[i] == i32::MAX { 
                continue;
            }
            let ll = trie.max_prefix_len(target_bytes, i);
            for l in 1..=ll { 
                dp[i+l] = dp[i + l].min(dp[i]+1);
            }
        }

        if dp[n] == i32::MAX { 
            return -1;
        }
        dp[n]
    }
}
