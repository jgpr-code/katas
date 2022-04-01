# Texas Hold'em Poker

n players with up to 7 cards
The first 2 cards are unique per player, the last 5 are the same for everyone
Cards:
Clubs (C), Diamonds (D), Hearts (H), Spades (S)
2, 3, 4, 5, 6, 7, 8, 9, 10, Joker (J), Queen (Q), King (K), Ace (A)
Example input:
KC 9S KS KD 9D 3C 6D
9C AH KS KD 9D 3C 6D
AC QC KS KD 9D 3C
9H 5S
4D 2D KS KD 9D 3C 6D
7S 10S KS KD 9D

Task: What “hands” do the player have and which player wins?

High Card: Player with the highest card wins. On draw, player with the second highest card wins
Pair: 2 cards with the same “numerical” value. In case of a draw: higher pair wins, afterwards higher high card wins
Two Pairs: 2 pairs. In case of draw: higher “high” pair wins, then higher “low” pair, then high card
Three of a Kind: 3 cards with the same numerical value.
Straight: 5 cards with sequential numerical value
Flush: 5 cards of the same suit
Full House: Three of a kind and a pair
Four of a Kind: 4 cards with the same numerical value
Straight Flush: 5 cards with sequential numerical value and the same suit
Royal Flush: 5 cards with sequential numerical value and the same suit with the high card Ace
Example input:
KC 9S KS KD 9D 3C 6D (Full House (K, K, K, 9, 9), winner)
9C AH KS KD 9D 3C 6D (Two Pair (K, K, 9, 9))
AC QC KS KD 9D 3C
9H 5S
4D 2D KS KD 9D 3C 6D (Flush, 5 diamonds)
7S 10S KS KD 9D
