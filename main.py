import pandas as pd

def run():
    darko_data = pd.read_csv("data/darko.csv")


    transition = pd.read_csv("data/transition.csv")
    misc = pd.read_csv("data/misc.csv")
    iso = pd.read_csv("data/iso.csv")
    pick_and_roll_handler = pd.read_csv("data/pick_and_roll_handler.csv")
    pick_and_roll_roller = pd.read_csv("data/pick_and_roll_roller.csv")
    spotup = pd.read_csv("data/spotup.csv")
    putback = pd.read_csv("data/putbacks.csv")
    postup = pd.read_csv("data/postup.csv")
    daily = pd.read_csv("data/daily.csv")

    # res = darko_data.merge(transition,on=['PLAYER', 'Team'],how='outer')
    res = transition.merge(misc,on=['PLAYER', 'TEAM'],how='outer')
    res = res.merge(iso,on=['PLAYER', 'TEAM'],how='outer')
    res = res.merge(pick_and_roll_handler, on=['PLAYER', 'TEAM'], how="outer")
    res = res.merge(pick_and_roll_roller, on=['PLAYER', 'TEAM'], how="outer")
    res = res.merge(spotup, on=['PLAYER', 'TEAM'], how="outer")
    res = res.merge(putback, on=['PLAYER', 'TEAM'], how="outer")
    res = res.merge(postup, on=['PLAYER', 'TEAM'], how="outer")
    

    res = daily.merge(res, on='PLAYER')
    res = darko_data.merge(res, on='PLAYER')

    res.to_csv("master.csv")

    # darko_data.to_csv("master.csv")

    # transition.to_csv("t.csv")






if __name__ == "__main__":
    run()
