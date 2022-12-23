import re

class Dirs:
    def __init__(self, subdirs, sum) -> None:
        self.subdirs = subdirs
        self.sum = sum

def parse(input):
    data = []

    root = re.compile(r'\$\scd\s/')
    cd_dir = re.compile(r'\$\scd\s[a-z]+')
    subdir = re.compile(r'dir\s[a-z]+')
    f = re.compile(r'[0-9]+\s[a-z.]+')

    sum = 0
    dir = ''

    entry = Dirs([], 0)
    hash = {}

    with open(input, "r") as file:
        for line in file.readlines():
            if root.match(line):
                dir = '/'
            if subdir.match(line):
                entry.subdirs.append(line.split(' ')[1])
            if f.match(line):
                entry.sum += int(line.split(' ')[0])
            if cd_dir.match(line):
                hash[dir] = entry
                entry = Dirs([], 0)
                dir = line.split(' ')[2]

        hash[dir] = entry
        return hash

def traverse(startpos, hashtable):
    sum = 0

    if len(hashtable[startpos].subdirs) == 0:
        return hashtable[startpos].sum
    for obj in hashtable[startpos]:
        while len(obj.subdirs) > 1:
            obj.sum += traverse(obj.subdirs.pop(), hashtable)
        obj.sum += traverse(obj.subdirs.pop(), hashtable)
    return sum

def sort_table(hashtable):
    sum = 0
    for obj in hashtable:
        if obj.sum >= 100000:
            sum += obj.sum
    return sum

def main():
    hashtable = parse("input.txt")
    traverse('/', hashtable)
    val = sort_table(hashtable)
    print(val)

if __name__=="__main__":
    main()
