#include <iostream>
#include <sstream>
#include <vector>


namespace
{
	int max_strength = 0;
	int max_length = 0;

	using Component = std::pair<int, int>;

	void next(std::vector<Component> components, int port, int total, int length)
	{
		size_t index = 0;
		for (const auto& component : components)
		{
			bool first = component.first == port;
			bool second = component.second == port;
			if (first || second)
			{
				auto new_components = components;
				new_components.erase(new_components.begin() + index);
				next(new_components, second ? component.first : component.second, total + component.first + component.second, length + 1);
			}
			++index;
		}

		if (length >= max_length && total > max_strength)
		{
			max_strength = total;
			max_length = length;
		}
	}
}

void main()
{
	std::vector<Component> components;

	std::string input;
	while (std::getline(std::cin, input))
	{
		std::stringstream cin(input);

		int left, right;
		cin >> left;
		cin.ignore(1);
		cin >> right;

		components.emplace_back(left, right);
	}

	next(components, 0, 0, 0);

	std::cout << max_strength << std::endl;
}