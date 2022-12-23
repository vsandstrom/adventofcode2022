import math
def parse(input):
    data = []
    with open(input, "r") as file:
        for line in file.readlines():
            temp = list(line)
            temp.remove('\n')
            temp = [int(obj) for obj in temp]
            data.append(temp)
        return data

def visible_trees(forest):
    # iterate over each row in the forest and store coordinates of visible trees
    visible_trees_lat = set()
    visible_trees_lat_rev = set()
    visible_trees_long = set()
    visible_trees_long_rev = set()








    print(visible_trees_lat)
    print(visible_trees_lat_rev)
    print(visible_trees_long)
    print(visible_trees_long_rev)

def forest_size(input):
    sum = 0
    for line in input:
        sum += len(line)

    return sum

def testedges(input):
    sum = 0
    sum += len(input) * 2
    sum += len(input[0]) * 2
    sum -= 4

    return sum

def main():
    #print(testedges(parse("input.txt")))
    input = parse("test.txt")
    total = forest_size(input)
    inv_trees = visible_trees(input)

    # print(total - inv_trees)


if __name__=="__main__":
    main()
