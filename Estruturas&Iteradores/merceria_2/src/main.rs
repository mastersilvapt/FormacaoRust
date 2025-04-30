
mod item;
mod supermarket;
mod supermarketerrors;

use crate::supermarket::SuperMarket;
///
/// Mercearia 2.0
///
/// Para implementar esta iteração do exercício, deve copiar a versão anterior Mercearia 1.0
/// E fazer todas as alterações pedidas pelo enunciado.
///
/// Devem manter ambas as versões do exercício.
fn main() {
    let mut supermarket = SuperMarket::new();
    
    supermarket.tests();
    supermarket.print_inventory();
}
