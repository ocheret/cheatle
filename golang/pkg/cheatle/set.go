package cheatle

type Set[T comparable] struct {
	// empty structs occupy no memory
	items map[T]struct{}
}

func NewSet[T comparable]() *Set[T] {
	s := Set[T]{}
	s.Clear()
	return &s
}

func (s *Set[T]) Has(v T) bool {
	_, ok := s.items[v]
	return ok
}

func (s *Set[T]) Add(v T) *Set[T] {
	s.items[v] = struct{}{}
	return s
}

func (s *Set[T]) Remove(v T) *Set[T] {
	delete(s.items, v)
	return s
}

func (s *Set[T]) Clear() *Set[T] {
	s.items = make(map[T]struct{})
	return s
}

func (s *Set[T]) Size() int {
	return len(s.items)
}

// AddMulti Add multiple values in the set
func (s *Set[T]) AddMulti(list ...T) *Set[T] {
	for _, v := range list {
		s.Add(v)
	}
	return s
}

type FilterFunc[T comparable] func(v T) bool

// Filter returns a subset, that contains only the values that satisfies the given predicate P
func (s *Set[T]) Filter(P FilterFunc[T]) *Set[T] {
	res := NewSet[T]()
	for v := range s.items {
		if P(v) {
			res.Add(v)
		}
	}
	return res
}

func (s *Set[T]) Union(s2 *Set[T]) *Set[T] {
	res := NewSet[T]()
	for v := range s.items {
		res.Add(v)
	}

	for v := range s2.items {
		res.Add(v)
	}
	return res
}

func (s *Set[T]) Intersect(s2 *Set[T]) *Set[T] {
	res := NewSet[T]()
	for v := range s.items {
		if s2.Has(v) {
			res.Add(v)
		}
	}
	return res
}

// Difference returns the subset from s, that doesn't exists in s2 (param)
func (s *Set[T]) Difference(s2 *Set[T]) *Set[T] {
	res := NewSet[T]()
	for v := range s.items {
		if !s2.Has(v) {
			res.Add(v)
		}
	}
	return res
}

func (s *Set[T]) ToSlice() []T {
	result := []T{}
	for k := range s.items {
		result = append(result, k)
	}
	return result
}
