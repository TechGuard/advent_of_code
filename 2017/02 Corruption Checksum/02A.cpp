#include <iostream>
#include <sstream>


void _()
{
	int sum = 0;

	std::string input;
	while (std::getline(std::cin, input))
	{
		std::stringstream cin(input);

		int number;
		int min = 0xfffff;
		int max = 0x0;

		while (cin >> number)
		{
			if (number < min) min = number;
			if (number > max) max = number;
		}

		sum += max - min;
	}

	std::cout << sum << std::endl;
}