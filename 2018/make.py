import os, sys, re

day = sys.argv[1]
title = sys.argv[3]
example = sys.stdin.read()

# build template
with open(os.path.join('src', 'day_template.rs')) as f:
    template = f.read()

template = template.replace('<DAY>', day)
template = template.replace('<EXAMPLE INPUT>', ('\\\n' if example.count('\n') else '') + example)

# write template
title = re.sub(r'[^\w\s\-_]', '', title.lower())
title = re.sub(r'[\-_\s]+', '_', title)
filename = 'day{}_{}'.format(day, title)
filepath = os.path.join('src', filename + '.rs')
with open(filepath, 'x') as f:
    f.write(template)
    print('{}\nCreated {}'.format(example, filepath))

# insert into main.rs
with open(os.path.join('src', 'main.rs')) as f:
    main = f.read()

last_comma = main.rfind(',\n') + 2
if last_comma > 1:
    main = main[:last_comma] + '    {},\n'.format(filename) + main[last_comma:]
    with open(os.path.join('src', 'main.rs'), 'w') as f:
        f.write(main)