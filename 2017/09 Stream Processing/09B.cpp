#include <iostream>
#include <sstream>


void main()
{
	bool garbage = false;
	bool ignore = false;

	int score = 0;

	char input;
	while (std::cin.get(input))
	{
		if (ignore)
		{
			ignore = false;
			continue;
		}

		if (input == '!') ignore = true;
		if (input == '>') garbage = false;

		if (garbage && !ignore) ++score;

		if (input == '<') garbage = true;
	}

	std::cout << score << std::endl;
}