package main

import (
	"strings"
	"bufio"
	"fmt"
)

//var input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\nan\n"
var input = "abaacccccccccccccaaaaaaaccccccccccccccccccccccccccccccccccaaaaaa\nabaaccccccccccccccaaaaaaaaaaccccccccccccccccccccccccccccccccaaaa\nabaaaaacccccccccaaaaaaaaaaaaccccccccccccccccccccccccccccccccaaaa\nabaaaaaccccccccaaaaaaaaaaaaaacccccccccccccccccdcccccccccccccaaaa\nabaaaccccccccccaaaaaaaaccacacccccccccccccccccdddcccccccccccaaaaa\nabaaacccccccccaaaaaaaaaaccaaccccccccccccciiiiddddcccccccccccaccc\nabcaaaccccccccaaaaaaaaaaaaaaccccccccccciiiiiijddddcccccccccccccc\nabccaaccccccccaccaaaaaaaaaaaacccccccccciiiiiijjddddccccaaccccccc\nabccccccccccccccaaacaaaaaaaaaaccccccciiiiippijjjddddccaaaccccccc\nabccccccccccccccaacccccaaaaaaacccccciiiippppppjjjdddddaaaaaacccc\nabccccccccccccccccccccaaaaaaccccccckiiippppppqqjjjdddeeeaaaacccc\nabccccccccccccccccccccaaaaaaccccckkkiippppuupqqjjjjdeeeeeaaccccc\nabccccccccccccccccccccccccaaccckkkkkkipppuuuuqqqjjjjjeeeeeaccccc\nabccccccccccccccccccccccccccckkkkkkoppppuuuuuvqqqjjjjjkeeeeccccc\nabcccccccccccccccccccccccccckkkkooooppppuuxuvvqqqqqqjkkkeeeecccc\nabccaaccaccccccccccccccccccckkkoooooopuuuuxyvvvqqqqqqkkkkeeecccc\nabccaaaaacccccaaccccccccccckkkoooouuuuuuuxxyyvvvvqqqqqkkkkeecccc\nabcaaaaacccccaaaacccccccccckkkooouuuuxxxuxxyyvvvvvvvqqqkkkeeeccc\nabcaaaaaaaaaaaaacccccccccccjjjooottuxxxxxxxyyyyyvvvvrrrkkkeecccc\nabcccaaaacaaaaaaaaacaaccccccjjoootttxxxxxxxyyyyyyvvvrrkkkfffcccc\nSbccaacccccaaaaaaaaaaaccccccjjjooottxxxxEzzzyyyyvvvrrrkkkfffcccc\nabcccccccccaaaaaaaaaaaccccccjjjooootttxxxyyyyyvvvvrrrkkkfffccccc\nabcaacccccaaaaaaaaaaaccccccccjjjooottttxxyyyyywwvrrrrkkkfffccccc\nabaaacccccaaaaaaaaaaaaaacccccjjjjonnttxxyyyyyywwwrrlllkfffcccccc\nabaaaaaaaaaaacaaaaaaaaaaccccccjjjnnnttxxyywwyyywwrrlllffffcccccc\nabaaaaaaaaaaaaaaaaaaaaaaccccccjjjnntttxxwwwwwywwwrrlllfffccccccc\nabaaccaaaaaaaaaaaaaaacccccccccjjjnntttxwwwsswwwwwrrlllfffccccccc\nabaacccaaaaaaaacccaaacccccccccjjinnttttwwsssswwwsrrlllgffacccccc\nabccccaaaaaaccccccaaaccccccccciiinnntttsssssssssssrlllggaacccccc\nabccccaaaaaaaccccccccccaaccccciiinnntttsssmmssssssrlllggaacccccc\nabccccaacaaaacccccccaacaaaccccciinnnnnnmmmmmmmsssslllgggaaaacccc\nabccccccccaaacccccccaaaaacccccciiinnnnnmmmmmmmmmmllllgggaaaacccc\nabaaaccccccccccccccccaaaaaacccciiiinnnmmmhhhmmmmmlllgggaaaaccccc\nabaaaaacccccccccccaaaaaaaaaccccciiiiiiihhhhhhhhmmlgggggaaacccccc\nabaaaaaccccaaccccaaaaaaacaacccccciiiiihhhhhhhhhhggggggcaaacccccc\nabaaaaccccaaaccccaaaacaaaaacccccccciiihhaaaaahhhhggggccccccccccc\nabaaaaaaacaaacccccaaaaaaaaaccccccccccccccaaaacccccccccccccccccaa\nabaacaaaaaaaaaaaccaaaaaaaaccccccccccccccccaaaccccccccccccccccaaa\nabcccccaaaaaaaaacccaaaaaaaccccccccccccccccaacccccccccccccccccaaa\nabccccccaaaaaaaaaaaaaaaaacccccccccccccccccaaacccccccccccccaaaaaa\nabcccccaaaaaaaaaaaaaaaaaaaaaccccccccccccccccccccccccccccccaaaaaa\nan\n"
var reader = bufio.NewReader(strings.NewReader(input))

type Node struct {
	X, Y int
}

func main() {
	grid := make(map[Node]byte)
	x, y := 0, 0
	rows := 0
	for {
		line, ok := readLine()
		if !ok {
			break
		}

		for i := range line {
			if line[i] == 'S' {
				x = rows
				y = i
			}

			node := Node{
				X: rows,
				Y: i,
			}

			grid[node] = line[i]
		}

		rows++
	}

	SNode := Node{
		X: x,
		Y: y,
	}

	grid[SNode] = 'a'

	for i := 0; i < 41; i++ {
		traverse(i, 0, grid)
	}
}

func traverse(x, y int, grid map[Node]byte) bool {
	queue := make([]Node, 0)

	queue = append(queue, Node{X: x, Y: y})

	steps := 1
	visitedMap := make(map[Node]bool)

	dist := 1

	for len(queue) != 0 {
		dist--
		workingNode := queue[0]
		queue = queue[1:]

		neighbours, finished := getNeighbours(workingNode, grid, visitedMap)
		if finished {
			fmt.Println(steps)
			break
		}

		queue = append(queue, neighbours...)
		if dist == 0 {
			//if workingNode.Y == 0 {
			//	// part 2
			//	if grid[workingNode] == 'a' {
			//		steps = 0
			//		dist = 1
			//		continue
			//	}
			//}

			steps++
			dist = len(queue)
		}
	}

	return false
}

func getNeighbours(n Node, grid map[Node]byte, visited map[Node]bool) ([]Node, bool) {
	res := make([]Node, 0)

	visited[n] = true

	bottomNode := Node{X: n.X + 1, Y: n.Y}
	rightNode := Node{X: n.X, Y: n.Y + 1}
	topNode := Node{X: n.X - 1, Y: n.Y}
	leftNode := Node{X: n.X, Y: n.Y - 1}

	if grid[n] == 'z' &&
		(grid[bottomNode] == 'E' || grid[rightNode] == 'E' || grid[topNode] == 'E' || grid[leftNode] == 'E') {
		return nil, true
	}

	if char, ok := grid[bottomNode]; ok && !visited[bottomNode] && int(char)-int(grid[n]) <= 1 {
		visited[bottomNode] = true
		res = append(res, bottomNode)
	}

	if char, ok := grid[rightNode]; ok && !visited[rightNode] && int(char)-int(grid[n]) <= 1 {
		visited[rightNode] = true
		res = append(res, rightNode)
	}

	if char, ok := grid[topNode]; ok && !visited[topNode] && int(char)-int(grid[n]) <= 1 {
		visited[topNode] = true
		res = append(res, topNode)
	}

	if char, ok := grid[leftNode]; ok && !visited[leftNode] && int(char)-int(grid[n]) <= 1 {
		visited[leftNode] = true
		res = append(res, leftNode)
	}

	return res, false
}
func readLine() (string, bool) {
	line, err := reader.ReadString('\n')
	if err != nil {
		panic(err)
	}

	line = strings.TrimRight(line, "\n")

	if strings.Contains(line, "an") {
		return "", false
	}

	return line, true
}
