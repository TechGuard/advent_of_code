#include <iostream>
#include <list>


void _()
{
	int step_size;
	std::cin >> step_size;

	std::list<int> values = { 0 };

	int position = 0;
	auto it = values.begin();

	for (int i = 1; i <= 2017; ++i, ++position)
	{
		position = (position + step_size) % values.size();
		it = std::next(values.begin(), position + 1);
		values.insert(it, i);
	}

	std::cout << *it << std::endl;
}