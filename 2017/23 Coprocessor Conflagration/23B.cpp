#include <iostream>


namespace
{
	using i64 = std::int64_t;
}

void main()
{
	i64 answer = 0;

	for (i64 i = 109300; i <= 126300; i += 17) // loops 1000x
	{
		for (i64 j = 2; j <= i / sqrt(i); ++j)
		{
			if (i % j == 0)
			{
				++answer;
				break;
			}
		}
	}

	std::cout << answer << std::endl;
}