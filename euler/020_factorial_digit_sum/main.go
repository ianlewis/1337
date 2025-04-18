// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//nolint:all // TODO(#28): Fix golangci-lint issues.
package main

import (
	"fmt"
	"math/big"
	"strconv"
)

func factorial(n int64) *big.Int {
	f := big.NewInt(1)
	for ; n > 0; n-- {
		f = f.Mul(f, big.NewInt(n))
	}
	return f
}

func main() {
	sum := 0
	str := factorial(int64(100)).String()
	for _, r := range str {
		d, err := strconv.Atoi(string(r))
		if err != nil {
			panic(err)
		}
		sum += d
	}
	fmt.Println(sum)
}
