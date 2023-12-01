package main

import (
	"strings"
	"bufio"
	"strconv"
	"fmt"
)

//var input = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1\nan\n"
var input = "Monkey 0:\n  Starting items: 63, 84, 80, 83, 84, 53, 88, 72\n  Operation: new = old * 11\n  Test: divisible by 13\n    If true: throw to monkey 4\n    If false: throw to monkey 7\n\nMonkey 1:\n  Starting items: 67, 56, 92, 88, 84\n  Operation: new = old + 4\n  Test: divisible by 11\n    If true: throw to monkey 5\n    If false: throw to monkey 3\n\nMonkey 2:\n  Starting items: 52\n  Operation: new = old * old\n  Test: divisible by 2\n    If true: throw to monkey 3\n    If false: throw to monkey 1\n\nMonkey 3:\n  Starting items: 59, 53, 60, 92, 69, 72\n  Operation: new = old + 2\n  Test: divisible by 5\n    If true: throw to monkey 5\n    If false: throw to monkey 6\n\nMonkey 4:\n  Starting items: 61, 52, 55, 61\n  Operation: new = old + 3\n  Test: divisible by 7\n    If true: throw to monkey 7\n    If false: throw to monkey 2\n\nMonkey 5:\n  Starting items: 79, 53\n  Operation: new = old + 1\n  Test: divisible by 3\n    If true: throw to monkey 0\n    If false: throw to monkey 6\n\nMonkey 6:\n  Starting items: 59, 86, 67, 95, 92, 77, 91\n  Operation: new = old + 5\n  Test: divisible by 19\n    If true: throw to monkey 4\n    If false: throw to monkey 0\n\nMonkey 7:\n  Starting items: 58, 83, 89\n  Operation: new = old * 19\n  Test: divisible by 17\n    If true: throw to monkey 2\n    If false: throw to monkey 1\nan\n"
var reader = bufio.NewReader(strings.NewReader(input))

var an = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19

//var an = 23 * 19 * 13 * 17

type Monkey struct {
	id           int
	operandOne   int
	operandTwo   int
	operation    string
	testDividend int

	inspections int

	items []int

	trueMonkeyNumber  *Monkey
	falseMonkeyNumber *Monkey
}

func (m *Monkey) Inspect() {
	for i := range m.items {
		operandOne := m.operandOne
		operandTwo := m.operandTwo

		// if operand is "old"
		if m.operandOne == -1 {
			operandOne = m.items[i]
		}

		if m.operandTwo == -1 {
			operandTwo = m.items[i]
		}

		newWorryLevel := 0
		if m.operation == "*" {
			newWorryLevel = operandOne * operandTwo
		} else {
			newWorryLevel = operandOne + operandTwo
		}

		newWorryLevel %= an

		if newWorryLevel%m.testDividend == 0 {
			m.trueMonkeyNumber.items = append(m.trueMonkeyNumber.items, newWorryLevel)
		} else {
			m.falseMonkeyNumber.items = append(m.falseMonkeyNumber.items, newWorryLevel)
		}

		m.inspections++
	}

	m.items = make([]int, 0)
}

func main() {
	monkeys := make([]Monkey, 8)
	counter := 0
	for {
		line, ok := readLine()
		if !ok {
			break
		}

		// parse first line
		if strings.Contains(line, "Monkey") {
			id, err := strconv.Atoi(string(line[len(line)-2]))
			if err != nil {
				panic(err)
			}

			monkeys[counter].id = id
			continue
		}

		if strings.Contains(line, "Starting") {
			line = strings.TrimPrefix(line, "  Starting items: ")
			numbersString := strings.Split(line, ",")
			items := make([]int, 0)
			for i := range numbersString {
				numberString := strings.TrimSpace(numbersString[i])

				number, err := strconv.Atoi(numberString)
				if err != nil {
					panic(err)
				}

				items = append(items, number)
			}

			monkeys[counter].items = items
			continue
		}

		if strings.Contains(line, "Operation") {
			line = strings.TrimPrefix(line, "  Operation: new = old ")

			monkeys[counter].operandOne = -1
			monkeys[counter].operation = string(line[0])

			operandTwo, err := strconv.Atoi(line[2:])
			if err != nil {
				monkeys[counter].operandTwo = -1
			} else {
				monkeys[counter].operandTwo = operandTwo
			}

			continue
		}

		if strings.Contains(line, "Test") {
			line = strings.TrimPrefix(line, "  Test: divisible by ")

			testDividend, err := strconv.Atoi(line)
			if err != nil {
				panic(err)
			}

			monkeys[counter].testDividend = testDividend

			continue
		}

		if strings.Contains(line, "If") {
			_, afterString, _ := strings.Cut(line, "monkey ")

			after, err := strconv.Atoi(afterString)
			if err != nil {
				panic(err)
			}

			if strings.Contains(line, "true") {
				monkeys[counter].trueMonkeyNumber = &monkeys[after]
			} else {
				monkeys[counter].falseMonkeyNumber = &monkeys[after]
				counter++
			}
			continue
		}
	}

	for j := 0; j < 10000; j++ {
		for i := 0; i < counter; i++ {
			monkeys[i].Inspect()
		}
	}

	max1, max2 := 0, 0
	for i := 0; i < counter; i++ {
		if monkeys[i].inspections > max1 {
			max2 = max1
			max1 = monkeys[i].inspections
		} else if monkeys[i].inspections > max2 {
			max2 = monkeys[i].inspections
		}
	}

	fmt.Println(max1 * max2)
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
