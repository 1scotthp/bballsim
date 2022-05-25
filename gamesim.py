# This is a sample Python script.

# Press ⌃R to execute it or replace it with your code.
# Press Double ⇧ to search everywhere for classes, files, tool windows, actions, and settings.

from cmath import nan
from multiprocessing.sharedctypes import Value
from operator import concat
from random import randrange

import numpy
import pandas as pd

TRANSITION_MAX = 6


## need a schema for possession out come returns



# class Player:
#     def __init__(self, name, age, blk, stl, twoFGp, twoFGrate, threeFGp, threeFGrate, to):
#         self.name = name
#         self.age = age
#         self.blk = blk
#         self.stl = stl
#         self.to = to
#         self.twoFGp = twoFGp
#         self.twoFGrate = twoFGrate
#         self.threeFGp = threeFGp
#         self.threeFGrate = threeFGrate


# p1 = Player("JOHN", 30, 2, 2, 40, 20, 30, 20, 10)
# p2 = Player("RICK", 30, 2, 2, 40, 20, 30, 20, 10)
# p3 = Player("BOB", 30, 2, 2, 40, 20, 30, 20, 10)
# p4 = Player("JORDAN", 30, 2, 2, 40, 20, 30, 20, 10)
# p5 = Player("YOU", 30, 2, 2, 40, 20, 30, 20, 10)
# p6 = Player("KC", 30, 2, 2, 40, 20, 30, 20, 10)
# p7 = Player("NATE", 30, 2, 2, 40, 20, 30, 20, 10)
# p8 = Player("SCOTT", 30, 2, 2, 40, 20, 30, 20, 10)
# p9 = Player("ANTONIO", 30, 2, 2, 40, 20, 30, 20, 10)
# p10 = Player("VIC", 30, 2, 2, 40, 20, 30, 20, 10)


def random_chance(p):
    return p > randrange(100)


# def random_ind_from_arr(prob_array):
#     # prob_array should add to 1

#     print("FIRST", prob_array)
#     num = randrange(100)
#     sum = 0
#     ind = 0

#     total = numpy.sum(prob_array)

#     prob_array = numpy.array([x/total for x in prob_array])
    

#     print("2nd", prob_array)
        



#     for i in prob_array:
#         sum += i
#         if sum > num:
#             return ind
#             # print("SHOT taken", ind)
#         ind += 1
#     return 5

import numpy as np

import random

def weighted_choice(objects, weights):
    """ returns randomly an element from the sequence of 'objects', 
        the likelihood of the objects is weighted according 
        to the sequence of 'weights', i.e. percentages."""

    weights = np.array(weights, dtype=np.float64)
    sum_of_weights = weights.sum()
    # standardization:x
    np.multiply(weights, 1 / sum_of_weights, weights)
    weights = weights.cumsum()
    x = random.random()
    for i in range(len(weights)):
        if x < weights[i]:
            return objects[i]




def team_turnover_odds(offRating, defRating):
    return (defRating - offRating)/4+30


# 1. based on TO rate of offensive and def team get likelihood of TO
# 2. Assign it to a random playtype based on likelihood (charge, steal, etc)
# 3. Assign it to an offensive player
# 4. Assign it to a defensive player
# returns (turnover, offPlayerTurnover, defPlayerTurnover, timeLeft)
def sim_turnover(home_on_court, away_on_court, timeLeft, possession, possessionStart):
    # get to rate of the 5 players
    # get forced to rate of 5 def players

    # home team with ball
    if possession:
        offToPer100 = sum(player for player in home_on_court["Tov/100"]) / 5
        defToPer100 = sum(player for player in away_on_court["Stl/100"]) / 5

    else:
        offToPer100 = sum(player for player in away_on_court["Tov/100"]) / 5
        defToPer100 = sum(player for player in home_on_court["Stl/100"]) / 5

    to_odds = team_turnover_odds(offToPer100, defToPer100)
    to_happened = random_chance(to_odds)

    # there was a turnover


    if to_happened:
        # find who committed turnover
        if possession:
            to_player = home_on_court['Player'].to_numpy()[randrange(5)]
            def_to_player = away_on_court["Player"].to_numpy()[randrange(5)]
            return to_happened, to_player, def_to_player, timeLeft - 1
        else:
            to_player = away_on_court["Player"].to_numpy()[randrange(5)]
            def_to_player = home_on_court['Player'].to_numpy()[randrange(5)]
            return to_happened, to_player, def_to_player, timeLeft - 1

    else:
        return False, None, None, timeLeft - 5


def sim_which_player_takes_shot(players):
    # total_usage = sum(player.twoFGrate for player in players)

    # prob_arr = map(lambda x: x.twoFGrate/total_usage, players)
    player_names = players["Player"].to_numpy()
    
    prob_arr = players["FGA/100"].to_numpy()

    return player_names[weighted_choice([0, 1, 2, 3, 4], prob_arr)]



### 
def get_possession_time(homeTeamPlayers, possessionStart):
    if random_chance(40):
        return 3
    elif random_chance(40):
        return 8
    else:
        return 15


def transition_possession(home_on_court, away_on_court, timeLeft, possession):
    to, offPlayerTurnover, defPlayerTurnover, timeLeft = sim_turnover(
        home_on_court, away_on_court, timeLeft, possession, "transition")

    # keep deciding

    if possession:
        offensive_players = home_on_court
    else:
        offensive_players = away_on_court

    shooter = sim_which_player_takes_shot(offensive_players)
    
    offensive_players = offensive_players.to_dict()
    # for i in offensive_players.keys():
    #     print(offensive_players[i])

    # for prob in offensive_players["FG2%"]:
    #     print(a[:-1])
    
    if random_chance(50):
        # print(shooter, " SHOOTS, HE SCORES")
        return "2", shooter
    else:
        # print(shooter, " SHOOTS, HE MISSES")
        return "0", shooter


LEAGUE_O_REB_3 = 0.3
LEAGUE_O_REB_2 = 0.2

darko_df = pd.read_csv("data/daily.csv")
darko_df["REBRATING_O"] = darko_df["daily_Minutes"] * darko_df["daily_Pace"] * darko_df["daily_OREB"]
darko_df["REBRATING_D"] = darko_df["daily_Minutes"] * darko_df["daily_Pace"] * darko_df["daily_DREB"]
print(darko_df)
### get darko stats and then do other stuff

#TODO this
def sim_rebound(offense, defense):
    offense = offense.to_dict()
    defense = defense.to_dict()

    off_names = []
    def_ratings = []

    # player_dict = {}
  
    for key in offense["Player"].keys():
        print(offense["Player"][key])
    

    # print(player_dict)
    d = darko_df.to_dict()

    for key in d.items():
        # print(key)
        if key[0] == "REBRATING_O":
            data = list(key[1].items())
            off_ratings = np.array(data)

    print(len(off_ratings))



    #### for each key we have, get that players o reb from darko

    
        # [off_ratings.append(player) for player in offense.items() if name == player]

    # for name in defense["Player"]:
    #     [def_ratings.append(player) for player in offense.items() if name == player]
    # print(off_ratings, def_ratings)
    


 




def half_court_possession(home_on_court, away_on_court, timeLeft, possession):
    ## before play type assign player

    play_type = get_play_type(home_on_court, away_on_court, timeLeft, possession)
    to, offPlayerTurnover, defPlayerTurnover, timeLeft = sim_turnover(home_on_court, away_on_court, timeLeft, possession, play_type)

     # turnover happened. start possession for other team
    if to:
        # print("Turnover by", offPlayerTurnover, "HOME: ", homeScore, "AWAY: ", awayScore, "TIME: ", timeLeft)
        return "turnover", offPlayerTurnover

    else:
        # get a shot off -> who shoots it
        
        if possession:
            offensive_players = home_on_court
            defensive_players = away_on_court
        else:
            offensive_players = away_on_court
            defensive_players = away_on_court
        shooter = sim_which_player_takes_shot(offensive_players)
        if random_chance(50):
            # print(shooter, " SHOOTS, HE SCORES")
            return "2", shooter
        else:
            # print(shooter, " SHOOTS, HE MISSES")
            sim_rebound(offensive_players, defensive_players)
            return "0", shooter
           



# given players choose play type distribution
def get_play_type(homeTeamPlayers, awayTeamPlayers, timeLeft, possession):
    play_type = "none"

# possession true if home team has it
def start_possession(homeTeamPlayers, awayTeamPlayers, timeLeft, homeScore, awayScore, possession, possessionStart):

    possessionLength = get_possession_time(homeTeamPlayers, possessionStart)

    if possessionLength < TRANSITION_MAX:
        # run transition possession
        outcome, player = transition_possession(homeTeamPlayers, awayTeamPlayers, timeLeft, possession)
    else:
        outcome, player = half_court_possession(homeTeamPlayers, awayTeamPlayers, timeLeft, possession)
        # run non transition possession
    return outcome, player, possessionLength
    
def get_real_players(team1, team2):
    df = pd.read_csv("darko.csv")

    t1 = pd.DataFrame([])
    t2 = pd.DataFrame([])

    for row, index in df.iterrows():
        if index["Team"] == team1:
            t1 = t1.append(index)
        if index["Team"] == team2:
            t2 = t2.append(index)
    return t1, t2


    # get a shot off

    # possible block

    # shot either goes in or does not

    # who gets rebound -> return result in the form of

    # (possessionTeam, awayTeamPoints, homeTeamPoints, timeLeft)


def damn_should_i_sub(team):
    team["Court Score"] =  team["DPM"] - team["TO"]
    return team.sort_values(by="Court Score").tail(5)

# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    homeScore = 0
    awayScore = 0
    possession = True

    # box_score = pd.DataFrame(columns=["NAME", "PTS", "REB", "AST", "BLK", "TO"], data=[
    #     [p1.name, 0, 0, 0, 0, 0],
    #     [p2.name, 0, 0, 0, 0, 0],
    #     [p3.name, 0, 0, 0, 0, 0],
    #     [p4.name, 0, 0, 0, 0, 0],
    #     [p5.name, 0, 0, 0, 0, 0],
    #     [p6.name, 0, 0, 0, 0, 0],
    #     [p7.name, 0, 0, 0, 0, 0],
    #     [p8.name, 0, 0, 0, 0, 0],
    #     [p9.name, 0, 0, 0, 0, 0],
    #     [p10.name, 0, 0, 0, 0, 0],
    # ])

    teams = ["Milwaukee Bucks", "Phoenix Suns", "Dallas Mavericks", "Golden State Warriors", "Miami Heat", "Boston Celtics", "Utah Jazz", "Memphis Grizzlies"]

    def generate_matchups(n):
        matchups = []
        while n > 0:
            matchups.append([teams[randrange(8)], teams[randrange(8)]])
            n-=1
        return matchups

    games = generate_matchups(10)

    

    for home_name, away_name in games:

        home, away= get_real_players(home_name, away_name)

        box_score = pd.concat([home, away])

    
        box_score.insert(0, "PTS", 0, True)
        box_score.insert(14, "SEC", 0, True)
        box_score.insert(0, "TO", 0, True)
        home.insert(0, "PTS", 0, True)
        home.insert(0, "TO", 0, True)
        away.insert(0, "PTS", 0, True)
        away.insert(0, "TO", 0, True)

        second_dict = {}

        for name in box_score["Player"]:
            second_dict[name] = 0
        

        home_on_court = home.head(5)
        away_on_court = away.head(5)

        timeLeft = 600
        quarter = 0

        while quarter < 4:
            while timeLeft > 0:
                outcome, player, possessionLength = start_possession(home_on_court,away_on_court, timeLeft, homeScore, awayScore, possession, "rebound")
                timeLeft -= possessionLength

                for name in home_on_court["Player"]:
                    second_dict[name] += possessionLength

                for name in away_on_court["Player"]:
                    second_dict[name] += possessionLength


                if outcome == "turnover":
                    # increment the box score
                    if possession:
                        home.loc[home["Player"] == player, 'TO'] += 1
                    else:
                        away.loc[away["Player"] == player, 'TO'] += 1
                    box_score.loc[box_score["Player"] == player, 'TO'] += 1
                elif outcome == "2":
                    if possession:
                        homeScore += 2
                    else:
                        awayScore += 2
                    box_score.loc[box_score["Player"] == player, 'PTS'] += 2
                elif outcome == "3":
                    if possession:
                        homeScore += 3
                    else:
                        awayScore += 3
                    box_score.loc[box_score["Player"] == player, 'PTS'] += 3

                possession = not possession
                if random_chance(50):
                    home_on_court = damn_should_i_sub(home)
                    away_on_court = damn_should_i_sub(away)
            
            quarter+=1
            timeLeft = 600

        # box_score.insert(0, "MIN", 0, True)

        for name in box_score["Player"]:
            box_score.loc[box_score["Player"] == name, "MIN"] = round(second_dict[name] / 60)
            
  
        print(box_score)
        box_score.to_csv(f"box_scores/{home_name} vs {away_name}.csv")

        
        home = home.iloc[0:0]
        away = away.iloc[0:0]
        box_score = box_score.iloc[0:0]




##TODO ADD THREES


     
 
        

# See PyCharm help at https://www.jetbrains.com/help/pycharm/







# team class
# player class









# # based on usage assign the shot to a player
# # based on player tendencies, assign shot type
# # returns (shotTaker, timeLeft)
# def sim_take_shot(homeTeamPlayers, awayTeamPlayers, timeLeft, homeScore, awayScore):
#
# # 1. each person has a percentage of their 3s and 2s that get blocked
# # 2. use this and the defense's block rate to see if there was a block
# # 3. if there was a block assign it to a player
# # returns (block, blocker, timeLeft)
# def sim_block(shooter, oppPlayers):
#
#
# # based on shooter and opponent FG% against assign shot outcome
# # returns (make, timeLeft, homeScore, awayScore)
# def sim_shot_outcome(shooter, shotType, oppPlayers, timeLeft, homeScore, awayScore):
#
#
# # get team rebounding rate from players
# # sim team rebound outcome
# # assign to a player
# # returns (team, player, homeScore, awayScore, timeLeft)
# def sim_rebound(homeTeamPlayers, awayTeamPlayers):