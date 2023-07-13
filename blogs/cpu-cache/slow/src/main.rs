// slow
fn main() {
    let mut array1 = vec![vec![1; 10000]; 10000];
    let array2 = vec![vec![2; 10000]; 10000];
    for i in 0..10000 {
        for j in 0..10000 {
            array1[j][i] = array2[j][i];
        }
    }
}
