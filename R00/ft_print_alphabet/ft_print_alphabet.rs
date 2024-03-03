use std::io::Write;

fn ft_putchar(c: char)
{
	let _ = std::io::stderr().write(format!("{}", c).as_bytes());
}

fn ft_print_alphabet()
{
	let mut letter: u8 = 97;
	while letter <= 122
	{
		ft_putchar(letter as char);
		letter += 1;
	}
}

fn main()
{
	ft_print_alphabet();
}