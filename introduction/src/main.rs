fn main() {
    let x = 5;
    let y = x + 1;

    println!("x = {}, y = {}", x, y);

    let nome = "Leonardo";
    let mut nome2 = String::from("");

    nome2 = match nome2.len() {
        0 => "Sem nome".to_string(),
        _ => "Leonardo".to_string(),
    };

    println!("nome = {}, nome2 = {}", nome, nome2);

    println!("Hello, world!");

    let (mut b, n) = (5, 10);

    b += 5;

    println!("b = {}, n = {}", b, n);

    let [mut r, t] = [1, 2];

    // só posso colocar como u32 (unsigned) se tenho certeza que o valor p é positivo
    let p: u32 = 10;

    [r, _] = [10, 5];

    println!("r = {}, t = {}", r, t);

    let mut tuple: (u32, &str, ()) = (1, "Leonardo", ());

    tuple.0 = 10;

    println!("tuple = {:?}", tuple.2);
}
