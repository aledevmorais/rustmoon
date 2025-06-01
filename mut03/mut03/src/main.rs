fn main() {
    println!("inicio do Programa");
    let mut x:i32 = 5;
    println!("O valor de x é: {x}");

    x = 6;
    let x:i32 = 666;
    println!("O valor de x agora é: {x}");

    let mut y:i32 = 5;
    println!("O valor de y é: {y}");
    y = 6;
    println!("O valor de y agora é: {y}");

    //essa é esquematização de reatribuição de valor de uma let (mutável)
    // o cargo run emitirá um warning pois a x =6; não foi utilizada e abaixo o run fez o println!
    // O valor de x é: 5
    // O valor de x agora é: 666
    // O valor de y é: 5
    // O valor de y agora é: 6
}
