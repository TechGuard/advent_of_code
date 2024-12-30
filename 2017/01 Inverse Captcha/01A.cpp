#include <iostream>
#include <sstream>


void _()
{
	std::string input;
	std::getline(std::cin, input);

	int sum = 0;

	for (size_t i = 0; i < input.size(); ++i)
	{
		int a = input[i] - '0';
		int b = input[(i + 1) % input.size()] - '0';
		if (a == b)
			sum += a;
	}

	std::cout << sum << std::endl;
}