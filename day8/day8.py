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

    for i in range(1, len(forest)-1):
        ltallest = forest[i][0]
        rtallest = forest[i][len(forest[i])-1]

        for j in range(1, len(forest[i])):

            if forest[i][j] > ltallest:
                visible_trees_lat.add((i,j))
                ltallest = forest[i][j]

            if forest[i][len(forest[i])-1-j] > rtallest:
                visible_trees_lat_rev.add((i, len(forest[i])-1-j))
                rtallest = forest[i][len(forest[i])-1-j]
        
    for j in range(1, len(forest[0])-1):
        ltallest = forest[0][j]
        rtallest = forest[len(forest)-1][j]

        for i in range(1, len(forest)):

            if forest[i][j] > ltallest:
                visible_trees_long.add((i,j))
                ltallest = forest[i][j]

            if forest[i][len(forest[i])-1-j] > rtallest:
                visible_trees_long_rev.add((i, len(forest[i])-1-j))
                rtallest = forest[i][len(forest[i])-1-j]


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
