#include <iostream>
#include <utility>


// https://math.stackexchange.com/questions/163080/on-a-two-dimensional-grid-is-there-a-formula-i-can-use-to-spiral-coordinates-in#answer-163101
std::pair<int, int> position(int n)
{
	const int k = (int)ceil((sqrt(n) - 1.0) / 2.0);
	int t = 2 * k + 1;
	int m = t * t;

	t -= 1;

	if (n >= m - t)
		return { k - (m - n), -k };

	m -= t;

	if (n >= m - t)
		return { -k, -k + (m - n) };

	m -= t;

	if (n >= m - t)
		return { -k + (m - n), k };

	return { k, k - (m - n - t) };
}

void _()
{
	int index;
	std::cin >> index;

	const auto pos = position(index);
	int solution = std::abs(pos.first) + std::abs(pos.second); // distance

	std::cout << solution << std::endl;
}