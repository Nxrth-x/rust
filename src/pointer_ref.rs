// Reference pointers - Point to a resource in memory

pub fn run() {
    // Primitive array
    let array1 = [1, 2, 3];
    let array2 = array1;

    println!("Values (Arrays): {:?}", (array1, array2));

    // With non-primitives, if you assing another variables to a piece of data, the first variable
    // will no longer hold that value. You'll need to use a reference (&) to point to the
    // resource[...]
    // Rust's own documentation

    // Vectors
    // let vec1 = vec![1, 2, 3];
    // let vec2 = vec1;
    // Doing this will give you an error
    // println!("Vectors: {:?}", (vec1, vec2));

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("Values (Vectors): {:?}", (&vec1, vec2));
}
