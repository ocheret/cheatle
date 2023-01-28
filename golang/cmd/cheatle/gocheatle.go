package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/ocheret/cheatle/golang/pkg/cheatle"
)

func prompt() {
	fmt.Print("cheatle> ")
}

func printHelp() {
	fmt.Println("reset : start a new cheating session")
	fmt.Println("- <LETTERS> : letters not in solution")
	fmt.Println("-N <LETTER> : letter in solution but not in Nth position")
	fmt.Println("+N <LETTER> : letter is in Nth position")
	fmt.Println(">=N <LETTER>: letter occurs at least N time in word")
	fmt.Println("list : list remaining solutions")
	fmt.Println("help : this messsage")
}

func main() {
	defer fmt.Println("\nBye!")
	chtl := cheatle.NewCheatle()
	scanner := bufio.NewScanner(os.Stdin)
	prompt()
	for scanner.Scan() {
		line := scanner.Text()
		line = strings.ToLower(line)
		tokens := strings.Fields(line)
		if len(tokens) == 0 || tokens[0] == "" {
			prompt()
			continue
		}
		command := tokens[0]
		if command == "reset" {
			chtl.Reset()
		} else if command == "help" {
			printHelp()
		} else if command == "-" {
			if len(tokens) < 2 {
				printHelp()
				continue
			}
			for _, r := range tokens[1] {
				chtl.NotInWord(r)
			}
		} else if command == "list" {
			chtl.Filter()
			words := chtl.GetWords()
			fmt.Println(strings.Join(words, "\n"))
			fmt.Printf("count: %d\n", len(words))
		} else if strings.HasPrefix(command, ">=") {
			if len(tokens) < 2 {
				printHelp()
				continue
			}
			c := []rune(command)
			count, error := strconv.Atoi(string(c[2]))
			if error != nil || count < 1 || count > 5 {
				fmt.Println("Invalid count (must be 1 thru 5")
				continue
			}
			chtl.SetMinOccurences([]rune(tokens[1])[0], count)
		} else {
			c := []rune(command)
			if len(c) == 2 {
				var f func(*cheatle.Cheatle, int, rune)
				switch c[0] {
				case '-':
					f = (*cheatle.Cheatle).MisplacedInWord
				case '+':
					f = (*cheatle.Cheatle).PlacedInWord
				default:
					print("Invalid command")
					continue
				}
				pos, error := strconv.Atoi(string(c[1]))
				if error != nil || pos < 1 || pos > 5 {
					fmt.Println("Invalid position (must be 1 thru 5)")
					continue
				}
				// User enters 1 thru 5 but we want 0 thru 4
				pos--
				f(chtl, pos, []rune(tokens[1])[0])
			} else {
				fmt.Println("Invalid command")
				printHelp()
			}
		}
		prompt()
	}
}
