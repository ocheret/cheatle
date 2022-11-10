package cheatle

import (
	_ "embed"
	"regexp"
	"strings"
)

//go:embed fivechars.txt
var dictionary string

var words []string

const alphabet = "[abcdefghijklmnopqrstuvwxyz]"

func init() {
	// Get the complete list of words in the dictionary
	words = strings.Split(strings.Trim(dictionary, "\n"), "\n")
}

type Cheatle struct {
	remaining []string
	required  *Set[rune]
	indices   []int
}

func (c *Cheatle) Reset() {
	c.remaining = make([]string, 5)
	for i := 0; i < 5; i++ {
		c.remaining[i] = strings.Clone(alphabet)
	}
	c.required = NewSet[rune]()
	c.indices = make([]int, 0, len(words))
	for i := range words {
		c.indices = append(c.indices, i)
	}
}

func NewCheatle() *Cheatle {
	c := Cheatle{}
	c.Reset()
	return &c
}

func (c *Cheatle) RemoveLeter(pos int, letter rune) {
	c.remaining[pos] =
		strings.Replace(c.remaining[pos], string(letter), "", 1)
}

func (c *Cheatle) NotInWord(letter rune) {
	for pos := 0; pos < 5; pos++ {
		c.RemoveLeter(pos, letter)
	}
}

func (c *Cheatle) PlacedInWord(pos int, letter rune) {
	c.remaining[pos] = string(letter)
	if c.required.Has(letter) {
		c.required.Remove(letter)
	}
}

func (c *Cheatle) MisplacedInWord(pos int, letter rune) {
	c.RemoveLeter(pos, letter)
	c.required.Add(letter)
}

func (c *Cheatle) HasRequired(word string) bool {
	req := c.required.ToSlice()
	for _, r := range req {
		if !strings.ContainsRune(word, r) {
			return false
		}
	}
	return true
}

func (c *Cheatle) Filter() {
	expr := strings.Join(c.remaining, "")
	prog := regexp.MustCompile(expr)
	filtered := make([]int, 0)
	for _, index := range c.indices {
		w := words[index]
		if prog.MatchString(w) && c.HasRequired(w) {
			filtered = append(filtered, index)
		}
	}
	c.indices = filtered
}

func (c *Cheatle) GetWords() []string {
	result := make([]string, 0, len(c.indices))
	for _, index := range c.indices {
		result = append(result, words[index])
	}
	return result
}
