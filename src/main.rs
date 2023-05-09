use std::io;

fn convert_to_int(data_input: & String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();    // Convert String to Int
    x
}

fn main() {
    let mut A1 = String::new();
    let mut B1 = String::new();
    let mut C1 = String::new();
    let mut D1 = String::new();
    
    io::stdin().read_line(&mut A1).expect("Erro ao ler A");
    io::stdin().read_line(&mut B1).expect("Erro ao ler B");
    io::stdin().read_line(&mut C1).expect("Erro ao ler C");
    io::stdin().read_line(&mut D1).expect("Erro ao ler D");
    
    let mut A = convert_to_int(&mut A1);
    let mut B = convert_to_int(&mut B1);
    let mut C = convert_to_int(&mut C1);
    let mut D = convert_to_int(&mut D1);
    
    if (B > C && D > A) && (C + D > A + B) && (A > 0 && B > 0 && C > 0 && D > 0) && (A%2 == 0) {
        println!("Valores aceitos");
    }
    else{
    println!("Valores nao aceitos");
    }
}