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
else:
    print("ERROR: INVALID INPUT") 

print("Total current: " + str(totalCurrent) + " Amps")
