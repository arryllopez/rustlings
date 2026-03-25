// TODO: Fix the compiler error in the function without adding any new line.
// the parameters has to be declared as mutable to be able to push new value into the vector, 
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    // the ownership of vec is moved back to the caller, so the caller can access the modified vector after this function call
    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        // immutable vector declared here
        let vec0 = vec![22, 44, 66];
        
        let vec1 = fill_vec(vec0); //the ownership of vec0 is moved to fill_vec, so vec0 is no longer accessible after this line
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
