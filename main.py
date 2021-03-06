from cmath import nan
import csv
from doctest import master
import json
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
    res = transition.merge(misc, on=['PLAYER', 'TEAM'], how='outer')
    res = res.merge(iso, on=['PLAYER', 'TEAM'], how='outer')
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


def check():
    master_df = pd.DataFrame(columns=["margin", "pLength", "clock"])
    df = pd.read_csv("21900151.csv")
    prevTimeLeft = 0
    curmargin = 0

    # find out how the possession started
    for index, row in df.iterrows():
        ### check for possession start


        ### check for possession end
        if row["shot_made"] == 0.0: 
            master_df.loc[len(master_df.index)] = [
                curmargin, row["seconds_elapsed"] - prevTimeLeft, row["seconds_elapsed"]]
            prevTimeLeft = row["seconds_elapsed"]

        elif row["shot_made"] == 1.0:
            curmargin = row["scoremargin"]
            master_df.loc[len(master_df.index)] = [
                row["scoremargin"], row["seconds_elapsed"] - prevTimeLeft, row["seconds_elapsed"]]
            prevTimeLeft = row["seconds_elapsed"]
        elif row["is_turnover"]:
            #  print(row["seconds_elapsed"])
            master_df.loc[len(master_df.index)] = [
                row["scoremargin"], row["seconds_elapsed"] - prevTimeLeft, row["seconds_elapsed"]]
            prevTimeLeft = row["seconds_elapsed"]

    master_df.to_csv("possessionLengths.csv")

    print(master_df)

    # print("TO", row["is_turnover"], row["is_steal"], row["shot_made"])

    # possession ends with a shot, turnover, offensive foul



def textParse():
    lines = []
    with open("test.txt") as f:
        contents = f.readlines()
        c = f.read()
        print(len(c))
    

if __name__ == "__main__":
    # check()
    textParse()


a = "144.17873597273316,35.583333333333336,141.7404146053531,37,139.32356764553205,38.416666666666664,136.906720685711,39.833333333333336,134.51134813344893,41.25,132.2877708416588,42.666666666666664,130.06419354986866,44.083333333333336,128.01241151855044,45.5,125.917680672114,46.916666666666664,123.8229498256776,48.333333333333336,121.68527016412298,49.75,119.76331068759286,51.166666666666664,117.84135121106276,52.583333333333336,116.13511191955713,54,114.57236071492311,55.416666666666664,113.00960951028905,56.833333333333336,111.59034639252657,58.25,110.16425050872256,59.666666666666664,108.73815462491855,61.083333333333336,107.30522597507303,62.5,105.78542358555704,63.916666666666664,104.26562119604104,65.33333333333333,102.6589450668546,66.75,101.12938158299367,68.16666666666667,99.59981809913273,69.58333333333333,98.1473672605973,71,96.81497788250533,72.41666666666667,95.48258850441336,73.83333333333333,94.27026058676485,75.25,93.3985948617567,76.66666666666667,92.52692913674855,78.08333333333333,91.99592560438077,79.5,91.8866013477168,80.91666666666667,91.77727709105288,82.33333333333333,92.0896321100928,83.75,92.40393934800166,85.16666666666667,92.7182465859105,86.58333333333333,93.03450604268835,88,93.4200692693157,89.41666666666667,93.80563249594307,90.83333333333333,94.26049949241997,92.25,94.98867713055677,93.66666666666667,95.71685476869358,95.08333333333333,96.71834304849033,96.5,98.11417953982487,97.91666666666667,99.51001603115942,99.33333333333333,101.30020073403182,100.75,103.36760051630209,102.16666666666667,105.43500029857233,103.58333333333333,107.77961516024048,105,110.470748871156,106.41666666666667,113.16188258207156,107.83333333333333,116.19953514223448,109.25,119.10736514760913,110.66666666666667,122.01519515298378,112.08333333333333,124.79320260357014,113.5,127.45505303145082,114.91666666666667,130.1169034593315,116.33333333333333,132.6625968645065,117.75,135.187791971557,119.16666666666667,137.71298707860754,120.58333333333333,140.21768388753355,122,142.67650355303812,123.41666666666667,145.13532321854268,124.83333333333333,147.54826574062577,126.25,149.7708669229814,127.66666666666667,151.99346810533706,129.08333333333334,154.02572794796527,130.5,155.89204918672854,131.91666666666666,157.7583704254918,133.33333333333334,159.45875306039014,134.75,161.0576203141005,136.16666666666666,162.6564875678109,137.58333333333334,164.15383944033331,139,165.47841994294927,140.41666666666666,166.8030004455653,141.83333333333334,167.95480957827485,143.25,169.0412193788729,144.66666666666666,170.12762917947097,146.08333333333334,171.14863964795754,147.5,172.08765692394616,148.91666666666666,173.02667419993477,150.33333333333334,173.88369828342545,151.75,174.66360972159063,153.16666666666666,175.44352115975582,154.58333333333334,176.14631995259552,156,176.68708457930833,157.41666666666666,177.22784920602112,158.83333333333334,177.60657966660696,160.25,177.9560268441578,161.66666666666666,178.30547402170865,163.08333333333334,178.62563791622452,164.5,178.85697585220097,165.91666666666666,179.08831378817737,167.33333333333334,179.23082576561433,168.75,179.34405446001628,170.16666666666666,179.4572831544182,171.58333333333334,179.5412285657852,173,179.61346066393813,174.41666666666666,179.68569276209112,175.83333333333334,179.74621154703013,177.25,179.8038020036656,178.66666666666666,179.86139246030106,180.08333333333334,179.91605458863305,181.5,179.94826619997153,182.91666666666666,179.98047781131,184.33333333333334,179.990238905655,185.75,179.99511945282748,187.16666666666666,180,188.58333333333334,180,189.29166666666666,180,190,180"

arr = a.split(",")


x = []
y = []
i = 0
while i < len(arr) - 1:
    x.append(arr[i])
    y.append(arr[i+1])
    i += 2

# print(y)



def make_json(csvFilePath, jsonFilePath):
     
    # create a dictionary
    data = {}
     
    # Open a csv reader called DictReader
    with open(csvFilePath, encoding='utf-8') as csvf:
        csvReader = csv.DictReader(csvf)
         
        # Convert each row into a dictionary
        # and add it to data
        i = 0
        for rows in csvReader:
             
            # Assuming a column named 'No' to
            # be the primary key

            rows["FG3A/100"] = float(rows["FG3A/100"])
            rows["FGA/100"] = float(rows["FGA/100"])
            rows["Usg%"] = float(rows["Usg%"][0:4])

            rows["FG2%"] = float(rows["FG2%"][0:4])
            rows["FG3%"] = float(rows["FG3%"][0:4])
            # print(rows["Usg%"])
            key = i
            data[key] = rows
            i+=1
 
    # Open a json writer, and use the json.dumps()
    # function to dump data
    with open(jsonFilePath, 'w', encoding='utf-8') as jsonf:
        jsonf.write(json.dumps(data, indent=4))

make_json("darko.csv", "darko.json")



# print(pd.read_json("darko.json"))
