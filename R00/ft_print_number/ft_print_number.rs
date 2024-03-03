use std::io::Write;

fn ft_print_number()
{
	let mut num: i32 = 0;

	while num <= 9
	{
		let _ = std::io::stderr().write(format!("{}", num).as_bytes());
		num += 1;
	}
}

fn main()
{
	ft_print_number();
}
