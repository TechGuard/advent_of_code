#include <iostream>
#include <iomanip>


void main()
{
	const int number_length = 256;
	const int hash_length = 16;
	const int rounds = 64;

	int numbers[number_length];
	for (int i = 0; i < number_length; ++i) numbers[i] = i;

	char lengths[256]{};
	int length_count = 0;

	while (std::cin >> lengths[length_count]) ++length_count;

	lengths[length_count++] = 17;
	lengths[length_count++] = 31;
	lengths[length_count++] = 73;
	lengths[length_count++] = 47;
	lengths[length_count++] = 23;

	int pos = 0;
	int skip = 0;

	for (int k = 0; k < rounds; ++k)
	{
		for (int i = 0; i < length_count; ++i)
		{
			int length = lengths[i];
			int end = pos + length - 1;

			for (int j = 0; j < length / 2; ++j)
			{
				int tmp = numbers[(pos + j) % number_length];
				numbers[(pos + j) % number_length] = numbers[(end - j) % number_length];
				numbers[(end - j) % number_length] = tmp;
			}

			pos += length + skip;
			++skip;
		}
	}

	const int hash_count = number_length / hash_length;

	for (int i = 0; i < hash_count; i++)
	{
		int offset = i * hash_length;

		int hash = numbers[offset];
		for (int j = 1; j < hash_count; j++) hash ^= numbers[offset + j];

		std::cout << std::setfill('0') << std::setw(2) << std::hex << hash;
	}

	std::cout << std::endl;
}