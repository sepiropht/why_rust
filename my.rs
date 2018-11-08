fn main() {
  let ages = vec![23, 24, 25, 34, 33];
  let moyenne_age = ages
                     .iter()
                     .fold(0, |total, age| total + age) / 5;
}