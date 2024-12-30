#include <iostream>
#include <sstream>


namespace
{
	using i64 = std::int64_t;
	using u64 = std::uint64_t;

	struct Particle
	{
		size_t index;
		i64 position[3];
		i64 velocity[3];
		i64 acceleration[3];

		Particle(size_t index, std::string input) : index(index)
		{
			std::stringstream cin(input);
			cin.ignore(3);

			cin >> position[0]; cin.ignore(1);
			cin >> position[1]; cin.ignore(1);
			cin >> position[2]; cin.ignore(6);

			cin >> velocity[0]; cin.ignore(1);
			cin >> velocity[1]; cin.ignore(1);
			cin >> velocity[2]; cin.ignore(6);

			cin >> acceleration[0]; cin.ignore(1);
			cin >> acceleration[1]; cin.ignore(1);
			cin >> acceleration[2];
		}
	};

	u64 get_length(i64 values[3])
	{
		return abs(values[0]) + abs(values[1]) + abs(values[2]);
	}
}

void _()
{
	size_t total = 0;
	size_t smallest_index = 0;
	u64 smallest_length = 0xffffffffff;

	std::string input;
	while (std::getline(std::cin, input))
	{
		Particle p(total++, input);
		u64 length = get_length(p.acceleration);

		if (length < smallest_length)
		{
			smallest_index = p.index;
			smallest_length = length;
		}
	}

	std::cout << smallest_index << std::endl;
}