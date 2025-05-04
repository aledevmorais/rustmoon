fn main() {
    println!("inicio do Programa");
    let x:i32 = 5;
    println!("O valor de x é: {x}");

    //x = 10;
    //não é possivel mudar o valor de x, pois ele é imutável
    //se eu quisesse mudar o valor de x, teria que declarar ele como mutável
    //let mut x:i32 = 5;
    
    let x:i32 = 60;
    println!("O valor de x é: {x}");
    // posso fazer pois o let não é criação é atribuição por isso posso reatribuir o valor de x
    // falando sobre mutabilidade, o rust é uma linguagem que tem como padrão a imutabilidade correto
    // o que significa que se eu não declarar uma variável como mutável, ela será imutável
    // o que é interessante, pois isso evita muitos bugs
    // agora outra coisa é que rust permitiu uma alteração de valor da let mas não alterar o seu tipo.
    // quando eu gerei o cargo run ele assumiu dois valores para x.
}
