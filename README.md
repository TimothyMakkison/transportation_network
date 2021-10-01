# transportation_network

This was the final programming assignnment for an Advanced Programming lab. Found [here.](https://github.com/k4io/lab-i-copy)

Given a collection of places and transportation links, calculate various values such as distance, the construction and vaidity of routes in a time efficient manner. 
Additional complexity is 

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
| :---:   | :-: | :-: | 
|8441694|	061761	|Bus|
|8441694	|10381699	|Car|
|8611522	|8631524	|Car|
|8611522|	11251704	|Rail|

### Comamnds.txt
```
MaxLink
FindDist 9361783 11391765
FindNeighbour 8611522
Check Ship 14601225 12321385 8611522 9361783
FindRoute Foot 9081958 51889340
```


