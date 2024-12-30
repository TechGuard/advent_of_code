#include <iostream>


void _()
{
	int numbers[2000]{};
	int number_count = 0;
	while (std::cin >> numbers[number_count]) ++number_count;

	int idx = 0;
	int steps = 0;

	while (idx >= 0 && idx < number_count)
	{
		int jump = numbers[idx]++;
		idx += jump;
		++steps;
	}

	std::cout << steps << std::endl;
}