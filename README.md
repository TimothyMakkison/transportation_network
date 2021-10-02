# transportation_network

This was the final programming assignnment for an Advanced Programming lab. Found [here.](https://github.com/k4io/lab-i-copy)

# Summary
The objective of this final lab is to implement, exercise and assess the performance of data structures that represent a transport network and support route-finding and evaluation. A network is a collection of nodes and arcs so, in the context of a transport network the nodes will correspond to road/rail junctions; towns/cities/villages; bus/rail stations; air/sea ports, etc., while the network arcs will be road/rail route segments or air corridors/sea lanes as appropriate.

Data will be supplied (in formats as described below) for the nodes and arcs representing a transport network, together with a ‘command’ file specifying operations to be performed on the network data. These operations may include, for example, reporting the distance between two nodes (places) on the network; finding neighbour nodes to a starting point; finding or validating a node sequence to form a journey between two places (pairs of nodes for origin and destination). For each command type, a specific output format will be defined. To streamline the assessment process, your software must implement all input and output formats precisely.

You will construct software in C++ with suitable class definitions for the necessary data structures, to input the supplied data and work through the commands. You will also be expected to present diagnostic data on process efficiency, for example the number of node/arc records visited in the data structure.


## Data
3 files were provided; [Places.csv](https://github.com/TimothyMakkison/transportation_network/blob/master/Places.csv) (nodes) containing the Name, Id and Coordinates, [Links.csv](https://github.com/TimothyMakkison/transportation_network/blob/master/Links.csv) (edges)
containing a pair of place Id's and the type of transportation. [Commands.txt](https://github.com/TimothyMakkison/transportation_network/blob/master/Commands.txt) contains commands that must be read and performed e.g. find the shortest path between two places.

### Places.csv
|Name|Id|Latitude|Longitude|
| :---:   | :-: | :-: | :-: |
|Whitley Bridge|	8441694|	53.694	|-1.156|
|Doncaster Rail|	8611522	|53.522|	-1.139|
|Doncaster Ctr|	8631524	|53.524|	-1.137|


### Links.csv
|Place a|Place b| Transport Mode|
| :-:   | :-: | :-: | 
|8441694|	061761	|Bus|
|8441694	|10381699	|Car|
|8611522	|8631524	|Car|
|8611522|	11251704	|Rail|

### Modes
Commands specify what type of travel mode is used, this determines what links are allowed to be traversed as follows.
ie: Foot can use any link, whereas rail and ship may only go on their respect lines.
|     |Foot Route|Bike Route|Car Route|Bus Route|Ship Route|Rail Route|
| :-:  | :-:  | :-:  | :-:  | :-:  | :-:  | :-:  |
|Foot Mode| X  | X | X | X | X | X |
|Bike Mode|  | X | X | X | X | X |
|Car Mode|   |  | X | X | X |  |
|Bus Mode|  |  |  | X | X |  |
|Rail Mode|  |  |  |   | X |  |
|Ship Mode|  |  |  |   |  | X |

#### Commands
### Example Comamnds.txt
```
MaxLink
FindDist 9361783 11391765
FindNeighbour 8611522
Check Ship 14601225 12321385 8611522 9361783
FindRoute Foot 9081958 51889340
```

### List Of Commands
#### MaxDist 
Finds the two furthest separated places (calculated using longitude & latitude). 
```
MaxDist
```
Returns:
```
<Name of place 1>,<Name of place 2>,<Distnace in kilometres>
York Rail,Rotterdam Harbour,416.543
```

#### MaxLink 
Finds the two most separated connected places (calculated using longitude & latitude).
'''
MaxLink
'''
Returns:
```
<Id of place 1>,<Id of place 2>,<Distance in kilometres>
```

#### FindDist
Finds the distance between two places.
``` 
FindDist <Id of place 1> <Id of place 2>
ie:
FindDist 9361783 11391765
```
Returns:
```
<Name of place 1>,<Name of place 2>,<Distance in kilometres> 

#### FindNeighbour
Lists the neighbours of a given place.
'''
FindNeighbour <Id of place>
ie:
FindNeighbour 8611522
```
Returns
```
<Id of neighbour 1>
<Id of neighbour 2>
<....>
```

#### Check
Check verifies that a specific route is possible by checking the travel mode and whether the route is viable.
```
Check <Travel mode> <Id of place 1> <Id of place 2> <Id of place 3> <...>
ie: 
Check Rail 14601225 12321385 8611522 9361783
```

Example return for a valid route:
```
Check Rail 14601225 12321385 8611522 9361783
14601225,12321385,PASS
12321385,8611522,PASS
8611522,9361783,PASS
```

Example return for an invalid route:
```
Check Ship 14601225 12321385 8611522 9361783
14601225,12321385,FAIL
```

#### FindRoute
Finds a valid route between two points given a travel mode. This route does not have to be the fastest, instead should it should calculate a valid route in the shortest amount of time possible.
```
FindRoute <Travel mode> <Id of place 1> <Id of place 2>
eg:
FindRoute Rail 9081958 15832241
```

Example return for a valid route
```
FindRoute Rail 9081958 15832241
9081958
12032132
15832241
```

Example return of invalid route
```
FindRoute Ship 9081958 15832241
FAIL
```

#### FindFastest Route
Command FindShortestRoute will find the shortest journey sequence of Nodes between first (start) and destination (second) places by the stated Mode. The shortest route is one defined as requiring the least number of nodes and not distance.

This command is similar to the command FindRoute but it is required to find only the shortest route in terms of number of nodes travelled.

It will output the references of a route from the starting Node to the end Node by the stated Mode (e.g. Rail, Car, etc.), followed by a blank line. If there is no valid route then output FAIL.

Input form:
```
FindShortestRoute <Travel mode> <Id of place 1> <Id of place 2>

eg:
FindShortestRoute Rail 9081958 15832241
```

Example return for valid route:
```
FindShortestRoute Rail 9081958 15832241
9081958
12032132
15832241
```

Example return for an invalid route:
```
FindShortestRoute Ship 9081958 15832241
FAIL
```
  



