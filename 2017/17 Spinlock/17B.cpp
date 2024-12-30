#include <iostream>


void main()
{
	int step_size;
	std::cin >> step_size;

	int position = 0;
	int length = 1;
	int value = 0;

	for (int i = 1; i <= 50000000; ++i, ++position)
	{
		position = (position + step_size) % length;
		if (position == 0)
			value = i;
		++length;
	}

	std::cout << value << std::endl;
}