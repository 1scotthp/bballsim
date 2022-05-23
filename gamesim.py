# This is a sample Python script.

# Press ⌃R to execute it or replace it with your code.
# Press Double ⇧ to search everywhere for classes, files, tool windows, actions, and settings.
from ast import Constant
from random import randrange
import pandas as pd

TRANSITION_MAX = 6


class Player:
    def __init__(self, name, age, blk, stl, twoFGp, twoFGrate, threeFGp, threeFGrate, to):
        self.name = name
        self.age = age
        self.blk = blk
        self.stl = stl
        self.to = to
        self.twoFGp = twoFGp
        self.twoFGrate = twoFGrate
        self.threeFGp = threeFGp
        self.threeFGrate = threeFGrate


p1 = Player("JOHN", 30, 2, 2, 40, 20, 30, 20, 10)
p2 = Player("RICK", 30, 2, 2, 40, 20, 30, 20, 10)
p3 = Player("BOB", 30, 2, 2, 40, 20, 30, 20, 10)
p4 = Player("JORDAN", 30, 2, 2, 40, 20, 30, 20, 10)
p5 = Player("YOU", 30, 2, 2, 40, 20, 30, 20, 10)
p6 = Player("KC", 30, 2, 2, 40, 20, 30, 20, 10)
p7 = Player("NATE", 30, 2, 2, 40, 20, 30, 20, 10)
p8 = Player("SCOTT", 30, 2, 2, 40, 20, 30, 20, 10)
p9 = Player("ANTONIO", 30, 2, 2, 40, 20, 30, 20, 10)
p10 = Player("VIC", 30, 2, 2, 40, 20, 30, 20, 10)


def random_chance(p):
    return p > randrange(100)


def random_ind_from_arr(prob_array):
    # prob_array should add to 100

    num = randrange(100)
    sum = 0
    ind = 0

    for i in prob_array:
        sum += i*100
        if sum > num:
            return ind
            # print("SHOT taken", ind)
        ind += 1
    return 5


def print_hi(name):
    # Use a breakpoint in the code line below to debug your script.
    print(f'Hi, {name}')  # Press ⌘F8 to toggle the breakpoint.


def team_turnover_odds(offRating, defRating):
    return (defRating - offRating)/4+30


# 1. based on TO rate of offensive and def team get likelihood of TO
# 2. Assign it to a random playtype based on likelihood (charge, steal, etc)
# 3. Assign it to an offensive player
# 4. Assign it to a defensive player
# returns (turnover, offPlayerTurnover, defPlayerTurnover, timeLeft)
def sim_turnover(homeTeamPlayers, awayTeamPlayers, timeLeft, possession, possessionStart):
    # get to rate of the 5 players
    # get forced to rate of 5 def players

    # home team with ball
    if possession:
        offToRating = sum(player.to for player in homeTeamPlayers)
        defToRating = sum(player.stl for player in awayTeamPlayers)

    else:
        offToRating = sum(player.to for player in homeTeamPlayers)
        defToRating = sum(player.stl for player in awayTeamPlayers)

    to_odds = team_turnover_odds(offToRating, defToRating)
    print("odds", to_odds)
    to_happened = random_chance(to_odds)

    # there was a turnover

    if to_happened:
        # find who committed turnover
        if possession:
            to_player = homeTeamPlayers[randrange(4)]
            def_to_player = awayTeamPlayers[randrange(4)]
            return to_happened, to_player, def_to_player, timeLeft - 1
        else:
            to_player = awayTeamPlayers[randrange(4)]
            def_to_player = homeTeamPlayers[randrange(4)]
            return to_happened, to_player, def_to_player, timeLeft - 1

    else:
        return False, None, None, timeLeft - 5


def sim_which_player_takes_shot(players):
    total_usage = sum(player.twoFGrate for player in players)

    prob_arr = map(lambda x: x.twoFGrate/total_usage, players)

    return players[random_ind_from_arr(prob_arr)]


def get_possession_time(homeTeamPlayers, possessionStart):
    if random_chance(40):
        return 3
    elif random_chance(40):
        return 8
    else:
        return 15


def transition_possession(homeTeamPlayers, awayTeamPlayers, timeLeft, possession):
    to, offPlayerTurnover, defPlayerTurnover, timeLeft = sim_turnover(
        homeTeamPlayers, awayTeamPlayers, timeLeft, possession, "transition")

    # keep deciding

    if possession:
        offensive_players = homeTeamPlayers
    else:
        offensive_players = awayTeamPlayers
    shooter = sim_which_player_takes_shot(offensive_players)
    if random_chance(50):
        print(shooter.name, " SHOOTS, HE SCORES")
        return "2", shooter.name
    else:
        print(shooter.name, " SHOOTS, HE MISSES")
        return "0", shooter.name


def half_court_possession(homeTeamPlayers, awayTeamPlayers, timeLeft, possession):

    play_type = get_play_type(homeTeamPlayers, awayTeamPlayers, timeLeft, possession)
    to, offPlayerTurnover, defPlayerTurnover, timeLeft = sim_turnover(homeTeamPlayers, awayTeamPlayers, timeLeft, possession, play_type)

     # turnover happened. start possession for other team
    if to:
        print("Turnover by", offPlayerTurnover.name, "HOME: ", homeScore, "AWAY: ", awayScore, "TIME: ", timeLeft)
        return "turnover", offPlayerTurnover.name

    else:
        # get a shot off -> who shoots it
        
        if possession:
            offensive_players = homeTeamPlayers
        else:
            offensive_players = awayTeamPlayers
        shooter = sim_which_player_takes_shot(offensive_players)
        if random_chance(50):
            print(shooter.name, " SHOOTS, HE SCORES")
            return "2", shooter.name
        else:
            print(shooter.name, " SHOOTS, HE MISSES")
            return "0", shooter.name
           



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
    


# interlaken 190
# EU PASS 7 day 274
#






    # get a shot off

    # possible block

    # shot either goes in or does not

    # who gets rebound -> return result in the form of

    # (possessionTeam, awayTeamPoints, homeTeamPoints, timeLeft)


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    print_hi('PyCharm')
    homeScore = 0
    awayScore = 0
    possession = True

    box_score = pd.DataFrame(columns=["NAME", "PTS", "REB", "AST", "BLK", "TO"], data=[
        [p1.name, 0, 0, 0, 0, 0],
        [p2.name, 0, 0, 0, 0, 0],
        [p3.name, 0, 0, 0, 0, 0],
        [p4.name, 0, 0, 0, 0, 0],
        [p5.name, 0, 0, 0, 0, 0],
        [p6.name, 0, 0, 0, 0, 0],
        [p7.name, 0, 0, 0, 0, 0],
        [p8.name, 0, 0, 0, 0, 0],
        [p9.name, 0, 0, 0, 0, 0],
        [p10.name, 0, 0, 0, 0, 0],
    ])


    timeLeft = 200

    while timeLeft > 0:
        outcome, player, possessionLength = start_possession([p1, p2, p3, p4, p5], [p6, p7, p8, p9, p10], timeLeft, homeScore, awayScore, possession, "rebound")
        timeLeft -= possessionLength

        if outcome == "turnover":
            # increment the box score
            box_score.loc[box_score["NAME"] == player, 'TO'] += 1
        elif outcome == "2":
            if possession:
                homeScore += 2
            else:
                awayScore += 2
            box_score.loc[box_score["NAME"] == player, 'PTS'] += 2
        elif outcome == "3":
            if possession:
                homeScore += 3
            else:
                awayScore += 3
            box_score.loc[box_score["NAME"] == player, 'PTS'] += 3

        possession = not possession
        print("HOME: ", homeScore, "AWAY: ", awayScore, "TIME: ", timeLeft)
    print(box_score)



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