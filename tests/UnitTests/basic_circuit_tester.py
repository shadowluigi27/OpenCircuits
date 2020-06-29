# basic program demonstrating ohms law

# check to see if circuit is series or parallel
circuitType = input("Series or Parallel: ")

voltage_source = int(input("Enter source voltage value: ")) # store voltage source value

num_resistors = int(input("Enter number of resistors: ")) # ask for number of resistors
# num_capacitors = int(input("Enter number of capacitors: "))
# num_inductors = int(input("Enter number of inductors: "))

countR = num_resistors 
resistorValues = [] # list to store resistor values

# loop through number of resistors and ask for value of each  
while(countR != 0):
    resistorValue = int(input("Enter resistor value: "))
    resistorValues.append(resistorValue)
    countR -= 1

# series resistance and current calculation
if (circuitType == "series" or circuitType == "Series"):
    total_resistance_series = sum(resistorValues)
    print("Total Series Resistance: " + str(total_resistance_series) + " Ohms")
    totalCurrent = voltage_source / total_resistance_series # Ohm's Law to find total current
    print("Total current: " + str(totalCurrent) + " Amps")

    # Find voltage at certain resistor (values work for series only for now) 

    # specify which resistor you want to find voltage for
    specified_res = int(input("Enter resistor number: "))
    if (specified_res > num_resistors or specified_res < 1):
        print("ERROR: Resistor does not exist") 
        raise SystemExit

    specified_res_val = resistorValues[specified_res - 1]

    specified_res_volt = totalCurrent * specified_res_val

    print("Total voltage at resistor " + str(specified_res) + ": " + str(specified_res_volt) + "Volts")

# parallel resistance and current calculation
elif (circuitType == "Parallel" or circuitType == "parallel"):
    count = len(resistorValues)
    i = 0
    # change values of list to 1 / value 
    while (count != 0):
        resistorValues[i] = 1 / resistorValues[i]
        i += 1
        count -= 1
        
    total_resistance_parallel = 1 / sum(resistorValues)
    print("Total Parallel Resistance: " + str(total_resistance_parallel) + " Ohms")
    totalCurrent = voltage_source / total_resistance_parallel 
    print("Total current: " + str(totalCurrent) + " Amps")
else:
    print("ERROR: INVALID INPUT") 

    



