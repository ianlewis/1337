// Copyright 2025 Ian Lewis
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Package besttimetobuyandsellstock implements LeetCode problem 121. Best Time
// to Buy and Sell Stock.
//
// You are given an array prices where prices[i] is the price of a given stock
// on the ith day.
//
// You want to maximize your profit by choosing a single day to buy one stock
// and choosing a different day in the future to sell that stock.
//
// Return the maximum profit you can achieve from this transaction. If you
// cannot achieve any profit, return 0.
//
// Example 1:
//
//	Input: prices = [7,1,5,3,6,4]
//	Output: 5
//	Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6),
//	profit = 6-1 = 5.
//	Note that buying on day 2 and selling on day 1 is not allowed because you
//	must buy before you sell.
//
// Example 2:
//
//	Input: prices = [7,6,4,3,1]
//	Output: 0
//	Explanation: In this case, no transactions are done and the max profit = 0.
package besttimetobuyandsellstock

import "math"

func maxProfit(prices []int) int {
	var maxProfit int

	minBuy := math.MaxInt32

	for i := range prices {
		if prices[i] < minBuy {
			minBuy = prices[i]
		}

		profit := prices[i] - minBuy

		if profit > maxProfit {
			maxProfit = profit
		}
	}

	return maxProfit
}
