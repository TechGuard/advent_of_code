#include <iostream>
#include <sstream>
#include <string>
#include <vector>


namespace
{
	std::vector<struct Layer> layers;

	struct Layer
	{
		int depth = 0;
		int range = 0;
		int scanner = 0;
		bool scanup = false;
		int remember_state = 0;

		void Update()
		{
			if (scanup)
			{
				if (--scanner == 0)
					scanup = false;
			}
			else
			{
				if (++scanner == range - 1)
					scanup = true;
			}
		}
	};

	bool IsDetected()
	{
		// copy
		auto layers = ::layers;

		// start simulation
		int max_depth = layers.back().depth;
		auto next_layer = layers.begin();

		for (int depth = 0; depth <= max_depth; depth++)
		{
			// check current layer
			if (next_layer->depth == depth && next_layer->scanner == 0)
				return true;

			// update scanners
			for (auto& layer : layers)
				layer.Update();

			// move to next layer
			if (next_layer->depth == depth)
				++next_layer;
		}

		return false;
	}
}

void main()
{
	// parse layers
	std::string input;
	while (std::getline(std::cin, input))
	{
		std::stringstream cin(input);

		Layer layer{};
		cin >> layer.depth;

		char skip;
		cin >> skip;

		cin >> layer.range;
		layers.push_back(layer);
	}

	int delay = 0;
	while (true)
	{
		if (!IsDetected())
			break;

		for (auto& layer : layers)
			layer.Update();

		++delay;
	}

	std::cout << delay << std::endl;
}