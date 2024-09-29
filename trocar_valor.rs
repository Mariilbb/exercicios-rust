// Escreva uma função que receba dois valores (qualquer tipo), que sejam mutáveis e troque seus valores. Utilize referências para realizar a troca.

fn swap(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

fn main() {
    let mut a: i32 = 10;
    let mut b: i32 = -7;
    
    println!("a = {}, b = {}", a, b);
    swap(&mut a, &mut b);
    println!("a = {}, b = {}", a, b);
}
