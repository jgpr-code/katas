#let chance(dices) = {
  let sum = 0
  for d in dices {
    sum += d
  }
  [#sum]
}

#let yatzy(dices) = {
  let first = dices.at(0)
  if dices.all(d => d == first) {
    [50]
  } else {
    [0]
  }
}

#let numeric(dices, category, numeric_dict) = {
  [#dices.filter(d => d == numeric_dict.at(category)).len()]
}

#let count_dices(dices) = {
  let counts = (0,0,0,0,0,0)
  for d in dices {
    counts.at(d - 1) += 1
  }
  return counts
}

#let pair(dices) = {
  let counts = count_dices(dices)
  let pos = counts.rev().position(e => e >= 2)
  if pos == none {
    [0]
  } else {
    let answer = (6 - pos) * 2
    [#answer]
  }
}

#let two_pair(dices) = {
  let counts = count_dices(dices)
  let at_least_two = counts.enumerate().filter(n => n.at(1) >= 2)
  let len = at_least_two.len()
  if len >= 2 {
    let (first_idx, _) = at_least_two.at(len - 1)
    let (second_idx, _) = at_least_two.at(len - 2)
    let answer = (first_idx + 1) * 2 + (second_idx + 1) * 2
    [#answer]
  } else {
    [0]
  }
  // let pos = counts.rev().position(e => e >= 2)
}

#let play_yatzy(dices, category) = {
  assert.eq(dices.len(), 5)
  let numeric_dict = (
    "ones": 1,
    "twos": 2,
    "threes": 3,
    "fours": 4,
    "fives": 5,
    "sixes": 6,
  )
  if category == "chance" {
    chance(dices)
  }
  else if category == "yatzy" {
    yatzy(dices)
  }
  else if category in numeric_dict {
    numeric(dices, category, numeric_dict)
  }
  else if category == "pair" {
    pair(dices)
  }
  else if category == "two_pair" {
    two_pair(dices)
  }
  else {
    category + " is unimplemented"
  }
}

#let yatzy_answer(dices, category) = {
  ["#category" for dices #dices = ]
  play_yatzy(dices, category)
  [\ ]
}

#let test_yatzy(dices, category, answer) = {
  assert.eq(play_yatzy(dices, category), answer)
  yatzy_answer(dices, category)
}

= Yatzy Unit Test Report

#test_yatzy((1,2,3,4,5), "chance", [15])
#test_yatzy((6,6,6,6,6), "chance", [30])
#test_yatzy((2,2,2,2,2), "yatzy", [50])
#test_yatzy((1,1,1,1,1), "yatzy", [50])
#test_yatzy((1,1,1,1,2), "yatzy", [0])
#test_yatzy((1,1,2,3,4), "ones", [2])
#test_yatzy((1,1,1,2,6), "ones", [3])
#test_yatzy((2,2,2,2,3), "twos", [4])
#test_yatzy((3,4,5,6,6), "threes", [1])
#test_yatzy((3,4,5,6,6), "threes", [1])
#test_yatzy((3,4,5,6,6), "fives", [1])
#test_yatzy((3,4,5,6,6), "sixes", [2])
#test_yatzy((2,2,2,2,2), "ones", [0])
#test_yatzy((1,2,3,4,5), "pair", [0])
#test_yatzy((3,3,4,4,6), "pair", [8])
#test_yatzy((3,3,5,5,5), "pair", [10])
#test_yatzy((3,3,3,4,5), "two_pair", [0])
#test_yatzy((3,3,4,4,6), "two_pair", [14])
