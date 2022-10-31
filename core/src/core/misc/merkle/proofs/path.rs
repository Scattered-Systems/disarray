/*
   Appellation: path <proofs> [merkle]
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/

pub fn proof_path(index: usize, size: usize) -> Vec<usize> {
    let mut ans: Vec<usize> = Vec::new();
    let mut pos = index;
    let mut leaf_size = size;
    while leaf_size > 1 {
        if leaf_size % 2 != 0 {
            leaf_size += 1;
        }
        if pos % 2 == 0 {
            ans.push(pos + 1);
        } else {
            ans.push(pos - 1);
        }
        pos /= 2;
        leaf_size /= 2;
    }
    return ans;
}
