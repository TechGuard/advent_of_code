import os, sys, re

day = sys.argv[1]
title = sys.argv[3]
example = sys.stdin.read()

# build template
with open(os.path.join('solutions', 'day_template.py')) as f:
    template = f.read()

template = template.replace('<EXAMPLE INPUT>', ('\\\n' if example.count('\n') else '') + example)

# write template
title = re.sub(r'[^\w\s\-_]', '', title)
filename = '{} {}.py'.format(day, title)
filepath = os.path.join('solutions', filename)
with open(filepath, 'x') as f:
    f.write(template)
    print('{}\nCreated {}'.format(example, filepath))