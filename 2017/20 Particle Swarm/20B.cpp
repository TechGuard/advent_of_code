#include <iostream>
#include <sstream>
#include <vector>
#include <algorithm>


namespace
{
	using i64 = std::int64_t;

	struct Particle
	{
		i64 position[3];
		i64 velocity[3];
		i64 acceleration[3];

		Particle(std::string input)
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

		void update()
		{
			velocity[0] += acceleration[0];
			velocity[1] += acceleration[1];
			velocity[2] += acceleration[2];

			position[0] += velocity[0];
			position[1] += velocity[1];
			position[2] += velocity[2];
		}

		bool operator<(const Particle& other) const
		{
			if (position[0] != other.position[0])
				return position[0] < other.position[0];
			if (position[1] != other.position[1])
				return position[1] < other.position[1];
			return position[2] < other.position[2];
		}
	};

	void set(i64(&a)[3], const i64(&b)[3])
	{
		a[0] = b[0];
		a[1] = b[1];
		a[2] = b[2];
	}

	bool equal(const i64 (&a)[3], const i64 (&b)[3])
	{
		return a[0] == b[0] && a[1] == b[1] && a[2] == b[2];
	}
}

void main()
{
	std::string input;
	std::vector<Particle> particles;
	while (std::getline(std::cin, input)) particles.emplace_back(input);

	for (int i = 0; i < 1000; ++i)
	{
		if (particles.size() == 0)
			break;

		for (auto& p : particles)
			p.update();

		std::sort(particles.begin(), particles.end());

		i64 prev_position[3];
		bool remove_prev = true;

		for (auto itr = particles.begin(); itr != particles.end();)
		{
			if (equal(prev_position, itr->position))
			{
				if (remove_prev)
				{
					remove_prev = false;
					--itr;
					continue;
				}
				itr = particles.erase(itr);
			}
			else
			{
				set(prev_position, itr->position);
				remove_prev = true;
				++itr;
			}
		}
	}

	std::cout << particles.size() << std::endl;
}