#include <iostream>
#include <vector>
#include <string>


namespace
{
	std::string getState(int* banks, int bank_count)
	{
		std::string str;
		for (int i = 0; i < bank_count; i++)
			str += std::to_string(banks[i]);
		return str;
	}
}

void _()
{
	int banks[16]{};
	int bank_count = 0;
	while (std::cin >> banks[bank_count]) ++bank_count;

	int steps = 0;
	std::vector<std::string> last_states;

	while (true)
	{
		++steps;

		// Find highest block count
		int most_idx = 0;
		int most_blocks = 0;
		for (int i = 0; i < bank_count; i++)
		{
			if (banks[i] > most_blocks)
			{
				most_blocks = banks[i];
				most_idx = (int)i;
			}
		}

		// Redistrubute
		banks[most_idx] = 0;
		for (int i = most_idx + 1, j = 0; j < most_blocks; ++i, ++j)
			++banks[i % bank_count];

		// Check if we've seen this state before
		const std::string state = getState(banks, bank_count);
		const auto itr = std::find(last_states.begin(), last_states.end(), state);
		if (itr != last_states.end())
		{
			break;
		}

		// Remember state
		last_states.push_back(state);
	}

	std::cout << steps << std::endl;
}