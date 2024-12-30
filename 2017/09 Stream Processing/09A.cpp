#include <iostream>
#include <sstream>


void _()
{
	int group_count = 0;
	bool garbage = false;
	bool ignore = false;

	int score = 0;

	char input;
	while (std::cin >> input)
	{
		if (ignore)
		{
			ignore = false;
			continue;
		}

		if (input == '!') ignore = true;

		if (!garbage)
		{
			if (input == '<') garbage = true;
			if (input == '{') ++group_count;
			if (input == '}')
			{
				score += group_count;
				--group_count;
			}
		}

		if (input == '>') garbage = false;
	}

	std::cout << score << std::endl;
}