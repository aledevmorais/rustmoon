const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60;
// const UMA_HORA_EM_SEGUNDOS: i32 = 3600; //outra forma de escrever o mesmo valor
// const UMA_HORA_EM_SEGUNDOS: i32 = 60 * 60; //outra forma de escrever o mesmo valor
// não posso declarar uma const sem tipo, pois o compilador não consegue inferir o tipo de uma constante
// vai dar erro de missing type for constant
// outra coisa é que posso declarar a const novamente dentro do escopo de uma função como outro valor
// esquematizando:
 


fn main() {
// const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 6000;
// viu que tem uma const declarada fora do escopo da função main
// o rust irá assumir o valor da constante UMA_HORA_EM_SEGUNDOS DENTRO DO ESCOPo DA FUNÇÃO main
// NÃO É UMA FORMA RECOMENDADA DE FAZER, MAS É POSSÍVEL.
// não é recomendado, pois pode gerar confusão e bugs no código, mas pode fazer - ficar atento com isso
// imagina dez funções ou mais e assumir a condição humana de verificar se o valor esta dentro ou fora de escopo
// tem que tomar cuidado com variáveis mutáveis e constantes
    println!("Iniciando o Programa");
    let mut x: i32 = 5;
    println!("O Valor de x é: {x}");

    x = UMA_HORA_EM_SEGUNDOS;
    println!("O novo Valor de x agora é sem alterar o tipo: {x}");

}
