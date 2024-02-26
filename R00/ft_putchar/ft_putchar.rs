use std::io::Write;

fn ft_putchar(c: char)
{
	//let _ = std::io::stderr().write(format!("{}", c).as_bytes());
	println!("{}", c);
}

fn main()
{
	ft_putchar(3);
}
