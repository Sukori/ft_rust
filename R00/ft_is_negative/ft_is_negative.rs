fn ft_is_negative(num: i32)
{
	if num < 0
	{
		println!("{}", 'N');
	}
	else
	{
		println!("{}", 'P');	
	}
}

fn main()
{
	ft_is_negative(-42);
}