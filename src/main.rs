use std::io;

fn convert_to_int(data_input: & String) -> i32
{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main()
{
    let mut soma: i32;
    let mut valor_entrada = String::new();
    let mut valor_int: i32;
    let mut r: i32;

    io::stdin().read_line(&mut valor_entrada).expect("Erro ao ler valor_entrada.");
    valor_int = convert_to_int(&valor_entrada);

    soma = 0;
    while valor_int != 0
    {
        r = valor_int % 10;
        soma += r;
        valor_int = valor_int / 10;
    }

    println!("A soma dos digitos é: {}", soma);
}
