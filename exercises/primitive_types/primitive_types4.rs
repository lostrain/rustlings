// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let index1 = 1;
    let index2 = 3;
    let nice_slice = &a[index1..=index2];

    assert_eq!([2, 3, 4], nice_slice)
}
