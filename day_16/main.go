package main

import (
	"bufio"
	"fmt"
	"math"
	"regexp"
	"strconv"
	"strings"
)

const timeLimit = 26

var input = "Valve OM has flow rate=0; tunnels lead to valves AA, EZ\nValve ZZ has flow rate=0; tunnels lead to valves LR, QY\nValve NC has flow rate=0; tunnels lead to valves KX, QI\nValve QI has flow rate=5; tunnels lead to valves TX, NC, QS, HY, UX\nValve QS has flow rate=0; tunnels lead to valves CY, QI\nValve FP has flow rate=0; tunnels lead to valves IW, SJ\nValve ZR has flow rate=0; tunnels lead to valves ID, KC\nValve YR has flow rate=21; tunnels lead to valves RS, OT, FV\nValve SJ has flow rate=23; tunnel leads to valve FP\nValve QY has flow rate=0; tunnels lead to valves ZZ, NU\nValve KD has flow rate=13; tunnels lead to valves WY, ZP\nValve GT has flow rate=0; tunnels lead to valves SG, PD\nValve DB has flow rate=0; tunnels lead to valves TX, MX\nValve KW has flow rate=0; tunnels lead to valves AK, HM\nValve TX has flow rate=0; tunnels lead to valves QI, DB\nValve YX has flow rate=0; tunnels lead to valves HY, AA\nValve NA has flow rate=0; tunnels lead to valves NU, KS\nValve ST has flow rate=0; tunnels lead to valves YO, PD\nValve UX has flow rate=0; tunnels lead to valves QI, OT\nValve OT has flow rate=0; tunnels lead to valves UX, YR\nValve AK has flow rate=0; tunnels lead to valves KW, PD\nValve UC has flow rate=0; tunnels lead to valves YH, KC\nValve FF has flow rate=0; tunnels lead to valves YO, IN\nValve GN has flow rate=0; tunnels lead to valves CY, MX\nValve KK has flow rate=0; tunnels lead to valves WY, YO\nValve PD has flow rate=10; tunnels lead to valves GT, ID, HW, ST, AK\nValve LR has flow rate=18; tunnels lead to valves ZZ, NM, SG, YK\nValve CY has flow rate=14; tunnels lead to valves VB, GN, QS, FV\nValve YH has flow rate=0; tunnels lead to valves UC, VQ\nValve RS has flow rate=0; tunnels lead to valves MX, YR\nValve YO has flow rate=20; tunnels lead to valves FF, NM, KK, ST, ZU\nValve HQ has flow rate=0; tunnels lead to valves AA, MX\nValve UE has flow rate=0; tunnels lead to valves HM, IN\nValve NM has flow rate=0; tunnels lead to valves LR, YO\nValve KX has flow rate=7; tunnels lead to valves NC, UZ, XK, PV\nValve IW has flow rate=0; tunnels lead to valves VQ, FP\nValve IN has flow rate=22; tunnels lead to valves FF, UE\nValve WY has flow rate=0; tunnels lead to valves KK, KD\nValve HY has flow rate=0; tunnels lead to valves YX, QI\nValve AA has flow rate=0; tunnels lead to valves KS, OM, XO, HQ, YX\nValve ZU has flow rate=0; tunnels lead to valves YO, NU\nValve YK has flow rate=0; tunnels lead to valves ZP, LR\nValve XK has flow rate=0; tunnels lead to valves XO, KX\nValve VB has flow rate=0; tunnels lead to valves CY, UZ\nValve ZP has flow rate=0; tunnels lead to valves KD, YK\nValve VQ has flow rate=11; tunnels lead to valves YH, IW, EZ\nValve HW has flow rate=0; tunnels lead to valves NU, PD\nValve NU has flow rate=8; tunnels lead to valves ZU, UD, NA, HW, QY\nValve UZ has flow rate=0; tunnels lead to valves KX, VB\nValve PV has flow rate=0; tunnels lead to valves DY, KX\nValve MX has flow rate=6; tunnels lead to valves HQ, DB, DY, RS, GN\nValve KS has flow rate=0; tunnels lead to valves NA, AA\nValve UD has flow rate=0; tunnels lead to valves NU, IO\nValve FV has flow rate=0; tunnels lead to valves YR, CY\nValve SG has flow rate=0; tunnels lead to valves LR, GT\nValve HM has flow rate=24; tunnels lead to valves KW, UE\nValve XO has flow rate=0; tunnels lead to valves AA, XK\nValve KC has flow rate=12; tunnels lead to valves IO, UC, ZR\nValve IO has flow rate=0; tunnels lead to valves UD, KC\nValve DY has flow rate=0; tunnels lead to valves PV, MX\nValve ID has flow rate=0; tunnels lead to valves PD, ZR\nValve EZ has flow rate=0; tunnels lead to valves VQ, OM\nan\n"

// var input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\nValve CC has flow rate=2; tunnels lead to valves DD, BB\nValve DD has flow rate=20; tunnels lead to valves CC, AA, EE\nValve EE has flow rate=3; tunnels lead to valves FF, DD\nValve FF has flow rate=0; tunnels lead to valves EE, GG\nValve GG has flow rate=0; tunnels lead to valves FF, HH\nValve HH has flow rate=22; tunnel leads to valve GG\nValve II has flow rate=0; tunnels lead to valves AA, JJ\nValve JJ has flow rate=21; tunnel leads to valve II\nan\n"
var reader = bufio.NewReader(strings.NewReader(input))

type Node struct {
	name string
	rate int
}

var graph = make(map[string][]string)
var nodes = make(map[string]Node)

var dist = make(map[string]map[string]int)

func main() {
	parseRegex := regexp.MustCompile(`[A-Z]{2}|\d+`)
	nonEmptyNodes := make([]string, 0)
	for {
		line, ok := readLine()
		if !ok {
			break
		}
		// Valve [AA] has flow rate=[0]; tunnels lead to valves [DD], [II], [BB]
		tokens := parseRegex.FindAllString(line, -1)

		rate, err := strconv.Atoi(tokens[1])
		if err != nil {
			panic(err)
		}

		node := Node{
			name: tokens[0],
			rate: rate,
		}

		nodes[tokens[0]] = node
		graph[tokens[0]] = tokens[2:]

		if rate != 0 {
			nonEmptyNodes = append(nonEmptyNodes, tokens[0])
		}
	}

	nodeBits := make(map[string]uint16)
	for i, node := range nonEmptyNodes {
		nodeBits[node] = 1 << i
	}

	var permutations [][]string
	valuableNodesMap := make(map[string]bool)

	// calculate distance between valuable nodes
	dijkstra(append(nonEmptyNodes, "AA"))

	for i := range nonEmptyNodes {
		valuableNodesMap[nonEmptyNodes[i]] = true
	}

	// get all permutations for viable paths
	permutations = fuck(valuableNodesMap, make([]string, 0))

	fmt.Println("Starting permutations")

	result := make(map[uint16]uint16)
	for i := range permutations {
		pressure := calcPressure(permutations[i], timeLimit)
		// minor optimization
		if pressure < 700 {
			continue
		}

		key := uint16(0)
		for j := range permutations[i] {
			key |= nodeBits[permutations[i][j]]
		}

		if uint16(pressure) > result[key] {
			result[key] = uint16(pressure)
		}
	}

	maxSum := uint16(0)
	max := uint16(0)

	for key1, v1 := range result {
		if v1 > max {
			max = v1
		}
		for key2, v2 := range result {
			if key1&key2 == 0 {
				if v2+v1 > maxSum {
					maxSum = v1 + v2
				}
			}
		}
	}

	fmt.Println(max, maxSum)
}

func calcPressure(path []string, limit int) int {
	pressurePerMinute := 0
	pressure := 0
	time := 0
	for i, valve := range path {
		if pressurePerMinute == 0 {
			time += dist["AA"][valve] + 1
		} else {
			timeSpent := dist[path[i-1]][valve] + 1
			pressure += pressurePerMinute * timeSpent

			time += timeSpent
		}

		pressurePerMinute += nodes[valve].rate
	}

	pressure += pressurePerMinute * (limit - time)

	return pressure
}

func checkLen(arr []string) int {
	if len(arr) < 1 {
		return 0
	}

	pathLen := dist["AA"][arr[len(arr)-1]]

	for j := len(arr) - 2; j >= 0; j-- {
		pathLen += dist[arr[j+1]][arr[j]]
	}

	return pathLen
}

func fuck(input map[string]bool, path []string) [][]string {
	res := make([][]string, 0)
	var helper func(input map[string]bool, path []string) bool

	helper = func(input map[string]bool, path []string) bool {
		// 26 for part 2, 30 for part 1
		if checkLen(path) < timeLimit {
			tmp := make([]string, len(path))
			copy(tmp, path)
			res = append(res, tmp)
		} else {
			return false
		}

		path = append(path, "")

		tmp := make(map[string]bool)
		for valve := range input {
			tmp[valve] = true
		}

		for valve := range tmp {
			path[len(path)-1] = valve
			delete(tmp, valve)
			if helper(tmp, path) {
				tmp[valve] = true
			}
		}

		return true
	}

	helper(input, path)
	return res
}

func dijkstra(sources []string) {
	for i := range sources {
		source := sources[i]
		distanceMap := make(map[string]int)
		sptSet := make(map[string]bool)

		for key := range graph {
			distanceMap[key] = math.MaxInt
			sptSet[key] = false
		}

		distanceMap[source] = 0

		for range graph {
			min := minDistance(distanceMap, sptSet)

			sptSet[min] = true

			for _, adjacent := range graph[min] {
				if !sptSet[adjacent] && distanceMap[adjacent] > distanceMap[min]+1 {
					distanceMap[adjacent] = distanceMap[min] + 1
				}
			}
		}

		dist[source] = make(map[string]int)
		for neighbour, distance := range distanceMap {
			if nodes[neighbour].rate != 0 {
				dist[source][neighbour] = distance
			}
		}
	}
}

func minDistance(distanceMap map[string]int, sptSet map[string]bool) string {
	min := math.MaxInt
	var result string
	for node, distance := range distanceMap {
		if distance < min && !sptSet[node] {
			min = distance
			result = node
		}
	}

	return result
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
