  fn main () {
    let foo = vec![1, 2];
    let bar = foo;
    println!("{}", bar); // [1, 2]
    println!("{}", foo); // Erreur de compilation !!! La variable dans foo a été move 
  }