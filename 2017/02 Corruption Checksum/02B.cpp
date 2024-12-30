#include <iostream>
#include <sstream>
#include <algorithm>


int solve(const int* numbers, int number_count)
{
	for (int i = 0; i < number_count; ++i)
	{
		for (int j = i + 1; j < number_count; ++j)
		{
			int min = std::min(numbers[i], numbers[j]);
			int max = std::max(numbers[i], numbers[j]);
			if (max % min == 0)
				return max / min;
		}
	}
	return 0;
}

void main()
{
	int sum = 0;

	std::string input;
	while (std::getline(std::cin, input))
	{
		std::stringstream cin(input);

		int numbers[256]{};
		int number_count = 0;
		while (cin >> numbers[number_count]) ++number_count;

		sum += solve(numbers, number_count);
	}

	std::cout << sum << std::endl;
}