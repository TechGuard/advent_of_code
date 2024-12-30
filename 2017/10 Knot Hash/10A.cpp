#include <iostream>


void _()
{
	const int number_length = 256;
	int numbers[number_length];
	for (int i = 0; i < number_length; ++i) numbers[i] = i;

	int lengths[16]{};
	int length_count = 0;

	while (std::cin >> lengths[length_count])
	{
		++length_count;
		std::cin.get();
	}

	int pos = 0;
	int skip = 0;

	for (int i = 0; i < length_count; i++)
	{
		int length = lengths[i];
		int end = pos + length - 1;

		for (int j = 0; j < length / 2; j++)
		{
			int tmp = numbers[(pos + j) % number_length];
			numbers[(pos + j) % number_length] = numbers[(end - j) % number_length];
			numbers[(end - j) % number_length] = tmp;
		}

		pos += length + skip;
		++skip;
	}

	std::cout << (numbers[0] * numbers[1]) << std::endl;
}