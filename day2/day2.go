package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
	}
	defer file.Close()

	pt1, pt2 := solution(file)

	fmt.Println("Answer for PART 1:", pt1)
	fmt.Println("Answer for PART 2:", pt2)

}

func solution(file *os.File) (int, int) {
	aim, horizontal, vertical := 0, 0, 0
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		values := strings.Fields(scanner.Text())
		direction := values[0]
		value, _ := strconv.Atoi(values[1])

		if direction == "forward" {
			horizontal += value
			vertical += (aim * value)
		} else if direction == "up" {
			aim -= value
		} else if direction == "down" {
			aim += value
		}
	}

	return horizontal * aim, horizontal * vertical
}
