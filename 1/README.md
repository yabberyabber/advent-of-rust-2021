# Day 1a: Sonar Sweep

Given a stdin that has a sequence of ints (one int per line),
count how many subsequent pairs of lines are increasing.

Example input
```
199
200
208
210
200
207
240
269
260
263
```

Annotated input
```
199 (N/A - no previous measurement)
200 (increased)
208 (increased)
210 (increased)
200 (decreased)
207 (increased)
240 (increased)
269 (increased)
260 (decreased)
263 (increased)
```

Output
```
7
```
