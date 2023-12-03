import os
import sys
import glob
import string
import secrets
import argparse
from urllib.request import urlopen, Request
import importlib.util

AOC_YEAR = 2023

def gensym(length=32, prefix="gensym_"):
    """
    generates a fairly unique symbol, used to make a module name,
    used as a helper function for load_module

    :return: generated symbol
    """
    alphabet = string.ascii_uppercase + string.ascii_lowercase + string.digits
    symbol = "".join([secrets.choice(alphabet) for i in range(length)])

    return prefix + symbol

def load_module(source, module_name=None):
    """
    reads file source and loads it as a module

    :param source: file to load
    :param module_name: name of module to register in sys.modules
    :return: loaded module
    """

    if module_name is None:
        module_name = gensym()

    spec = importlib.util.spec_from_file_location(module_name, source)
    module = importlib.util.module_from_spec(spec)
    sys.modules[module_name] = module
    spec.loader.exec_module(module)

    return module

parser = argparse.ArgumentParser(prog='advent_of_code')
parser.add_argument('day', help='optional. Uses latest day by default', nargs='?', type=int)
parser.add_argument('-e', '--example', help='Run with example input', action='store_true')
args = parser.parse_args()

if args.day is None:
    last_day = sorted(glob.glob('solutions/day[0-9][0-9].py'))[-1]
    args.day = int(os.path.basename(last_day)[3:-3])

day_filename = os.path.join('solutions', 'day{:>02}.py'.format(args.day))
if not os.path.exists(day_filename):
    print('Day {:>02} is not implemented.'.format(args.day))
    exit(1)

day_module = load_module(day_filename)
if not hasattr(day_module, 'part_1') or not callable(day_module.part_1):
    print('Missing function in {}: `def part_1(input)`'.format(day_filename))
    exit(1)
if not hasattr(day_module, 'part_2') or not callable(day_module.part_2):
    print('Missing function in {}: `def part_2(input)`'.format(day_filename))
    exit(1)

print(' >> advent_of_code_{} day {:>02} <<'.format(AOC_YEAR, args.day))

if args.example:
    if not hasattr(day_module, 'EXAMPLE_INPUT') or not isinstance(day_module.EXAMPLE_INPUT, str):
        print('Missing `EXAMPLE_INPUT` in {}'.format(day_filename))
        exit(1)
    input = day_module.EXAMPLE_INPUT
else:
    if not os.path.exists('.input'):
        os.mkdir('.input')

    input_filename = os.path.join('.input', 'input_{:>02}.txt'.format(args.day))
    if not os.path.exists(input_filename):
        print('Downloading input ...')

        with open('.session_token', 'r') as f:
            session_token = f.read()

        if len(session_token) == 0:
            print('Please enter your session cookie into the .session_token file')
            exit(1)

        with open(input_filename, 'wb') as f:
            url = 'https://adventofcode.com/{}/day/{}/input'.format(AOC_YEAR, args.day)
            for line in urlopen(Request(url, headers={'Cookie': 'session={}'.format(session_token)})):
                f.write(line)

    with open(input_filename, 'r') as f:
        input = f.read()

print('Answer part_1: ', end='')
print('{}'.format(day_module.part_1(input)))

print('Answer part_2: ', end='')
print('{}'.format(day_module.part_2(input)))