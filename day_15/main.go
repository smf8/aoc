package main

import (
	"strings"
	"regexp"
	"strconv"
	"bufio"
	"math"
	"sort"
	"fmt"
	"math/big"
)

type Beacon struct {
	X, Y int
}

type Sensor struct {
	X, Y   int
	Beacon Beacon
}

type Interval struct {
	Start, End int
}

//var input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16\nSensor at x=13, y=2: closest beacon is at x=15, y=3\nSensor at x=12, y=14: closest beacon is at x=10, y=16\nSensor at x=10, y=20: closest beacon is at x=10, y=16\nSensor at x=14, y=17: closest beacon is at x=10, y=16\nSensor at x=8, y=7: closest beacon is at x=2, y=10\nSensor at x=2, y=0: closest beacon is at x=2, y=10\nSensor at x=0, y=11: closest beacon is at x=2, y=10\nSensor at x=20, y=14: closest beacon is at x=25, y=17\nSensor at x=17, y=20: closest beacon is at x=21, y=22\nSensor at x=16, y=7: closest beacon is at x=15, y=3\nSensor at x=14, y=3: closest beacon is at x=15, y=3\nSensor at x=20, y=1: closest beacon is at x=15, y=3\nan\n"
var input = "Sensor at x=98246, y=1908027: closest beacon is at x=1076513, y=2000000\nSensor at x=1339369, y=2083853: closest beacon is at x=1076513, y=2000000\nSensor at x=679177, y=3007305: closest beacon is at x=1076513, y=2000000\nSensor at x=20262, y=3978297: closest beacon is at x=13166, y=4136840\nSensor at x=3260165, y=2268955: closest beacon is at x=4044141, y=2290104\nSensor at x=2577675, y=3062584: closest beacon is at x=2141091, y=2828176\nSensor at x=3683313, y=2729137: closest beacon is at x=4044141, y=2290104\nSensor at x=1056412, y=370641: closest beacon is at x=1076513, y=2000000\nSensor at x=2827280, y=1827095: closest beacon is at x=2757345, y=1800840\nSensor at x=1640458, y=3954524: closest beacon is at x=2141091, y=2828176\nSensor at x=2139884, y=1162189: closest beacon is at x=2757345, y=1800840\nSensor at x=3777450, y=3714504: closest beacon is at x=3355953, y=3271922\nSensor at x=1108884, y=2426713: closest beacon is at x=1076513, y=2000000\nSensor at x=2364307, y=20668: closest beacon is at x=2972273, y=-494417\nSensor at x=3226902, y=2838842: closest beacon is at x=3355953, y=3271922\nSensor at x=22804, y=3803886: closest beacon is at x=13166, y=4136840\nSensor at x=2216477, y=2547945: closest beacon is at x=2141091, y=2828176\nSensor at x=1690953, y=2203555: closest beacon is at x=1076513, y=2000000\nSensor at x=3055156, y=3386812: closest beacon is at x=3355953, y=3271922\nSensor at x=3538996, y=719130: closest beacon is at x=2972273, y=-494417\nSensor at x=2108918, y=2669413: closest beacon is at x=2141091, y=2828176\nSensor at x=3999776, y=2044283: closest beacon is at x=4044141, y=2290104\nSensor at x=2184714, y=2763072: closest beacon is at x=2141091, y=2828176\nSensor at x=2615462, y=2273553: closest beacon is at x=2757345, y=1800840\nan\n"
var reader = bufio.NewReader(strings.NewReader(input))

func main() {
	regex := regexp.MustCompile(`[\-0-9]+`)
	sensors := make([]Sensor, 0)
	for {
		line, ok := readLine()
		if !ok {
			break
		}

		data := regex.FindAllString(line, 4)

		sensorX, _ := strconv.Atoi(data[0])
		sensorY, _ := strconv.Atoi(data[1])
		beaconX, _ := strconv.Atoi(data[2])
		beaconY, _ := strconv.Atoi(data[3])

		sensors = append(sensors, Sensor{
			X: sensorX,
			Y: sensorY,
			Beacon: Beacon{
				X: beaconX,
				Y: beaconY,
			},
		})
	}

	// for part one simply call rowIntervals(10) and count interval lengths

	for i := 0; i <= 4000000; i++ {
		IthInterval := rowIntervals(i, sensors)
		if len(IthInterval) != 1 {
			mul := big.NewInt(0).Mul(big.NewInt(int64(IthInterval[0].End+1)), big.NewInt(4000000))
			add := mul.Add(mul, big.NewInt(int64(i)))
			fmt.Printf("%s", add.String())
		}
	}

}

func rowIntervals(row int, sensors []Sensor) []Interval {
	intervals := make([]Interval, 0)
	for _, sensor := range sensors {
		distance := math.Abs(float64(sensor.X-sensor.Beacon.X)) + math.Abs(float64(sensor.Y-sensor.Beacon.Y))

		verticalDistance := math.Abs(float64(row) - float64(sensor.Y))

		lineLength := int(math.Max(distance-verticalDistance, 0))

		if lineLength != 0 {
			interval := Interval{
				Start: sensor.X - lineLength,
				End:   sensor.X + lineLength,
			}

			intervals = append(intervals, interval)
		}
	}

	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i].Start < intervals[j].Start
	})

	for i := 0; i < len(intervals)-1; i++ {
		if intervals[i].End >= intervals[i+1].Start-1 {
			//overlapping intervals

			if intervals[i+1].End > intervals[i].End {
				intervals[i].End = intervals[i+1].End
			}

			// remove second interval
			intervals = append(intervals[:i+1], intervals[i+2:]...)
			i--
			continue
		}
	}

	return intervals
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
