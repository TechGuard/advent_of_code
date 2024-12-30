#include <iostream>


const int grid_size = 512;
const int center_idx = grid_size / 2;
const int grid_radius = grid_size / 2 - 1;
int grid[grid_size + 1][grid_size + 1]{};


// https://stackoverflow.com/questions/398299/looping-in-a-spiral
template<typename Callback>
void snake(int width, int height, Callback callback)
{
	int x, y, dx, dy;
	x = y = dx = 0;
	dy = -1;
	int t = width < height ? width : height;
	int maxI = t*t;

	for (int i = 0; i < maxI; i++)
	{
		if ((-width / 2 <= x) && (x <= width / 2) && (-height / 2 <= y) && (y <= height / 2))
		{
			if (callback(x, y))
				return;
		}
		if ((x == y) || ((x < 0) && (x == -y)) || ((x > 0) && (x == 1 - y)))
		{
			t = dx;
			dx = -dy;
			dy = t;
		}
		x += dx;
		y += dy;
	}
}

void main()
{
	int index;
	std::cin >> index;

	int solution;
	grid[center_idx][center_idx] = 1;

	snake(grid_radius, grid_radius, [&](int x, int y)
	{
		int sum = 0;
		for (int offset_y = -1; offset_y <= 1; offset_y++) for (int offset_x = -1; offset_x <= 1; offset_x++)
		{
			sum += grid[center_idx + y + offset_y][center_idx + x + offset_x];
		}
		grid[center_idx + y][center_idx + x] = sum;

		if (sum > index)
		{
			solution = sum;
			return true;
		}
		return false;
	});

	std::cout << solution << std::endl;
}