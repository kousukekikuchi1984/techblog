fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: u64 = args.get(1).expect("need n").parse().unwrap();
    let sum: u64 = (0..n).filter(|x| x % 3 == 0).sum();
    println!("{}", sum);
}
