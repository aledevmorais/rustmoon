fn main() {
    let mut number = 3;
    println!("\n Usando While enquanto for diferente de 0");
    //while quando ou enquanto number for diferente de 0 decrementa 1 essa é a função do while
    //carrega o number que é = 3 é diferente de 0 SIM, então printa 3 e decrementa 1
    //carrega o number que é = 2 é diferente de 0 SIM, então printa 2 e decrementa 1
    //carrega o number que é = 1 é diferente de 0 SIM, então printa 1 e decrementa 1
    //carrega o number que é = 0 é igual a 0, a condição é que que ser diferente de 0
    //result = sai do while
    while number != 0 {
        println!("While {number}");
        number -= 1;
    }
}
