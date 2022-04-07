struct Solution;

extern "C" {
    fn linked_list_cycle() -> bool;
}

impl Solution {
    fn has_cycle() -> bool {
        unsafe { linked_list_cycle() }
    }
}

fn main() {
    println!("{}", Solution::has_cycle());
}
