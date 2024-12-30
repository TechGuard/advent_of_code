#include <iostream>
#include "hash.h"


#define RETURN_BOOLS(one, two, three, four) output[0] = one; output[1] = two; output[2] = three; output[3] = four; break;

namespace
{
	bool disk[128][128]{};
	bool marked[128][128]{};

	void HexToBits(char hex, bool* output)
	{
		switch (hex)
		{
			case '0': RETURN_BOOLS(0, 0, 0, 0);
			case '1': RETURN_BOOLS(0, 0, 0, 1);
			case '2': RETURN_BOOLS(0, 0, 1, 0);
			case '3': RETURN_BOOLS(0, 0, 1, 1);
			case '4': RETURN_BOOLS(0, 1, 0, 0);
			case '5': RETURN_BOOLS(0, 1, 0, 1);
			case '6': RETURN_BOOLS(0, 1, 1, 0);
			case '7': RETURN_BOOLS(0, 1, 1, 1);
			case '8': RETURN_BOOLS(1, 0, 0, 0);
			case '9': RETURN_BOOLS(1, 0, 0, 1);
			case 'a': RETURN_BOOLS(1, 0, 1, 0);
			case 'b': RETURN_BOOLS(1, 0, 1, 1);
			case 'c': RETURN_BOOLS(1, 1, 0, 0);
			case 'd': RETURN_BOOLS(1, 1, 0, 1);
			case 'e': RETURN_BOOLS(1, 1, 1, 0);
			case 'f': RETURN_BOOLS(1, 1, 1, 1);
		}
	}

	void GetBits(const char* input, bool* output)
	{
		bool* output_ptr = &output[0];
		const size_t length = strlen(input);

		for (size_t i = 0; i < length; i++, output_ptr += 4)
			HexToBits(input[i], output_ptr);
	}

	void MarkNeighbours(int x, int y)
	{
		marked[x][y] = true;
		if (x > 0   && disk[x - 1][y] && !marked[x - 1][y]) MarkNeighbours(x - 1, y);
		if (y > 0   && disk[x][y - 1] && !marked[x][y - 1]) MarkNeighbours(x, y - 1);
		if (x < 127 && disk[x + 1][y] && !marked[x + 1][y]) MarkNeighbours(x + 1, y);
		if (y < 127 && disk[x][y + 1] && !marked[x][y + 1]) MarkNeighbours(x, y + 1);
	}
}

void main()
{
	std::string input;
	std::getline(std::cin, input);

	int groups = 0;

	for (int i = 0; i < 128; i++)
	{
		std::string row = input + "-" + std::to_string(i);

		char hash[33];
		Hash(row.c_str(), hash);
		GetBits(hash, disk[i]);
	}

	for (int x = 0; x < 128; x++)
	{
		for (int y = 0; y < 128; y++)
		{
			if (marked[x][y] || !disk[x][y])
				continue;
			++groups;
			MarkNeighbours(x, y);
		}
	}

	std::cout << groups << std::endl;
}