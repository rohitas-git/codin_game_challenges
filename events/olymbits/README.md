# Summer Challenge 2024 with Fiverr - Olymbits

## Wooden League Division 1:

### The reason why safest approach failed?

Answer: Because winner has highest final score
'The scores for all four mini-games are multiplied together to determine the final score.' & mini_game_score = nb_silver_medals + nb_gold_medals * 3

Following this approach, I get more gold and bronze. Thus, lowering my mini-game score and greatly impacting final score. 
I observed that in some mini-game if I have zero score that is all bronze then final score is zero.

It would better to earn enough but necessarily highest in each mini-game, so that final score is highest.
Instead of focusing on one run of mini-game