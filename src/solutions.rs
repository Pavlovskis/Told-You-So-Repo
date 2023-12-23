#[allow(dead_code)]
pub mod solutions {
    pub fn queue_sort(v:Vec<i32>) -> Option<usize> {
        let mut min:i32 = i32::MAX;
        let mut pos:usize = 0;
        for i in 0..v.len() {
            if v[i] < min {
                min = v[i];
                pos = i
            }
        }
        for i in pos..v.len() - 1 {
            if v[i] > v[i + 1] {
                return None;
            }
        }
        Some(pos)
    }

    pub fn quingshan(s:&str, t:&str) -> bool {
        if s.len() == 1 && s != t { return true; }

        let mut prevt:char = ' ';
        for c in t.char_indices() {
            if prevt == c.1 { return false; }
            prevt = c.1;
        }

        let mut prevs:char = ' ';
        for c in s.char_indices() {
            if prevs == c.1 {               
                if &t[..1] == prevs.to_string() {
                    return false;
                }                    
                if c.1.to_string() == &t[t.len()-1..] {
                    return false;
                }
            } 
            prevs = c.1;
        }
        true
    }

    pub fn eraser_1d(s:&str, e:usize) -> usize {
        let mut count:usize = 0;

        let c:Vec<char> = s.chars().collect();
        let mut i:usize = 0;
        while i < c.len() {
            if c[i] == 'B' {
                count += 1;
                if i + e > s.len() {
                    break;
                }else { i += e-1; }
            }
            i += 1;
        }
        count   
    }

    //1869B
    pub fn traveling_2d(c:Vec<(i32,i32)>, maj:usize, s:usize, e:usize) -> usize {                
        pub fn travel(c:&Vec<(i32,i32)>, maj:usize, s:usize, e:usize, price:usize) -> usize {
            if c[e] == c[s] { return price; }

            let mut min_price:usize = usize::MAX;
            let mut pos:usize = 0;
            for i in c.into_iter().enumerate() {
                if i.1 == &c[s] { continue; }
                else {
                    let cand_min:usize = ((c[s].0 - i.1.0).abs() + (&c[s].1 - i.1.1).abs()) as usize;
                    
                    if cand_min <= min_price {
                        min_price = cand_min;
                        pos = i.0;
                    }
                }
                
            }

            return travel(&c, maj, pos, e, price + min_price);
        }

        let price = travel(&c, maj, s, e, 0);
        price
    }

    pub fn yarik_array(v:&Vec<i32>) -> i32 {
        pub fn is_even(n:i32) -> bool {
            if n % 2 == 0 { return true}
            else { return false }
        }
        pub fn odd_even_pair(m:i32, n:i32) -> bool {
            if (is_even(m) && is_even(n)) || (!is_even(m) && !is_even(n)) { return true; }
            else { return false; }
        }
        let mut sum:i32 = 0;

        let mut l:usize = 0;
        let mut r:usize = v.len();
        while l < r {
            let mut cand_sum:i32 = 0;
            for i in l..r {
                cand_sum += v[i];
                if odd_even_pair(v[i], v[i+1]) {
                    l = i + 1;
                    sum = sum.max(cand_sum);
                }
            }
            println!();
            for j in (l..r).rev() {
            }
            break;
        }

        sum
    }

    pub fn chip_ribbon(v:Vec<u32>) {
        
    }

}

#[cfg(test)]
mod tests {
    use crate::solutions::solutions::{queue_sort, quingshan, eraser_1d, traveling_2d, yarik_array};

    #[test]
    fn test1() {
        assert_eq!(yarik_array(&vec![1,2,3,4,5]), 15);
    }
    #[test]
    fn test2() {
        assert_eq!(eraser_1d("WWBWBWW", 3), 1);
    }
    #[test]
    fn test3() {
        assert_eq!(eraser_1d("BWBWB", 4), 2);
    }
    #[test]
    fn test4() {
        assert_eq!(eraser_1d("BBBBB", 5), 1);
    }
    #[test]
    fn test5() {
        assert_eq!(eraser_1d("BWBWBBBB", 2), 4);
    }
    // #[test]
    // fn test6() {
    //     assert_eq!(eraser_1d("WBBWBBWBBW", 2), 3);
    // }
    // #[test]
    // fn test7() {
    //     assert_eq!(eraser_1d("BBBB", 1), 4);
    // }
    // #[test]
    // fn test8() {
    //     assert_eq!(eraser_1d("WWW", 2), 0);
    // }
}