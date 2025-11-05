/* Controle de Fluxo - Control Flow */



fn main() {
    let number = 3;
    if number < 5 {
        println!("Condição Verdadeira!");
    } else {
        println!("Condicão Falsa!")
    }
 //cascata de ifs

 if number > 3  {
    println!("temos um número que é igual ou maior que 3");
 } else if number < 3 {
    println!("opa temos um valor da variável menor que 3")
} else {
    println!("o número é igual a 3");
}
//exercise se number2 é par ou ímpar
 let number2 = 7;
    if number2 % 2 == 0 {
        println!("O número {} é par", number2);
    } else {
        println!("O número {} é ímpar", number2);
    }
//exercise se idade pode dirigir ou não
 let idade = 17;
    if idade >= 18 {
        println!("Você tem mais que 18 anos e pode dirigir.");
    } else {
        println!("voce te menos de 18 anos e não pode dirigir");
    }
//exercise Verificar a nota com o conceito A, B, C, D, E "reprovado"
    let nota = 29;
        if nota >= 90 {
            println!("Nota A");
        } else if nota >= 80 {
            println!("Nota B");
        } else if nota >= 50 {
            println!("Nota C");
        } else if nota >= 30 {
            println!("Nota D");
        } else {
            println!("Nota E - Reprovado");
        }
}
